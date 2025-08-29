use crate::board::board::{Board, Ply};
use std::time::Duration;
use crate::ai::minmax::{min_max, MinMaxResponse, MinMaxEval};
use crate::ai::transposition_table::{TranspositionTable, move_ordering};

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
        let mut transposition_table = TranspositionTable::new();
        let mut response = MinMaxResponse::ZERO;
        use std::time::Instant;

        let start = Instant::now(); // start timer

        for depth in 1..7 {
            response = min_max(board.clone(), depth as u32, &alpha, &beta, &mut transposition_table);
        }

        let duration = start.elapsed(); // time elapsed
        println!("Eval: {}", response.eval.value);
        println!("Time elapsed: {:?}", duration);

        let ply = response.ply.expect("invalid move");

        ply

    }
}
