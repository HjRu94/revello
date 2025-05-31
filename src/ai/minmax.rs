use crate::board::board::{Board, Ply, possible_plys, play, Player};

pub struct MinMaxResponse {
    pub eval: i32,
    pub ply: Option<Ply>
}

const plus_inf: MinMaxResponse = MinMaxResponse {
    eval: i32::MAX,
    ply: unsafe {Some(Ply::new_unchecked(0))}
};

const minus_inf: MinMaxResponse = MinMaxResponse {
    eval: i32::MIN,
    ply: unsafe {Some(Ply::new_unchecked(0))}
};

fn static_eval(board: Board) -> MinMaxResponse {
    let black_pieces: i32 = board.black.count_ones().try_into().unwrap();
    let white_pieces: i32 = board.white.count_ones().try_into().unwrap();

    let eval: i32 = black_pieces - white_pieces;

    let ret = MinMaxResponse {
        eval: eval,
        ply: None
    };

    return ret;
}

pub fn min_max(board: Board, depth: u32) -> MinMaxResponse {
    if depth == 0 {
        return static_eval(board);
    }
    let plys = possible_plys(&board);
    let vec_ply = plys.to_vec_ply();
    if board.turn == Player::Black {
        let mut val = minus_inf;
        for ply in vec_ply {
            let new_board = play(&board, ply.clone());
            let min_max_val = min_max(new_board, depth - 1);

            if min_max_val.eval >= val.eval {
                val.eval = min_max_val.eval;
                val.ply = Some(ply);
            }
        }
        return val;
    }
    else {
        let mut val = plus_inf;
        for ply in vec_ply {
            let new_board = play(&board, ply.clone());
            let min_max_val = min_max(new_board, depth - 1);

            if min_max_val.eval <= val.eval {
                val.eval = min_max_val.eval;
                val.ply = Some(ply);
            }
        }
        return val;
    }

}

