use crate::board::board::{Board, Ply, Plys, play, Player, possible_plys};
use crate::ai::minmax::{MinMaxResponse, MinMaxEval};
use std::collections::HashMap;

#[derive(Hash, PartialEq, Copy, Clone)]
pub struct TranspositionEntry {
    minmax_response: MinMaxResponse,
    depth: u32,
}

impl TranspositionEntry {
    pub fn get_depth(&self) -> u32 {
        self.depth
    }
    pub fn get_minmax_response(&self) -> MinMaxResponse {
        self.minmax_response
    }
}

impl TranspositionEntry {
    pub fn new(minmax_response: MinMaxResponse, depth: u32) -> Self{
        return TranspositionEntry{
            minmax_response: minmax_response,
            depth: depth
        }
    }
}

pub struct TranspositionTable {
    table: HashMap<Board, TranspositionEntry>
}

impl TranspositionTable {
    pub fn new() -> Self {
        return TranspositionTable {
            table: HashMap::new()
        }
    }

    pub fn get(&self, board: &Board) -> Option<TranspositionEntry> {
        self.table.get(board).copied()
    }

    pub fn insert(&mut self, board: Board, entry: TranspositionEntry) {
        self.table.insert(board, entry);
    }
}

static mut counter1: i32 = 0;
static mut counter2: i32 = 0;

pub fn move_ordering(
    board: &Board,
    transposition_table: &TranspositionTable,
    depth: u32,
) -> Vec<Ply> {
    let plys = possible_plys(board);
    if depth == 1 {
        return plys.into_iter().collect();
    }
    if plys.is_zero() {
        return vec![];
    }
    let mut scored_moves: Vec<(Ply, MinMaxEval)> = plys.into_iter().map(|ply| {
        let new_board = play(board, ply.clone());
        let val = transposition_table.get(&new_board);
        if let Some(transposition_entry) = transposition_table.get(&new_board) {
            let eval = transposition_entry.get_minmax_response().eval;
            unsafe {
                counter1 +=1;
            }
            return (ply, eval);
        }
        let mut eval = match board.turn {
            Some(Player::White) => MinMaxEval::MAX,
            Some(Player::Black) => MinMaxEval::MIN,
            None => MinMaxEval::ZERO,
        };
        unsafe {
            counter2 += 1;
        }
        (ply, eval)
    }).collect();

    unsafe {
        //println!("Have Done stuff: {}", counter1);
        //println!("Haven't Done stuff: {}", counter2);
    }

    // Sort descending so best moves come first
    if board.turn == Some(Player::Black) {
        scored_moves.sort_by(|a, b| b.1.cmp(&a.1));
    }
    else if board.turn == Some(Player::White) {
        scored_moves.sort_by(|a, b| a.1.cmp(&b.1));
    }

    let scored_moves = scored_moves.into_iter().map(|(ply, _)| ply).collect();

    scored_moves
}
