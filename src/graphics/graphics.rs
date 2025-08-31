use macroquad::prelude::*;
use crate::board::board::{Board, Piece, Ply, possible_plys, Player};

use std::time::Duration;

const SQUARE_SIZE:   f32 = 100.0;
const MARGIN:        f32 = 100.0;
const CIRCLE_RADIUS: f32 = 40.0;
const PLAYABLE_CIRCLE_RADIUS: f32 = 15.0;
const LINE_THICKNESS: f32 = 2.0;

const BLACK_COLOR: Color = Color::new(0.25, 0.25, 0.25, 1.0);
const WHITE_COLOR: Color = Color::new(0.92, 0.92, 0.88, 1.0);
const BACKGROUND_COLOR: Color = WHITE;
const BOARD_COLOR: Color = Color::new(0.35, 0.75, 0.35, 1.0);
const GRID_COLOR: Color = BLACK;

// Timer drawing variables

const TIMER_BOX_HEIGHT: f32 = 200.0;
const TIMER_BOX_WIDTH: f32 = 450.0;
const TIMER_BOX_TURN_COLOR: Color = Color::new(0.50, 0.50, 0.50, 1.0);
const TIMER_BOX_NOT_TURN_COLOR: Color = Color::new(0.70, 0.70, 0.70, 1.0);
const TIMEOUT_COLOR: Color = Color::new(0.80, 0.55, 0.55, 1.0);
const TIMER_FONT_SIZE: f32 = 200.0;
const TIMER_FONT_COLOR: Color = BLACK;

pub fn detect_ply() -> Option<Ply> {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return None;
    }
    let mut row: Option<usize> = None;
    let mut col: Option<usize> = None;
    let (mouse_x, mouse_y) = mouse_position();
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
pub fn draw_timers(black_time: &Duration, white_time: &Duration, is_black_turn: bool) {
    draw_time(2.0 * MARGIN + 8.0 * SQUARE_SIZE, MARGIN, black_time, is_black_turn, Player::Black);
    draw_time(2.0 * MARGIN + 8.0 * SQUARE_SIZE, MARGIN + 8.0 * SQUARE_SIZE - TIMER_BOX_HEIGHT, white_time, !is_black_turn, Player::White);
}

pub fn draw_time(x: f32, y: f32, time: &Duration, is_turn: bool, player: Player) {
    let total_micros: i32 = time.as_micros().try_into().unwrap();

    let seconds = total_micros / 1000000;
    let minuites = seconds / 60;
    let microseconds = total_micros % 1000000;
    let seconds = seconds % 60;
    let circle_color = if player == Player::Black {BLACK_COLOR} else {WHITE_COLOR};
    let separator = if !is_turn {':'}
    else {
        if seconds % 2 == 0 {':'} else {' '}
    };
    let time_string = if minuites == 0 && seconds < 10 {
        format!("{}{}{}", seconds, separator, microseconds / 10000)
    }
    else {
        format!("{}{}{}", minuites, separator, seconds)
    };
    let timer_box_color = if total_micros == 0 {
        TIMEOUT_COLOR
    }
    else {
        if is_turn {TIMER_BOX_TURN_COLOR} else {TIMER_BOX_NOT_TURN_COLOR}
    };
    // if time is up
    draw_rectangle(x, y, TIMER_BOX_WIDTH, TIMER_BOX_HEIGHT, timer_box_color);
    draw_circle(x + SQUARE_SIZE / 2.0, y + TIMER_BOX_HEIGHT / 2.0, CIRCLE_RADIUS, circle_color);
    draw_text(&time_string, x + SQUARE_SIZE, y + TIMER_BOX_HEIGHT / 2.0 + TIMER_FONT_SIZE / 4.0 , TIMER_FONT_SIZE, TIMER_FONT_COLOR);
}

pub fn draw_playable(board: &Board) {
    let plys = possible_plys(board);
    let color = match board.get_turn() {
        Some(Player::Black) => BLACK_COLOR,
        Some(Player::White) => WHITE_COLOR,
        None => BLACK, // this should not matter
    };
    for ply in plys {
        draw_ply(ply, color);
    }
}

fn draw_ply(ply: Ply, color: Color) {
    let (row, col) = ply.to_row_col();
    draw_circle(MARGIN + (col as f32 + 0.5) * SQUARE_SIZE, MARGIN + (row as f32 + 0.5) * SQUARE_SIZE, PLAYABLE_CIRCLE_RADIUS, color);
}

pub fn draw_board(board: &Board) {
    clear_background(BACKGROUND_COLOR);

    draw_grid();
    draw_pieces(board);
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
