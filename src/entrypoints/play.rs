use crate::graphics::graphics::{draw_board};
use crate::board::board::{Board, START_BOARD,Player};
pub async fn human_vs_ai() {
    let mut board = Board::new(
        0x0000001008000000,
        0x0000000810000000,
        Player::Black).expect("white and black are overlapping");

    loop {
        draw_board(&board).await;
    }
}
