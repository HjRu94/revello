use crate::board::board::{Board, Ply, possible_plys, play, Player};
use std::cmp::{Ordering, max, min};
use crate::ai::static_evaluation::{static_eval};


#[derive(Copy, Clone)]
pub struct MinMaxResponse {
    pub eval: MinMaxEval,
    pub ply: Option<Ply>
}

#[derive(Copy, Clone)]
pub struct MinMaxEval {
    pub value: i32
}

impl MinMaxEval {
    pub const MAX: MinMaxEval =
        MinMaxEval {
            value: i32::MAX,
        };

    pub const MIN: MinMaxEval =
        MinMaxEval {
            value: i32::MIN,
        };

    pub const ZERO: MinMaxEval =
        MinMaxEval {
            value: 0,
        };
}

impl MinMaxResponse {
    pub const MAX: MinMaxResponse =
        MinMaxResponse {
            eval: MinMaxEval::MAX,
            ply: unsafe {Some(Ply::new_unchecked(0))}
        };

    pub const MIN: MinMaxResponse =
        MinMaxResponse {
            eval: MinMaxEval::MIN,
            ply: unsafe {Some(Ply::new_unchecked(0))}
        };

    pub const ZERO: MinMaxResponse =
        MinMaxResponse {
            eval: MinMaxEval::ZERO,
            ply: unsafe {Some(Ply::new_unchecked(0))}
        };
}

impl PartialEq for MinMaxEval {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for MinMaxEval {}

impl PartialOrd for MinMaxEval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MinMaxEval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

pub fn min_max(board: Board, depth: u32, alpha: &MinMaxEval, beta: &MinMaxEval) -> MinMaxResponse {
    if depth == 0 {
        return static_eval(board);
    }
    let plys = possible_plys(&board);

    if plys.is_zero() {
        return static_eval(board);
    }

    let mut alpha = alpha.clone();
    let mut beta = beta.clone();

    // Maximizing player
    if board.turn == Some(Player::Black) {
        let mut best_move = MinMaxResponse::MIN;
        for ply in plys {
            let new_board = play(&board, ply.clone());
            let min_max_val = min_max(new_board, depth - 1, &alpha, &beta);

            if min_max_val.eval >= best_move.eval {
                best_move.eval = min_max_val.eval;
                best_move.ply = Some(ply);
            }

            alpha = max(best_move.eval, alpha);

            if beta < alpha {
                break;
            }
        }
        return best_move;
    }
    // Minimizing player
    else {
        let mut best_move = MinMaxResponse::MAX;
        for ply in plys {
            let new_board = play(&board, ply.clone());
            let min_max_val = min_max(new_board, depth - 1, &alpha, &beta);

            if min_max_val.eval <= best_move.eval {
                best_move.eval = min_max_val.eval;
                best_move.ply = Some(ply);
            }

            beta = min(beta, best_move.eval);

            if beta < alpha {
                break;
            }
        }
        return best_move;
    }
}

