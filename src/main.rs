mod env;
mod game;
mod testing_mode;
mod training_mode;

use clap::{App, Arg};
use testing_mode::testing_mode;
use training_mode::training_mode;

const LEARNING_RATE: f32 = 0.8;
const DISCOUNT: f32 = 0.9;
const N_EPISODES: usize = 500_000;

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("TicTacToe AI")
        .version("v1.0")
        .author("Dusten Knull <dakatk97@gmail.com>")
        .about("TicTacToe AI that learns using the Q Learning algorithm")
        .arg(
            Arg::with_name("test")
                .long("test")
                .short("t")
                .help("When this argument is specified, the program is run in 'policy test' mode"),
        )
        .get_matches();

    if matches.is_present("test") {
        testing_mode()
    } else {
        training_mode(LEARNING_RATE, DISCOUNT, N_EPISODES)
    }
}
