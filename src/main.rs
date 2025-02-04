#![warn(missing_docs)]

//! This program provides helpful tools to crate and solve NY Times Games.

mod args;
mod games;
mod reader;

fn main() {
    println!("crack-the-games");

    let crack_args: args::game::CrackArgs = argh::from_env();

    match crack_args.game {
        args::game::GameSubcommands::Bee(args) => {
            if let Some(error) = args.validate() {
                println!("Invalid parameters for Bee game.");
                println!("{}", error.to_string());
                println!("Use --help to get a description of the usage.");
                std::process::exit(1);
            }

            // TODO: Remove this nesting.

            // Get word reader
            let word_reader_result: Option<Box<dyn std::io::BufRead>> =
                reader::factory::get_word_dictionary_reader(&args);
            match word_reader_result {
                Some(mut word_reader) => {
                    let words_result =
                        games::bee::get_spelling_bee_suggestions(args, &mut word_reader);
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
        }
        args::game::GameSubcommands::Word(args) => {
            if let Some(error) = args.validate() {
                println!("Invalid parameters for Word game.");
                println!("{}", error.to_string());
                println!("Use --help to get a description of the usage.");
                std::process::exit(1);
            }
            games::word::get_wordle_suggestions(args);
        }
    }

    std::process::exit(0);
}
