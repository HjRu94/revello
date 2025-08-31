// This file contains the board representation and the game logic
//
// The Board is represented by two bitboards, one for each player.
// The Ply represents a move on the board, and Plys represents a set of possible moves.
//
// The board is represented as follows:
// 00 01 02 03 04 05 06 07
// 08 09 10 11 12 13 14 15
// 16 17 18 19 20 21 22 23
// 24 25 26 27 28 29 30 31
// 32 33 34 35 36 37 38 39
// 40 41 42 43 44 45 46 47
// 48 49 50 51 52 53 54 55
// 56 57 58 59 60 61 62 63


#[derive(Clone, PartialEq, Hash, Eq)]
pub struct Board {
    pub black: u64,
    pub white: u64,
    pub turn: Option<Player>,
}

#[derive(PartialEq, Clone, Copy, Hash, Eq)]
pub enum Player {
    Black,
    White,
}
#[derive(PartialEq, Clone, Copy)]
pub enum Piece {
    Black,
    White
}

#[derive(Clone, PartialEq, Eq, Copy, Hash)]
pub struct Ply(u64);

#[derive(PartialEq)]
pub struct Plys(u64);

impl Board {
    pub fn new(black: u64, white: u64, turn: Option<Player>) -> Option<Self> {
        if black & white == 0 {
            return Some(Board {
                white: white,
                black: black,
                turn: turn
            })
        }
        None
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Piece> {
        let index: usize = row * 8 + col;
        if ((1 << index) & self.white) != 0{
            return Some(Piece::White)
        }
        if ((1 << index) & self.black) != 0 {
            return Some(Piece::Black)
        }
        return None
    }
    pub fn flip_turn(&mut self) {
        if self.turn == None {
            return ;
        }
        self.turn = Some(!self.turn.expect("turn is None"));
    }
    
    pub fn set_turn(&mut self, turn: Option<Player>) {
        self.turn = turn;
    }
    pub fn get_turn(&self) -> Option<Player> {
        return self.turn;
    }

    pub fn count_pieces(&self) -> i32 {
        return (self.black | self.white).count_ones().try_into().unwrap();
    }

    pub fn count_black(&self) -> i32 {
        return self.black.count_ones().try_into().unwrap();
    }

    pub fn count_white(&self) -> i32 {
        return self.white.count_ones().try_into().unwrap();
    }
}

pub const START_BOARD: Board = Board {
    black: 0x0000000810000000,
    white: 0x0000001008000000,
    turn: Some(Player::Black),
};

use std::fmt;
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Turn: {}", if self.turn == Some(Player::Black) { "Black" } else if self.turn == Some(Player::White) { "White"} else { "None" })?;
        for row in 0..8 {
            for col in 0..8 {
                let index = row * 8 + col;
                let mask = 1 << index;
                let piece = if self.white & mask != 0 {
                    'W' // White piece
                } else if self.black & mask != 0 {
                    'B' // Black piece
                } else {
                    '.' // Empty square
                };
                write!(f, "{} ", piece)?;
            }
            writeln!(f)?; // Newline at end of row
        }
        Ok(())
    }
}

impl fmt::Display for Ply {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..8 {
            for j in 0..8 {
                let a = (self.0 >> (i * 8 + j)) % 2;
                write!(f, "{} ", a)?;
            }
            writeln!(f) ?;
        }
        Ok(())
    }
}

impl fmt::Display for Plys {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..8 {
            for j in 0..8 {
                let a = (self.0 >> (i * 8 + j)) % 2;
                write!(f, "{} ", a)?;
            }
            writeln!(f) ?;
        }
        Ok(())
    }
}


impl std::ops::Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}


// Implement new for Ply and Plys
impl Ply {
    pub fn new(ply: u64) -> Option<Self> {
        if (ply & (ply - 1)) == 0 {
            Some(Self(ply))
        } else {
            None
        }
    }
    pub fn from_row_col(row: usize, col:usize) -> Option<Self> {
        if row < 8 && col < 8 {
            let index: u64 = row as u64 * 8 + col as u64;
            return Self::new(1 << index);
        }
        None
    }

    pub const unsafe fn new_unchecked(ply: u64) -> Self {
        Self(ply)
    }

