use crate::board::board::{Board, Ply, possible_plys, Player as BoardPlayer};
use std::time::Duration;
use crate::ai::minmax::{min_max, MinMaxResponse, MinMaxEval};
use crate::ai::transposition_table::{TranspositionTable};
use crate::graphics::graphics::{draw_playable, detect_ply};
use std::sync::{Arc, Mutex};
use std::thread;

pub trait Player {
    fn update(&mut self, _board: &Board) {

    }
    fn generate_ply(&self, board: &Board, time_left: Duration) -> Ply;
}

#[derive(Clone)]
pub struct MinMaxPlayer {

}

#[derive(Clone)]
pub struct HumanPlayer {
    selected_ply: Arc<Mutex<Option<Ply>>>,
    player: BoardPlayer
}

impl MinMaxPlayer {
    pub fn new() -> Self {
        return MinMaxPlayer{};
    }
}

impl Player for MinMaxPlayer {
    fn generate_ply(&self, board: &Board, time_left: Duration) -> Ply{

        let alpha = MinMaxEval::MIN;
        let beta = MinMaxEval::MAX;
        let mut transposition_table = TranspositionTable::new();
        let mut response = MinMaxResponse::ZERO;
        use std::time::Instant;

        let start = Instant::now(); // start timer
        let factor = 2.0 / (64.0 - board.count_pieces() as f32);
        let allowed_thinking_time = Duration::from_secs_f32(time_left.as_secs_f32() * factor);
        let mut depth = 1;

        loop {
            if let Some(res) = min_max(board.clone(), depth as u32, &alpha, &beta, &mut transposition_table, start, allowed_thinking_time) {
                response = res;
            }
            else {
                break;
            }

            if depth >= 60 {
                break;
            }

            depth += 1;
        }

        let duration = start.elapsed(); // time elapsed
        println!("Eval: {}", response.eval.value);
        println!("Depth: {}", depth);
        println!("Time elapsed: {:?}", duration);

        let ply = response.ply.expect("invalid move");

        ply

    }
}

impl HumanPlayer {
    pub fn new(player: BoardPlayer) -> Self {
        HumanPlayer {
            selected_ply: Arc::new(Mutex::new(None)),
            player: player,
        }
    }
}

impl Player for HumanPlayer {
    fn update(&mut self, board: &Board) {
        if let Some(turn) = board.turn {
            if turn == self.player {
                draw_playable(&board);
                let mut sel = self.selected_ply.lock().unwrap();
                *sel = detect_ply();
            }
        }
    }

    fn generate_ply(&self, board: &Board, _time_left: Duration) -> Ply {
        loop {
            if let Some(ply) = *self.selected_ply.lock().unwrap() {
                if ply.is_in(possible_plys(board)) {
                    return ply;
                }
            }
            thread::sleep(Duration::from_millis(16)); // ~60 checks/sec
        }
    }
}
