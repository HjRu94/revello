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
        let allowed_thinking_time = Duration::from_secs(1);
        let mut depth = 1;

        while true {
            if let Some(res) = min_max(board.clone(), depth as u32, &alpha, &beta, &mut transposition_table, start, allowed_thinking_time) {
                response = res;
            }
            else {
                break;
            }
            depth += 1;
            if depth > 60 {
                break;
            }
        }

        let duration = start.elapsed(); // time elapsed
        println!("Eval: {}", response.eval.value);
        println!("Depth: {}", depth);
        println!("Time elapsed: {:?}", duration);

        let ply = response.ply.expect("invalid move");

        ply

    }
}
