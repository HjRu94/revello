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


#[derive(Clone)]
pub struct Board {
    pub white: u64,
    pub black: u64,
    pub turn: Player,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Player {
    Black,
    White,
}

#[derive(Clone)]
pub struct Ply(u64);

pub struct Plys(u64);

impl Board {
    pub fn new(black: u64, white: u64, turn: Player) -> Self {
        Board {
            white: white,
            black: black,
            turn: turn
        }
    }
}

pub const START_BOARD: Board = Board {
    white: 0x0000001008000000,
    black: 0x0000000810000000,
    turn: Player::Black,
};

use std::fmt;
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Turn: {}", if self.turn == Player::Black { "Black" } else { "White" })?;
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
    pub const unsafe fn new_unchecked(ply: u64) -> Self {
        Self(ply)
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
    let player = if board.turn == Player::Black { board.black } else { board.white};
    let opponent = if board.turn == Player::Black { board.white } else { board.black };
    let total = player | opponent;

    let t: u64 = 0xFFFFFFFFFFFFFF00;
    let r: u64 = 0x7F7F7F7F7F7F7F7F;
    let l: u64 = 0xFEFEFEFEFEFEFEFE;
    let b: u64 = 0x00FFFFFFFFFFFFFF;

    let mut mask: u64;

    // north
    mask = (player >> 8) & opponent & t;
    for _ in 0..5 { mask |= (mask >> 8) & opponent & t; };
    let n = (mask >> 8) & !total;

    // south
    mask = (player << 8) & opponent & b;
    for _ in 0..5 { mask |= (mask << 8) & opponent & b; };
    let s = (mask << 8) & !total;

    // east
    mask = (player << 1) & opponent & r;
    for _ in 0..5 { mask |= (mask << 1) & opponent & r; };
    let e = (mask << 1) & !total;

    // west
    mask = (player >> 1) & opponent & l;
    for _ in 0..5 { mask |= (mask >> 1) & opponent & l; };
    let w = (mask >> 1) & !total;

    // north east
    mask = (player >> 7) & opponent & r & t;
    for _ in 0..5 { mask |= (mask >> 7) & opponent & r & t; };
    let ne = (mask >> 7) & !total;

    // north west
    mask = (player >> 9) & opponent & l & t;
    for _ in 0..5 { mask |= (mask >> 9) & opponent & l & t; };
    let nw = (mask >> 9) & !total;

    // south east
    mask = (player << 9) & opponent & r & b;
    for _ in 0..5 { mask |= (mask << 9) & opponent & r & b; };
    let se = (mask << 9) & !total;

    // south west
    mask = (player << 7) & opponent & l & b;
    for _ in 0..5 { mask |= (mask << 7) & opponent & l & b; };
    let sw = (mask << 7) & !total;

    let plys = n | s | e | w | ne | nw | se | sw;

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
    let player = if board.turn == Player::Black { board.black } else { board.white};
    let opponent = if board.turn == Player::Black { board.white } else { board.black };

    let total = player | opponent;

    let uply: u64 = ply.into();

    if (total & uply) != 0 {
        return board.clone();
    }

    let t: u64 = 0xFFFFFFFFFFFFFF00;
    let b: u64 = 0x00FFFFFFFFFFFFFF;
    let r: u64 = 0x7F7F7F7F7F7F7F7F;
    let l: u64 = 0xFEFEFEFEFEFEFEFE;

    let mut n = uply >> 8 & opponent & t; let mut nr = player << 8 & opponent & b;
    let mut ne = uply >> 7 & opponent & r & t; let mut ner = player << 7 & opponent & l & b;
    let mut e = uply << 1 & opponent & r; let mut er = player >> 1 & opponent & l;
    let mut se = uply << 9 & opponent & r & b; let mut ser = player >> 9 & opponent & l & t;
    let mut s = uply << 8 & opponent & b; let mut sr = player >> 8 & opponent & t;
    let mut sw = uply << 7 & opponent & l & b; let mut swr = player >> 7 & opponent & r & t;
    let mut w = uply >> 1 & opponent & l; let mut wr = player << 1 & opponent & r;
    let mut nw = uply >> 9 & opponent & l & t; let mut nwr = player << 9 & opponent & r & b;

    for _ in 0..6 {
        n |= n >> 8 & opponent & t; nr |= nr << 8 & opponent & b;
        ne |= ne >> 7 & opponent & r & t; ner |= ner << 7 & opponent & l & b;
        e |= e << 1 & opponent & r; er |= er >> 1 & opponent & l;
        se |= se << 9 & opponent & r & b; ser |= ser >> 9 & opponent & l & t;
        s |= s << 8 & opponent & b; sr |= sr >> 8 & opponent & t;
        sw |= sw << 7 & opponent & l & b; swr |= swr >> 7 & opponent & r & t;
        w |= w >> 1 & opponent & l; wr |= wr << 1 & opponent & r;
        nw |= nw >> 9 & opponent & l & t; nwr |= nwr << 9 & opponent & r & b;
    }

    n = n & nr;
    ne = ne & ner;
    e = e & er;
    se = se & ser;
    s = s & sr;
    sw = sw & swr;
    w = w & wr;
    nw = nw & nwr;

    let flip = n | ne | e | se | s | sw | w | nw;

    if flip == 0 {
        return board.clone();
    }

    let mut new_white = board.white ^ flip;
    let mut new_black = board.black ^ flip;

    if board.turn == Player::Black {new_black |= uply} else {new_white |= uply};

    Board::new(new_black, new_white, !board.turn)
}
