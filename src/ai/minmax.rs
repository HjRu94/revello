use crate::board::board::{Board, Ply, possible_plys, play, Player};
use std::cmp::{Ordering, max, min};
use crate::ai::static_evaluation::{static_eval};


#[derive(Eq, Copy, Clone)]
pub struct MinMaxResponse {
    pub eval: i32,
    pub ply: Option<Ply>
}

impl MinMaxResponse {
    pub const MAX: MinMaxResponse =
        MinMaxResponse {
            eval: i32::MAX,
            ply: unsafe {Some(Ply::new_unchecked(0))}
        };

    pub const MIN: MinMaxResponse =
        MinMaxResponse {
            eval: i32::MIN,
            ply: unsafe {Some(Ply::new_unchecked(0))}
        };
}

impl PartialEq for MinMaxResponse {
    fn eq(&self, other: &Self) -> bool {
        self.eval == other.eval
    }
}

impl PartialOrd for MinMaxResponse {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MinMaxResponse {
    fn cmp(&self, other: &Self) -> Ordering {
        self.eval.cmp(&other.eval)
    }
}

pub fn min_max(board: Board, depth: u32, alpha: &mut MinMaxResponse, beta: &mut MinMaxResponse) -> MinMaxResponse {
    if depth == 0 {
        return static_eval(board);
    }
    let plys = possible_plys(&board);
    let vec_ply = plys.to_vec_ply();
    if board.turn == Some(Player::Black) {
        let mut best_val = MinMaxResponse::MIN;
        for ply in vec_ply {
            let new_board = play(&board, ply.clone());
            let min_max_val = min_max(new_board, depth - 1, &mut alpha.clone(), &mut beta.clone());
            if min_max_val.eval >= best_val.eval {
                best_val.eval = min_max_val.eval;
                best_val.ply = Some(ply);
            }

            *alpha = max(best_val, *alpha);

            if beta <= alpha {
                break;
            } 
        }
        return best_val;
    }
    else {
        let mut best_val = MinMaxResponse::MAX;
        for ply in vec_ply {
            let new_board = play(&board, ply.clone());
            let min_max_val = min_max(new_board, depth - 1, &mut alpha.clone(), &mut beta.clone());
            if min_max_val.eval <= best_val.eval {
                best_val.eval = min_max_val.eval;
                best_val.ply = Some(ply);
            }

            *beta = min(*beta, best_val);

            if beta <= alpha {
                break;
            }
        }
        return best_val;
    }

}

