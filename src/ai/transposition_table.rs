use crate::board::board::{Board};
use crate::ai::minmax::{MinMaxResponse};
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
