use crate::game::symbol::Symbol;
use crate::game::tictactoe::TicTacToe;
use rand::prelude::*;
use rl::environment::Environment;

const WIN_REWARD: f32 = 2.0;
const BLOCK_REWARD: f32 = 0.1;
const LOSE_REWARD: f32 = -1.0;
const DRAW_REWARD: f32 = 0.5;
const DEFAULT_REWARD: f32 = 0.0;

pub struct TicTacToeEnv {
    game: TicTacToe,
    rng: ThreadRng
}

impl TicTacToeEnv {
    pub fn new() -> Self {
        Self {
            game: TicTacToe::new(),
            rng: thread_rng()
        }
    }

    fn reward(&mut self, legal_moves: &Vec<u8>, action: u8) -> f32 {

        let mut losses = 0;

        for action in legal_moves {
            self.game.force_piece(Symbol::X, *action);
            let winner = self.game.is_winner(Symbol::X);
            self.game.force_piece(Symbol::EMPTY, *action);

            if winner {
                losses += 1;
            }
        }

        if losses > 0 {
            return LOSE_REWARD * (losses as f32);
        }

        self.game.force_piece(Symbol::X, action);
        let blocked = self.game.is_winner(Symbol::X);
        self.game.force_piece(Symbol::O, action);

        if blocked {
            BLOCK_REWARD
        } else {
            DEFAULT_REWARD
        }
    }
}

impl Environment for TicTacToeEnv {
    fn reset(&mut self) -> String {
        self.game.reset();
        self.game.flat()
    }

    fn step(&mut self, action: u8) -> (String, f32, bool) {
        self.game.place_piece(Symbol::O, action).unwrap();
        let state: String = self.game.flat();

        if self.game.is_winner(Symbol::O) {
            return (state, WIN_REWARD, true);
        }

        let legal_moves: Vec<u8> = self.game.legal_moves();
        if legal_moves.is_empty() {
            return (state, DRAW_REWARD, true);
        }

        let reward: f32 = self.reward(&legal_moves, action);
        let opponent_action: u8 = *legal_moves.choose(&mut self.rng).unwrap();

        self.game.place_piece(Symbol::X, opponent_action).unwrap();

        let done: bool = self.game.is_winner(Symbol::X);
        let state: String = self.game.flat();

        (state, reward, done)
    }

    fn allowed_actions(&self) -> Vec<u8> {
        self.game.legal_moves()
    }
}