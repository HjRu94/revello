use crate::graphics::graphics::{draw_board, detect_ply};
use crate::board::board::{Board, START_BOARD, Player, play};

pub async fn ai_vs_ai() {

}

pub async fn human_vs_ai() {

}

pub async fn human_vs_human() {
    let mut board = START_BOARD.clone();
    loop {
        let ply = detect_ply();
        if ply != None {
            board = play(&board, ply.expect("ply is None"));
        }
        draw_board(&board).await;
    }
}
