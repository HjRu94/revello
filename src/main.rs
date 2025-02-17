// Board struct using bitboards to represent the othello board

mod board;

fn main(){
    use board::{Board, possible_plys, play, printu64};
    let startpos = Board {
        white: 0x0000001008000000,
        black: 0x0000000810000000,
        turn: true
    };

    let plys = possible_plys(&startpos);
    let ply = 1 << 26;

    println!("{}", startpos);

    printu64(plys);
    println!("");
    printu64(ply);
    println!("");
    let new_board = play(&startpos, ply);

    println!("{}", new_board);
}
