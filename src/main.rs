use args::crack_the_bee::CrackTheBeeArgs;
use regex::Regex;
use std::io::{self, BufRead};
use std::rc::Rc;

mod args;
mod games;
mod reader;

fn get_word_dictionary_reader(
    crack_the_bee_args: &CrackTheBeeArgs,
) -> Option<Box<dyn std::io::BufRead>> {
    let word_file_reader_result: Result<Box<dyn std::io::BufRead>, io::Error>;
    match &crack_the_bee_args.file_path {
        Some(path) => {
            word_file_reader_result =
                reader::file_word_reader::create_file_word_reader(path.as_str());
            match word_file_reader_result {
                Ok(reader) => {
                    // Ok, we will proceed below to avoid too much nesting of pattern matchings.
                    return Some(reader);
                }
                Err(e) => {
                    println!("Failed to capture the letters: {}", e.to_string());
                    std::process::exit(2);
                }
            }
        }
        None => {
            // The user wants to use something other than a file1
        }
    }

    // Other readers to be generated here.

    None
}

fn main() {
    println!("crack-the-bee");

    let crack_the_bee_args: args::crack_the_bee::CrackTheBeeArgs = argh::from_env();
    match crack_the_bee_args.validate() {
        Some(error) => {
            println!("{}", error.to_string());
            println!("Use --help to get a description of the usage.");
            std::process::exit(1);
        }
        None => {
            // All good.
        }
    }

    // Get word reader
    let word_reader_result: Option<Box<dyn std::io::BufRead>> =
        get_word_dictionary_reader(&crack_the_bee_args);
    match word_reader_result {
        Some(mut word_reader) => {
            if crack_the_bee_args.spellingbee {
                let words_result =
                    games::bee::get_spelling_bee_suggestions(crack_the_bee_args, &mut word_reader);
                match words_result {
                    Ok(words) => {
                        for value in words.iter() {
                            println!("{}", value);
                        }
                    },
                    Err(e) => {
                        println!("{}", e);
                        std::process::exit(3);
                    }
                }
            } else if crack_the_bee_args.wordle {
                // TBD
            }
        }
        None => {
            println!("Failed to create a word reader.");
            println!("Use --help to get a description of the usage.");
            std::process::exit(2);
        }
    }

    std::process::exit(0);
}
