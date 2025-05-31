use crate::board::board::{Board, Ply};
use std::time::Duration;
use crate::ai::minmax::{min_max, MinMaxResponse};

pub trait Player {
    fn generate_ply(&self, board: Board, time_left: Duration) -> Ply;
}

pub struct MinMaxPlayer {

}

impl Player for MinMaxPlayer {
    fn generate_ply(&self, board: Board, time_left: Duration) -> Ply{

        let response = min_max(board, 8);
        let ply = response.ply.expect("invalid move");

        ply

    }
}
