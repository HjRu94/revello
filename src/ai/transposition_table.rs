use crate::board::board::{Board};
use crate::ai::minmax::{MinMaxResponse};
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
pub struct TranspositionEntry {
    board: Board,
    depth: u32,
}

impl TranspositionEntry {
    pub fn new(board: Board, depth: u32) -> Self{
        return TranspositionEntry{
            board: board,
            depth: depth
        }
    }
}

pub struct TranspositionTable {
    table: HashMap<TranspositionEntry, MinMaxResponse>
}

impl TranspositionTable {
    pub fn new() -> Self {
        return TranspositionTable {
            table: HashMap::new()
        }
    }

    pub fn get(&self, entry: &TranspositionEntry) -> Option<MinMaxResponse> {
        self.table.get(entry).copied()
    }

    pub fn insert(&mut self, entry: TranspositionEntry, value: MinMaxResponse) {
        self.table.insert(entry, value);
    }
}
