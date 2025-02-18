// Board struct using bitboards to represent the othello board

mod board;

fn main(){
    use board::{Board, possible_plys, play, Ply};
    let startpos = Board::new();

    let plys = possible_plys(&startpos);
    let ply = Ply{ ply: 1 << 26 };

    println!("{}", startpos);

    println!("{}", plys);
    println!("");
    println!("{}", ply);
    println!("");
    let new_board = play(&startpos, ply);

    println!("{}", new_board);
}
