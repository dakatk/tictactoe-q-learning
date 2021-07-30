use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, stdout};

use crate::game::{symbol::Symbol, tictactoe::TicTacToe};

/// Tests the AI against a human player using the
/// most recently generated policy
///
/// # Returns
///
/// Any IO errors that were caught while reading
/// 'actions.json', Ok(()) otherwise
pub fn testing_mode() -> Result<(), std::io::Error> {
    let mut file = match File::open("actions.json") {
        Err(err) => return Err(err),
        Ok(file) => file,
    };

    let actions: HashMap<_, _> = {
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)
            .expect("Error reading file contents");

        let policy_json: HashMap<String, u8> =
            serde_json::from_str(file_contents.as_str()).unwrap();
        policy_json
    };
    play_game(actions);

    Ok(())
}

/// Single game of TicTacToe between a human player
/// and AI derived from `policy`
///
/// * `actions` - The most recently generated policy,
/// deserialized from 'actions.json'
fn play_game(actions: HashMap<String, u8>) {
    let mut game = TicTacToe::new();
    println!("{}\n", game);

    loop {
        let state: String = game.flat();
        let action: u8 = if actions.contains_key(&state) {
            actions[&state]
        } else {
            game.legal_moves()[0]
        };

        game.place_piece(Symbol::O, action).unwrap();

        let action_row = action / 3;
        let action_col = action % 3;

        println!("O's turn: ({}, {})\n\n{}\n", action_row, action_col, game);

        if game.is_winner(Symbol::O) {
            println!("O wins!");
            break;
        } else if game.legal_moves().is_empty() {
            println!("Draw!");
            break;
        }

        print!("X's turn (row, col): ");
        stdout().flush().unwrap();

        let mut line = String::with_capacity(3);
        stdin().read_line(&mut line).unwrap();

        let (row, col) = {
            let values: Vec<&str> = line.trim().split(',').collect();

            let row: u8 = values[0].to_string().parse().unwrap();
            let col: u8 = values[1].to_string().parse().unwrap();

            (row, col)
        };

        let player_action = row * 3 + col;
        game.place_piece(Symbol::X, player_action).unwrap();

        println!("\n{}\n", game);
        if game.is_winner(Symbol::X) {
            println!("X wins!");
            break;
        }
    }
}
