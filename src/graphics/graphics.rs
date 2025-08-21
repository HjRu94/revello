use macroquad::prelude::*;
use crate::board::board::{Board, Piece, Ply};

const SQUARE_SIZE:   f32 = 100.0;
const MARGIN:        f32 = 100.0;
const CIRCLE_RADIUS: f32 = 40.0;
const LINE_THICKNESS: f32 = 2.0;

const BLACK_COLOR: Color = DARKGRAY;
const WHITE_COLOR: Color = LIGHTGRAY;
const BACKGROUND_COLOR: Color = WHITE;
const BOARD_COLOR: Color = GREEN;
const GRID_COLOR: Color = BLACK;

pub fn detect_ply() -> Option<Ply> {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return None;
    }
    let mut row: Option<usize> = None;
    let mut col: Option<usize> = None;
    let (mut mouse_x, mut mouse_y) = mouse_position();
    for i in 0..8 {
        for j in 0..8 {
            if MARGIN + i as f32 * SQUARE_SIZE <= mouse_x && mouse_x <= MARGIN + (i as f32 + 1.0) * SQUARE_SIZE {
                if MARGIN + j as f32 * SQUARE_SIZE <= mouse_y && mouse_y <= MARGIN + (j as f32 + 1.0) * SQUARE_SIZE {
                    row = Some(j);
                    col = Some(i);
                }
            }
        }
    }
    if row == None || col == None {
        return None;
    }
    let row = row.expect("row is None");
    let col = col.expect("col is None");

    let ply: Ply = Ply::from_row_col(row, col).expect("Ply returned None");

    return Some(ply);
}

pub async fn draw_board(board: &Board) {
    clear_background(BACKGROUND_COLOR);

    draw_grid();
    draw_pieces(board);

    next_frame().await;
}

fn draw_grid() {
    draw_rectangle(MARGIN, MARGIN, 8.0 * SQUARE_SIZE, 8.0 * SQUARE_SIZE, BOARD_COLOR);
    for i in 0..9 {
        draw_line(MARGIN + i as f32 * SQUARE_SIZE, MARGIN, MARGIN + i as f32 * SQUARE_SIZE, MARGIN + 8.0 * SQUARE_SIZE, LINE_THICKNESS, GRID_COLOR);
        draw_line(MARGIN, MARGIN + i as f32 * SQUARE_SIZE, MARGIN + 8.0 * SQUARE_SIZE, MARGIN + i as f32 * SQUARE_SIZE,  LINE_THICKNESS, GRID_COLOR);
    }
}

fn draw_pieces(board: &Board) {
    for i in 0..8 {
        for j in 0..8 {
            let piece = board.get(i, j);
            if piece == None {
                continue;
            }
            if piece.expect("") == Piece::White {
                draw_circle(MARGIN + (j as f32 + 0.5) * SQUARE_SIZE, MARGIN + (i as f32 + 0.5) * SQUARE_SIZE, CIRCLE_RADIUS, WHITE_COLOR);
            }
            if piece.expect("") == Piece::Black {
                draw_circle(MARGIN + (j as f32 + 0.5) * SQUARE_SIZE, MARGIN + (i as f32 + 0.5) * SQUARE_SIZE, CIRCLE_RADIUS, BLACK_COLOR);
            }
        }
    }
}
