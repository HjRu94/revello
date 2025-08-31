use crate::ai::minmax::{MinMaxResponse, MinMaxEval};
use crate::board::board::{Board, possible_plys, Player};

pub fn static_eval(board: &Board) -> MinMaxResponse {

    let black_pieces: i32 = board.black.count_ones().try_into().unwrap();
    let white_pieces: i32 = board.white.count_ones().try_into().unwrap();

    if board.turn == None {
        if black_pieces - white_pieces > 0 {
            return MinMaxResponse::new_empty_ply(MinMaxEval::new(i32::MAX - 64 + black_pieces - white_pieces));
        }
        else if white_pieces - black_pieces > 0 {
            return MinMaxResponse::new_empty_ply(MinMaxEval::new(i32::MIN + 64 + white_pieces - black_pieces));
        }
        else {
            return MinMaxResponse::ZERO;
        }
    }

    let black: u64 = board.black.into();
    let white: u64 = board.white.into();

    let t: u64 = 0xFFFFFFFFFFFFFF00;
    let r: u64 = 0x7F7F7F7F7F7F7F7F;
    let l: u64 = 0xFEFEFEFEFEFEFEFE;
    let b: u64 = 0x00FFFFFFFFFFFFFF;

    let tr: u64 = t & r;
    let tl: u64 = t & l;
    let br: u64 = b & r;
    let bl: u64 = b & l;

    //detect safe squares

    let mut bs_nw = black & (black << 1 | !l) & (black << 8 | !t) & (black << 9 | !tl) & (black << 7 | black >> 7 | !tl);
    let mut ws_nw = white & (white << 1 | !l) & (white << 8 | !t) & (white << 9 | !tl) & (white << 7 | white >> 7 | !tl);

    let mut bs_ne = black & (black >> 1 | !r) & (black << 8 | !t) & (black << 7 | !tr) & (black << 9 | black >> 9 | !tr);
    let mut ws_ne = white & (white >> 1 | !r) & (white << 8 | !t) & (white << 7 | !tr) & (white << 9 | white >> 9 | !tr);

    let mut bs_se = black & (black >> 1 | !r) & (black >> 8 | !b) & (black >> 9 | !br) & (black << 7 | black >> 7 | !br);
    let mut ws_se = white & (white >> 1 | !r) & (white >> 8 | !b) & (white >> 9 | !br) & (white << 7 | white >> 7 | !br);

    let mut bs_sw = black & (black << 1 | !l) & (black >> 8 | !b) & (black >> 7 | !bl) & (black << 9 | black >> 9 | !bl);
    let mut ws_sw = white & (white << 1 | !l) & (white >> 8 | !b) & (white >> 7 | !bl) & (white << 9 | white >> 9 | !bl);

    for _ in 1..5 {
        bs_nw = black & (bs_nw << 1 | !l) & (bs_nw << 8 | !t) & (bs_nw << 9 | !tl) & (bs_nw << 7 | bs_nw >> 7 | !tl);
        ws_nw = white & (ws_nw << 1 | !l) & (ws_nw << 8 | !t) & (ws_nw << 9 | !tl) & (ws_nw << 7 | ws_nw >> 7 | !tl);

        bs_ne = black & (bs_ne >> 1 | !r) & (bs_ne << 8 | !t) & (bs_ne << 7 | !tr) & (bs_ne << 9 | bs_ne >> 9 | !tr);
        ws_ne = white & (ws_ne >> 1 | !r) & (ws_ne << 8 | !t) & (ws_ne << 7 | !tr) & (ws_ne << 9 | ws_ne >> 9 | !tr);

        bs_se = black & (bs_se >> 1 | !r) & (bs_se >> 8 | !b) & (bs_se >> 9 | !br) & (bs_se << 7 | bs_se >> 7 | !br);
        ws_se = white & (ws_se >> 1 | !r) & (ws_se >> 8 | !b) & (ws_se >> 9 | !br) & (ws_se << 7 | ws_se >> 7 | !br);

        bs_sw = black & (bs_sw << 1 | !l) & (bs_sw >> 8 | !b) & (bs_sw >> 7 | !bl) & (bs_sw << 9 | bs_sw >> 9 | !bl);
        ws_sw = white & (ws_sw << 1 | !l) & (ws_sw >> 8 | !b) & (ws_sw >> 7 | !bl) & (ws_sw << 9 | ws_sw >> 9 | !bl);
    }

    let black_safe = bs_nw | bs_ne | bs_se | bs_sw;
    let white_safe = ws_nw | ws_ne | ws_se | ws_sw;

    let n_black_safe: i32 = black_safe.count_ones().try_into().unwrap();
    let n_white_safe: i32 = white_safe.count_ones().try_into().unwrap();

    let n_black_x: i32 = (
        ((!black & 1 << 00) << 9 & black) |
        ((!black & 1 << 07) << 7 & black) |
        ((!black & 1 << 56) >> 7 & black) |
        ((!black & 1 << 63) >> 9 & black)
    ).count_ones().try_into().unwrap();

    let n_white_x: i32 = (
        ((!white & 1 << 00) << 9 & white) |
        ((!white & 1 << 07) << 7 & white) |
        ((!white & 1 << 56) >> 7 & white) |
        ((!white & 1 << 63) >> 9 & white)
    ).count_ones().try_into().unwrap();

    let player_plys: u64 = possible_plys(&board).into();

    let mut flip_board = board.clone();
    flip_board.flip_turn();

    let opponent_plys: u64 = possible_plys(&flip_board).into();

    let n_black_plys: i32 = if board.turn == Some(Player::Black) {player_plys.count_ones().try_into().unwrap()} else {opponent_plys.count_ones().try_into().unwrap()};
    let n_white_plys: i32 = if board.turn == Some(Player::White) {player_plys.count_ones().try_into().unwrap()} else {opponent_plys.count_ones().try_into().unwrap()};

    let eval: i32 = n_black_plys - n_white_plys + 10 * (n_black_safe - n_white_safe) - 10 * (n_black_x - n_white_x);

    let ret = MinMaxResponse {
        eval: MinMaxEval {value: eval},
        ply: None
    };

    return ret;
}
