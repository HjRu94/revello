# ⚫⚪ Revello

Revello is a Rust-based project for exploring and experimenting with an
**Othello (Reversi) bot**.

## ⚙️ How it works

At its core, the bot combines an efficient **bitboard-based move
generator** with a **search algorithm**.
The search engine includes several techniques:

-   **Alpha-Beta Pruning** -- reduces the number of positions evaluated
    by pruning irrelevant branches.
-   **Transposition Tables** -- caches previously evaluated positions to
    avoid redundant work.
-   **Iterative Deepening** -- gradually deepens the search, allowing
    early cutoffs and better move ordering.
-   **Move Ordering** -- prioritizes promising moves based on results
    from earlier searches.

## 🚀 Running the program

You can run the program in different modes using Cargo:

``` bash
# start a game
cargo run -- play --black <player_type> --white <player_type> [--black-time <seconds>] [--white-time <seconds>]

# 🧑 vs 🤖 Play human vs AI with 5 minutes for the human and 30 seconds for the bot
cargo run -- play --black human --white minmax --black-time 300 --white-time 30

```

## 🛠️ Implementation details

-   🦀 The project is written **100% in Rust**.
-   🎨 The graphical interface is built using **macroquad**.

## 📜 License

This project is licensed under an **MIT License**
