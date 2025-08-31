use crate::ai::player::{MinMaxPlayer, HumanPlayer};
use crate::board::board::Player;
use std::time::Duration;

mod board;
mod ai;
mod graphics;
mod entrypoints;

use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(name = "game-cli")]
#[command(about = "A game CLI with play and analyse modes", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Play the game
    Play(PlayOptions),

    /// Analyse a game
    Analyse {
        /// Path to the game file to analyse
        #[arg(short, long)]
        file: String,
    },
}

#[derive(Args)]
struct PlayOptions {
    /// Who plays as black (human, minmax)
    #[arg(long)]
    black: PlayerType,

    /// Who plays as white (human, minmax)
    #[arg(long)]
    white: PlayerType,

    /// Time for black (seconds)
    #[arg(long, default_value_t = 300)]
    black_time: u64,

    /// Time for white (seconds)
    #[arg(long, default_value_t = 300)]
    white_time: u64,
}

#[derive(Clone, Debug)]
enum PlayerType {
    Human,
    MinMax,
}

impl std::str::FromStr for PlayerType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "human" => Ok(PlayerType::Human),
            "minmax" => Ok(PlayerType::MinMax),
            _ => Err(format!("Invalid player type: {}", s)),
        }
    }
}

use macroquad::prelude::Conf;

fn window_conf() -> Conf {
    Conf {
        window_title: "Revello".to_owned(),
        window_width: 3100,
        window_height: 2000,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let cli = Cli::parse();
    use crate::entrypoints::play::player_vs_player;

    match cli.command {
        Commands::Play(opts) => {
            let black_time = Duration::from_secs(opts.black_time);
            let white_time = Duration::from_secs(opts.white_time);

            match (opts.black, opts.white) {
                (PlayerType::Human, PlayerType::Human) => {
                    let mut black = HumanPlayer::new(Player::Black);
                    let mut white = HumanPlayer::new(Player::White);
                    player_vs_player(&mut black, &mut white, black_time, white_time).await;
                }
                (PlayerType::Human, PlayerType::MinMax) => {
                    let mut black = HumanPlayer::new(Player::Black);
                    let mut white = MinMaxPlayer::new();
                    player_vs_player(&mut black, &mut white, black_time, white_time).await;
                }
                (PlayerType::MinMax, PlayerType::Human) => {
                    let mut black = MinMaxPlayer::new();
                    let mut white = HumanPlayer::new(Player::White);
                    player_vs_player(&mut black, &mut white, black_time, white_time).await;
                }
                (PlayerType::MinMax, PlayerType::MinMax) => {
                    let mut black = MinMaxPlayer::new();
                    let mut white = MinMaxPlayer::new();
                    player_vs_player(&mut black, &mut white, black_time, white_time).await;
                }
            }
        }
        Commands::Analyse { file } => {
            println!("Analysing file: {}", file);
        }
    }
}
