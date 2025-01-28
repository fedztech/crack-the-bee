use crate::reader;
use crate::args::game::GameArgs;
use std::io;


pub fn get_word_dictionary_reader(game_args: &GameArgs) -> Option<Box<dyn std::io::BufRead>> {
    let word_file_reader_result: Result<Box<dyn std::io::BufRead>, io::Error>;
    match &game_args.file_path {
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