use crate::graphics::graphics::{draw_board, detect_ply, draw_playable};
use crate::board::board::{Board, START_BOARD, Player, play, Ply};
use crate::ai::player::{HumanPlayer, MinMaxPlayer, Player as AiPlayer};
use macroquad::prelude::next_frame;

use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{Ordering, AtomicBool};

use std::time::Duration;

pub async fn ai_vs_ai() {

}

pub async fn human_vs_ai() {
    let mut board = START_BOARD.clone();

    let ai_player: MinMaxPlayer = MinMaxPlayer{};

    let ai_player_move: Arc<Mutex<Option<Ply>>> = Arc::new(Mutex::new(None));
    let ai_thinking = Arc::new(AtomicBool::new(false));

    loop {
        draw_board(&board);
        let ply = detect_ply();

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

            if ply != None {
                board = play(&board, ply.expect("Ply is none"));
            }
            draw_playable(&board);
        }

        if let Some(ply) = ai_player_move.lock().unwrap().take() {
            board = play(&board, ply);
        }

        next_frame().await;
    }
}
pub async fn human_vs_human() {
    let mut board = START_BOARD.clone();

    loop {
        let ply = detect_ply();

        if ply != None {
            board = play(&board, ply.expect("Ply is none"));
        }
        draw_board(&board);
        draw_playable(&board);
        next_frame().await;
    }
}