    pub fn to_row_col(&self) -> (usize, usize) {
        let idx = self.0.trailing_zeros() as usize;
        let row = idx / 8;
        let col = idx % 8;
        (row, col)
    }

    pub fn is_in(&self, plys: Plys) -> bool {
        return self.0 & plys.0 != 0;
    }
}

impl Plys {
    pub fn new(plys: u64) -> Self {
        Self(plys)
    }

    pub fn to_vec_ply(self) -> Vec<Ply>{
        let mut ret = Vec::new();
        let mut bit = 0;

        let mut n: u64 = self.into();

        while n > 0 {
            if n & 1 == 1 {
                ret.push(Ply::new(1u64 << bit).expect("this shouldn't happen"));
            }
            n >>= 1;
            bit += 1;
        }
        ret
    }
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl IntoIterator for Plys {
    type Item = Ply;
    type IntoIter = PlysIter;

    fn into_iter(self) -> Self::IntoIter {
        PlysIter(self.0)
    }
}

pub struct PlysIter(u64);

impl Iterator for PlysIter {
    type Item = Ply;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        // Extract lowest set bit
        let lsb = self.0 & (!self.0 + 1);
        self.0 &= !lsb;
        Some(Ply::new(lsb).expect("new is returning None"))
    }
}

// Implement `Into<u64>` for Ply and Plys
impl From<Ply> for u64 {
    fn from(ply: Ply) -> u64 {
        ply.0
    }
}

impl From<Plys> for u64 {
    fn from(plys: Plys) -> u64 {
        plys.0
    }
}


pub fn possible_plys(board: &Board) -> Plys {
    if board.turn == None {
        return Plys::new(0);
    }
    let player = if board.turn.expect("turn is None") == Player::Black { board.black } else { board.white};
    let opponent = if board.turn.expect("turn is None") == Player::Black { board.white } else { board.black };
    let total = player | opponent;

    let t: u64 = 0xFFFFFFFFFFFFFF00;
    let r: u64 = 0x7F7F7F7F7F7F7F7F;
    let l: u64 = 0xFEFEFEFEFEFEFEFE;
    let b: u64 = 0x00FFFFFFFFFFFFFF;

    let tr: u64 = t & r;
    let tl: u64 = t & l;
    let br: u64 = b & r;
    let bl: u64 = b & l;

    let mut n = (player & t) >> 8 & opponent & t;
    let mut s = (player & b) << 8 & opponent & b;
    let mut e = (player & r) << 1 & opponent & r;
    let mut w = (player & l) >> 1 & opponent & l;
    let mut ne = (player & tr) >> 7 & opponent & tr;
    let mut se = (player & br) << 9 & opponent & br;
    let mut sw = (player & bl) << 7 & opponent & bl;
    let mut nw = (player & tl) >> 9 & opponent & tl;

    for _ in 0..5 {
        n |= n >> 8 & opponent & t;
        s |= s << 8 & opponent & b;
        e |= e << 1 & opponent & r;
        w |= w >> 1 & opponent & l;
        ne |= ne >> 7 & opponent & tr;
        se |= se << 9 & opponent & br;
        sw |= sw << 7 & opponent & bl;
        nw |= nw >> 9 & opponent & tl;
    }
    n = n >> 8 & !total;
    s = s << 8 & !total;
    e = e << 1 & !total;
    w = w >> 1 & !total;
    ne = ne >> 7 & !total;
    se = se << 9 & !total;
    sw = sw << 7 & !total;
    nw = nw >> 9 & !total;

    let plys = n | s | e | w | ne | se | sw | nw;
    Plys::new(plys)
}

