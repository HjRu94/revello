use crate::board::board::{Board, Ply};
use std::time::Duration;
use crate::ai::minmax::{min_max, MinMaxResponse, MinMaxEval};

pub trait Player {
    fn generate_ply(&self, board: &Board, time_left: Duration) -> Ply;
}

#[derive(Clone)]
pub struct MinMaxPlayer {

}

impl Player for MinMaxPlayer {
    fn generate_ply(&self, board: &Board, time_left: Duration) -> Ply{

        let alpha = MinMaxEval::MIN;
        let beta = MinMaxEval::MAX;
        let response = min_max(board.clone(), 7, &alpha, &beta);
        println!("Eval: {}", response.eval.value);
        let ply = response.ply.expect("invalid move");

        ply

    }
}
