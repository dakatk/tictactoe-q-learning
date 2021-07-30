use crate::env::tictactoe_env::TicTacToeEnv;
use rl::policies::GreedyPolicy;
use rl::temporal_difference::off_policy::OffPolicyTd;
use rl::trainer::Trainer;
use serde_json;
use std::io::prelude::*;
use std::path::Path;
use std::{collections::BTreeMap, fs::File};

/// Trains the AI through a given number of episodes,
/// then writes the resulting policy to a JSON file
///
/// * `n_epsiodes` - The number of episodes (games) to simulate
///
/// # Returns
///
/// Any IO errors that were caught while writing the JSON result
/// to a file, Ok(()) otherwise
pub fn training_mode(
    learning_rate: f32,
    discount: f32,
    n_episodes: usize,
) -> Result<(), std::io::Error> {
    let mut env = TicTacToeEnv::new();
    let behavior_policy = GreedyPolicy::new(None);
    let mut ai = OffPolicyTd::<u8>::new(learning_rate, discount);

    let actions: BTreeMap<&String, &u8> = Trainer::new(n_episodes, false)
        .train(&mut ai, &behavior_policy, &mut env)
        .results()
        .iter()
        .collect();

    let mut file = File::create(&Path::new("actions.json")).unwrap();
    let actions_json = serde_json::to_string_pretty(&actions).unwrap();

    file.write_all(actions_json.as_bytes())
}
