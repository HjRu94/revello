// Board struct using bitboards to represent the othello board

mod board;
mod ai;

fn main(){
    use revello::board::board::{possible_plys, play, Ply, START_BOARD};
    use revello::ai::player::{MinMaxPlayer, Player};
    use std::time::Duration;

    let startpos = START_BOARD;

    let plys = possible_plys(&startpos);
    let ply = Ply::new(1 << 26).expect("Invalid ply");

    println!("{}", startpos);

    println!("{}", plys);
    println!("");
    println!("{}", ply);
    println!("");
    let new_board = play(&startpos, ply);

    println!("{}", new_board);

    let player = MinMaxPlayer{};

    let ply2 = player.generate_ply(new_board, Duration::from_secs(10));
    print!("{}", ply2);

}
