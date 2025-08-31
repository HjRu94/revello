use crate::ai::player::{MinMaxPlayer, HumanPlayer};
use crate::board::board::{Player};
use std::time::Duration;

mod board;
mod ai;
mod graphics;
mod entrypoints;

use clap::{Parser, Subcommand, Args, ValueEnum};

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
    #[command(subcommand)]
    Play(PlayMode),

    /// Analyse a game
    Analyse {
        /// Path to the game file to analyse
        #[arg(short, long)]
        file: String,
    },
}

#[derive(Subcommand)]
enum PlayMode {
    /// AI vs AI game
    AiVsAi(AiOptions),

    /// Human vs Human game
    HumanVsHuman,

    /// Human vs AI game
    HumanVsAi(AiOptions),

    PlayerVsPlayer,
}

#[derive(Args)]
struct AiOptions {
    /// Select which AI to use
    #[arg(short = 'a', long = "ai", value_enum, default_value_t = AiType::MinMax)]
    ai: AiType,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum AiType {
    MinMax,
    Random,
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
    use crate::entrypoints::play::{human_vs_ai, human_vs_human, ai_vs_ai, player_vs_player};

    match cli.command {
        Commands::Play(play_mode) => match play_mode {
            PlayMode::AiVsAi(opts) => {
                //TODO Implement Ai vs Ai
                println!("AI vs AI with {:?} AI", opts.ai);
                ai_vs_ai().await;
            }
            PlayMode::HumanVsHuman => {
                //TODO Implement human vs human
                println!("Human vs Human");
                human_vs_human().await;
            }
            PlayMode::HumanVsAi(opts) => {
                //TODO Implement Human vs AI
                println!("Human vs AI with {:?} AI", opts.ai);
                human_vs_ai().await;
            }
            PlayMode::PlayerVsPlayer => {
                //TODO Implement Human vs AI
                println!("Player vs Player");
                let black_time = Duration::from_secs(30);
                let white_time = Duration::from_secs(90);
                let mut black_player: MinMaxPlayer = MinMaxPlayer::new();
                let mut white_player: HumanPlayer = HumanPlayer::new(Player::White);
                player_vs_player(&mut black_player, &mut white_player, black_time, white_time).await;
            }
        },
        Commands::Analyse { file } => {
            println!("Analysing file: {}", file);
        }
    }
}
