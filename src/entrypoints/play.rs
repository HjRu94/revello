use crate::graphics::graphics::{draw_board, draw_timers, draw_side_pannel};
use crate::board::board::{START_BOARD, Player, play, Ply};
use crate::ai::player::{Player as AiPlayer};
use macroquad::prelude::next_frame;

use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{Ordering, AtomicBool};

use std::time::{Duration, Instant};

pub async fn player_vs_player<T, U>(
    black_player: &mut T,
    white_player: &mut U,
    black_time: Duration,
    white_time: Duration,
    )
where
    T: AiPlayer + Clone + std::marker::Send + 'static,
    U: AiPlayer + Clone + std::marker::Send + 'static,
{
    let mut board = START_BOARD.clone();

    let black_total_time = black_time.clone();
    let white_total_time = white_time.clone();

    let mut black_time = black_total_time.clone();
    let mut white_time = white_total_time.clone();

    let start_time = Instant::now(); // start timer

    let black_move: Arc<Mutex<Option<Ply>>> = Arc::new(Mutex::new(None));
    let white_move: Arc<Mutex<Option<Ply>>> = Arc::new(Mutex::new(None));

    let ai_thinking = Arc::new(AtomicBool::new(false));

    loop {
        // update
        black_player.update(&board);
        white_player.update(&board);
        // draw
        next_frame().await;
        draw_board(&board);
        draw_side_pannel(&board);
        draw_timers(&black_time, &white_time, board.turn == Some(Player::Black));

        // Time keeping
        if board.turn == Some(Player::Black) {
            let elapsed = Instant::now() - start_time;
            if let Some(remaining) = (black_total_time + white_total_time).checked_sub(elapsed + white_time) {
                black_time = remaining;
            } else {
                black_time = Duration::from_secs(0);
                continue;
            }
        } else if board.turn == Some(Player::White) {
            let elapsed = Instant::now() - start_time;
            if let Some(remaining) = (black_total_time + white_total_time).checked_sub(elapsed + black_time) {
                white_time = remaining;
            } else {
                white_time = Duration::from_secs(0);
                continue;
            }
        }

        // move gen
        if board.turn == Some(Player::Black)
            && black_move.lock().unwrap().is_none()
            && !ai_thinking.load(Ordering::SeqCst)
        {
            let black_move_clone = black_move.clone();
            let board_clone = board.clone();
            let black_player_clone = black_player.clone();
            let ai_thinking_clone = ai_thinking.clone();

            ai_thinking.store(true, Ordering::SeqCst);

            thread::spawn( move || {
                let ply = black_player_clone.generate_ply(&board_clone, black_time);
                *black_move_clone.lock().unwrap() = Some(ply);
                ai_thinking_clone.store(false, Ordering::SeqCst);
                    });
        }

        if board.turn == Some(Player::White)
            && white_move.lock().unwrap().is_none()
            && !ai_thinking.load(Ordering::SeqCst)
        {
            let white_move_clone = white_move.clone();
            let board_clone = board.clone();
            let white_player_clone = white_player.clone();
            let ai_thinking_clone = ai_thinking.clone();

            ai_thinking.store(true, Ordering::SeqCst);

            thread::spawn( move || {
                let ply = white_player_clone.generate_ply(&board_clone, white_time);
                *white_move_clone.lock().unwrap() = Some(ply);
                ai_thinking_clone.store(false, Ordering::SeqCst);
                    });
        }

        if let Some(ply) = black_move.lock().unwrap().take() {
            board = play(&board, ply);
        }

        if let Some(ply) = white_move.lock().unwrap().take() {
            board = play(&board, ply);
        }

    }
}