pub fn play(board: &Board, ply: Ply) -> Board {

    // Playes the ply on the board.
    //
    // If the ply is not valid, the board is returned unchanged.
    //
    // Arguments:
    //  - board: The current board state
    //  - ply: The move to play
    //
    // Returns:
    // - The new board state

    if board.turn == None {
        return board.clone();
    }

    if possible_plys(board) == Plys(0) {

    }

    let player = if board.turn.expect("Turn is None") == Player::Black { board.black } else { board.white};
    let opponent = if board.turn.expect("Turn is None") == Player::Black { board.white } else { board.black };

    let t: u64 = 0xFFFFFFFFFFFFFF00;
    let b: u64 = 0x00FFFFFFFFFFFFFF;
    let r: u64 = 0x7F7F7F7F7F7F7F7F;
    let l: u64 = 0xFEFEFEFEFEFEFEFE;

    let tr: u64 = t & r;
    let tl: u64 = t & l;
    let br: u64 = b & r;
    let bl: u64 = b & l;

    let uply: u64 = ply.into();


    if uply & (player | opponent) != 0 {
        return board.clone();
    }

    let mut n = (uply & t) >> 8 & opponent & t;
    let mut nr = (player & b) << 8 & opponent & b;

    let mut s = (uply & b) << 8 & opponent & b;
    let mut sr = (player & t) >> 8 & opponent & t;

    let mut e = (uply & r) << 1 & opponent & r;
    let mut er = (player & l) >> 1 & opponent & l;

    let mut w = (uply & l) >> 1 & opponent & l;
    let mut wr = (player & r) << 1 & opponent & r;

    let mut ne = (uply & tr) >> 7 & opponent & tr;
    let mut ner = (player & bl) << 7 & opponent & bl;

    let mut se = (uply & br) << 9 & opponent & br;
    let mut ser = (player & tl) >> 9 & opponent & tl;

    let mut sw = (uply & bl) << 7 & opponent & bl;
    let mut swr = (player & tr) >> 7 & opponent & tr;

    let mut nw = (uply & tl) >> 9 & opponent & tl;
    let mut nwr = (player & br) << 9 & opponent & br;

    for _ in 0..5 {
        n |= n >> 8 & opponent & t;
        nr |= nr << 8 & opponent & b;

        s |= s << 8 & opponent & b;
        sr |= sr >> 8 & opponent & t;

        e |= e << 1 & opponent & r;
        er |= er >> 1 & opponent & l;

        w |= w >> 1 & opponent & l;
        wr |= wr << 1 & opponent & r;

        ne |= ne >> 7 & opponent & tr;
        ner |= ner << 7 & opponent & bl;

        se |= se << 9 & opponent & br;
        ser |= ser >> 9 & opponent & tl;

        sw |= sw << 7 & opponent & bl;
        swr |= swr >> 7 & opponent & tr;

        nw |= nw >> 9 & opponent & tl;
        nwr |= nwr << 9 & opponent & br;
    }

    n &= nr;
    s &= sr;
    e &= er;
    w &= wr;
    ne &= ner;
    se &= ser;
    sw &= swr;
    nw &= nwr;

    let flip = n | s | e | w | ne | se | sw | nw ;

    if flip == 0 {
        return board.clone();
    }

    let mut new_white = board.white ^ flip;
    let mut new_black = board.black ^ flip;

    if board.turn.expect("turn is None") == Player::Black {new_black |= uply} else {new_white |= uply};

    let mut ret_board = Board::new(new_black, new_white, board.turn).expect("the black and white pieces are overlapping");
    ret_board.flip_turn();

    if possible_plys(&ret_board) == Plys::new(0) {

        ret_board.flip_turn();

        if possible_plys(&ret_board) == Plys::new(0) {
            ret_board.set_turn(None);
            return ret_board;
        }
        else {
            return ret_board;
        }
    }

    else {
        return ret_board;
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_gen() {
        let board = START_BOARD;
        assert_eq!(count_moves(&board, 1), 4);
        assert_eq!(count_moves(&board, 2), 12);
        assert_eq!(count_moves(&board, 3), 56);
        assert_eq!(count_moves(&board, 4), 244);
        assert_eq!(count_moves(&board, 5), 1396);
        assert_eq!(count_moves(&board, 6), 8200);
        assert_eq!(count_moves(&board, 7), 55092);
        assert_eq!(count_moves(&board, 8), 390216);
        // assert_eq!(count_moves(&board, 9), 3005288);
        // assert_eq!(count_moves(&board, 10), 24571284);
    }

    fn count_moves(board: &Board, depth: i32) -> i32 {
        if depth == 0 {
            return 1;
        }
        let mut total = 0;
        let plys = possible_plys(board);
        for ply in plys{
            total += count_moves(&play(board, ply), depth - 1);
        }
        return total;
    }
}
