use crate::board::board::{Board, Ply};
use std::time::Duration;
use crate::ai::minmax::{min_max, MinMaxResponse};

pub trait Player {
    fn generate_ply(&self, board: &Board, time_left: Duration) -> Ply;
}

#[derive(Clone)]
pub struct MinMaxPlayer {

}

impl Player for MinMaxPlayer {
    fn generate_ply(&self, board: &Board, time_left: Duration) -> Ply{

        let mut alpha = MinMaxResponse::MIN;
        let mut beta = MinMaxResponse::MAX;
        let response = min_max(board.clone(), 7, &mut alpha, &mut beta);
        println!("Eval: {}", response.eval);
        let ply = response.ply.expect("invalid move");

        ply

    }
}
