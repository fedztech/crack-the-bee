#![warn(missing_docs)]

//! This program provides helpful tools to crate and solve NY Times Games.

mod args;
mod games;
mod reader;

fn main() {
    println!("crack-the-games");

    let game_args: args::game::GameArgs = argh::from_env();
    match game_args.validate() {
        Some(error) => {
            println!("{}", error.to_string());
            println!("Use --help to get a description of the usage.");
            std::process::exit(1);
        }
        None => {
            // All good.
        }
    }

    if game_args.spellingbee {
        // Get word reader
        let word_reader_result: Option<Box<dyn std::io::BufRead>> =
            reader::factory::get_word_dictionary_reader(&game_args);
        match word_reader_result {
            Some(mut word_reader) => {
                let words_result =
                    games::bee::get_spelling_bee_suggestions(game_args, &mut word_reader);
                match words_result {
                    Ok(words) => {
                        for value in words.iter() {
                            println!("{}", value);
                        }
                    }
                    Err(e) => {
                        println!("{}", e);
                        std::process::exit(3);
                    }
                }
            }
            None => {
                println!("Failed to create a word reader.");
                println!("Use --help to get a description of the usage.");
                std::process::exit(2);
            }
        }
    } else if game_args.wordle {
        games::word::get_wordle_suggestions(game_args);
    } else {
    }

    std::process::exit(0);
}
