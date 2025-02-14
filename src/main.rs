// Board struct using bitboards to represent the othello board


struct Board {
    // 00 01 02 03 04 05 06 07
    // 08 09 10 11 12 13 14 15
    // 16 17 18 19 20 21 22 23
    // 24 25 26 27 28 29 30 31
    // 32 33 34 35 36 37 38 39
    // 40 41 42 43 44 45 46 47
    // 48 49 50 51 52 53 54 55
    // 56 57 58 59 60 61 62 63

    // if turn is true, it is black's turn

    white: u64,
    black: u64,
    turn: bool,
}
fn printu64(number: u64) {
    for i in 0..8 {
        for j in 0..8 {
            let a = (number >> (i * 8 + j)) % 2;
            print!("{a}");
        }
        println!("")
    }
}

fn possible_moves(board: &Board) -> u64 {
    let player = if board.turn { board.black } else { board.white};
    let opponent = if board.turn { board.white } else { board.black };
    let total = player | opponent;

    let t: u64 = 0xFFFFFFFFFFFFFF00;
    let r: u64 = 0x7F7F7F7F7F7F7F7F;
    let l: u64 = 0xFEFEFEFEFEFEFEFE;
    let b: u64 = 0x00FFFFFFFFFFFFFF;

    let mut mask: u64 = 0;

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

    return n | s | e | w | ne | nw | se | sw;
}


fn main(){
    let startpos = Board {
        white: 0x0000001008000000,
        black: 0x0000000810000000,
        turn: true
    };

    let moves = possible_moves(&startpos);
    printu64(moves);
}
