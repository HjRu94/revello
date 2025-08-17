// Board struct using bitboards to represent the othello board

mod board;
mod ai;
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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Play(play_mode) => match play_mode {
            PlayMode::AiVsAi(opts) => {
                //TODO Implement Ai vs Ai
                println!("AI vs AI with {:?} AI", opts.ai);
            }
            PlayMode::HumanVsHuman => {
                //TODO Implement human vs human
                println!("Human vs Human");
            }
            PlayMode::HumanVsAi(opts) => {
                //TODO Implement Human vs AI
                println!("Human vs AI with {:?} AI", opts.ai);
            }
        },
        Commands::Analyse { file } => {
            println!("Analysing file: {}", file);
        }
    }
}
