use crate::board::board::{Board, Ply, possible_plys};
use std::time::Duration;
use crate::ai::minmax::{min_max, MinMaxResponse};
use crate::graphics::graphics::{detect_ply, draw_playable};

pub trait Player {
    fn generate_ply(&self, board: &Board, time_left: Duration) -> Ply;
}

#[derive(Clone)]
pub struct MinMaxPlayer {

}

impl Player for MinMaxPlayer {
    fn generate_ply(&self, board: &Board, time_left: Duration) -> Ply{

        let response = min_max(board.clone(), 6);
        let ply = response.ply.expect("invalid move");

        ply

    }
}

#[derive(Clone)]
pub struct HumanPlayer {

}

impl Player for HumanPlayer {

    fn generate_ply(&self, board: &Board, time_left: Duration) -> Ply {
        loop {
            draw_playable(&board);
            let ply = detect_ply();
            if ply == None {
            }
            else {
                let ply = ply.expect("ply is None");
                for p in possible_plys(&board) {
                    if ply == p {
                        return ply;
                    }
                }
            }
        }
    }
}
