use crate::graphics::graphics::{draw_board, detect_ply, draw_playable, draw_time, draw_timers, draw_side_pannel};
use crate::board::board::{START_BOARD, Player, play, Ply};
use crate::ai::player::{MinMaxPlayer, Player as AiPlayer};
use macroquad::prelude::next_frame;

use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{Ordering, AtomicBool};

use std::time::{Duration, Instant};

pub async fn ai_vs_ai() {
    let mut board = START_BOARD.clone();

    let total_time = Duration::from_secs(30);

    let mut black_time = total_time.clone();
    let mut white_time = total_time.clone();

    let start_time = Instant::now(); // start timer

    let ai_player1: MinMaxPlayer = MinMaxPlayer{};
    let ai_player2: MinMaxPlayer = MinMaxPlayer{};

    let ai_player1_move: Arc<Mutex<Option<Ply>>> = Arc::new(Mutex::new(None));
    let ai_player2_move: Arc<Mutex<Option<Ply>>> = Arc::new(Mutex::new(None));

    let ai_thinking = Arc::new(AtomicBool::new(false));

    loop {
        // draw
        next_frame().await;
        draw_board(&board);
        draw_side_pannel(&board);
        draw_timers(&black_time, &white_time, board.turn == Some(Player::Black));

        // Time keeping
        if board.turn == Some(Player::Black) {
            let elapsed = Instant::now() - start_time;
            if let Some(remaining) = (2 * total_time).checked_sub(elapsed + white_time) {
                black_time = remaining;
            } else {
                black_time = Duration::from_secs(0);
                continue;
            }
        } else if board.turn == Some(Player::White) {
            let elapsed = Instant::now() - start_time;
            if let Some(remaining) = (2 * total_time).checked_sub(elapsed + black_time) {
                white_time = remaining;
            } else {
                white_time = Duration::from_secs(0);
                continue;
            }
        }

        // move gen
        if board.turn == Some(Player::Black)
            && ai_player1_move.lock().unwrap().is_none()
            && !ai_thinking.load(Ordering::SeqCst)
        {
            let ai_player1_move_clone = ai_player1_move.clone();
            let board_clone = board.clone();
            let ai_player1_clone = ai_player1.clone();
            let ai_thinking_clone = ai_thinking.clone();

            ai_thinking.store(true, Ordering::SeqCst);

            thread::spawn( move || {
                let ply = ai_player1_clone.generate_ply(&board_clone, Duration::new(0, 0));
                *ai_player1_move_clone.lock().unwrap() = Some(ply);
                ai_thinking_clone.store(false, Ordering::SeqCst);
                    });
        }

        if board.turn == Some(Player::White)
            && ai_player2_move.lock().unwrap().is_none()
            && !ai_thinking.load(Ordering::SeqCst)
        {
            let ai_player2_move_clone = ai_player2_move.clone();
            let board_clone = board.clone();
            let ai_player2_clone = ai_player2.clone();
            let ai_thinking_clone = ai_thinking.clone();

            ai_thinking.store(true, Ordering::SeqCst);

            thread::spawn( move || {
                let ply = ai_player2_clone.generate_ply(&board_clone, Duration::new(0, 0));
                *ai_player2_move_clone.lock().unwrap() = Some(ply);
                ai_thinking_clone.store(false, Ordering::SeqCst);
                    });
        }

        if let Some(ply) = ai_player1_move.lock().unwrap().take() {
            board = play(&board, ply);
        }

        if let Some(ply) = ai_player2_move.lock().unwrap().take() {
            board = play(&board, ply);
        }

    }
}

pub async fn human_vs_ai() {

    let total_time = Duration::from_secs(30);

    let mut black_time = total_time.clone();
    let mut white_time = total_time.clone();

    let start_time = Instant::now(); // start timer
                                     //
    let mut board = START_BOARD.clone();

    let ai_player: MinMaxPlayer = MinMaxPlayer{};

    let ai_player_move: Arc<Mutex<Option<Ply>>> = Arc::new(Mutex::new(None));
    let ai_thinking = Arc::new(AtomicBool::new(false));

    loop {
        // draw
        next_frame().await;
        draw_board(&board);
        draw_side_pannel(&board);
        draw_timers(&black_time, &white_time, board.turn == Some(Player::Black));

        // Time Keeping
        if board.turn == Some(Player::Black) {
            let elapsed = Instant::now() - start_time;
            if let Some(remaining) = (2 * total_time).checked_sub(elapsed + white_time) {
                black_time = remaining;
            } else {
                black_time = Duration::from_secs(0);
                continue;
            }
        } else if board.turn == Some(Player::White) {
            let elapsed = Instant::now() - start_time;
            if let Some(remaining) = (2 * total_time).checked_sub(elapsed + black_time) {
                white_time = remaining;
            } else {
                white_time = Duration::from_secs(0);
                continue;
            }
        }

        // move gen
        if board.turn == Some(Player::Black)
            && ai_player_move.lock().unwrap().is_none()
            && !ai_thinking.load(Ordering::SeqCst)
        {
            let ai_player_move_clone = ai_player_move.clone();
            let board_clone = board.clone();
            let ai_player_clone = ai_player.clone();
            let ai_thinking_clone = ai_thinking.clone();

            ai_thinking.store(true, Ordering::SeqCst);

            thread::spawn( move || {
                let ply = ai_player_clone.generate_ply(&board_clone, Duration::new(0, 0));
                *ai_player_move_clone.lock().unwrap() = Some(ply);
                ai_thinking_clone.store(false, Ordering::SeqCst);
                    });
        }
        else if board.turn == Some(Player::White) {
            let ply = detect_ply();
            draw_playable(&board);

            if ply != None {
                board = play(&board, ply.expect("Ply is none"));
            }
        }

        if let Some(ply) = ai_player_move.lock().unwrap().take() {
            board = play(&board, ply);
        }

    }
}
pub async fn human_vs_human() {
    let mut board = START_BOARD.clone();
    let total_time = Duration::from_secs(30);

    let mut black_time = total_time.clone();
    let mut white_time = total_time.clone();

    let start_time = Instant::now(); // start timer
    loop {
        // Draw
        next_frame().await;
        draw_board(&board);
        draw_side_pannel(&board);
        draw_timers(&black_time, &white_time, board.turn == Some(Player::Black));

        // time keeping
        if board.turn == Some(Player::Black) {
            let elapsed = Instant::now() - start_time;
            if let Some(remaining) = (2 * total_time).checked_sub(elapsed + white_time) {
                black_time = remaining;
            } else {
                black_time = Duration::from_secs(0);
                continue;
            }
        } else if board.turn == Some(Player::White) {
            let elapsed = Instant::now() - start_time;
            if let Some(remaining) = (2 * total_time).checked_sub(elapsed + black_time) {
                white_time = remaining;
            } else {
                white_time = Duration::from_secs(0);
                continue;
            }
        }



        // Move gen
        draw_playable(&board);
        let ply = detect_ply();

        if ply != None {
            board = play(&board, ply.expect("Ply is none"));
        }
    }
}
