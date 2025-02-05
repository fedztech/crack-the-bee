#![warn(missing_docs)]
use crate::args::game::PossiblePaths;
use crate::reader;
use std::io;

/// Given the arguments, returns a BufRead to read the words.
///
/// If the GameArgs::file_path is given, then the BufRead will read from a file.
///
/// # Arguments
///
/// * `game_args`: The game arguments used to deduct the source of the word dictionary.
///
/// # Returns
///
/// An optional with either `Box<dyn std::io::BufRead>` or `None`
pub fn get_word_dictionary_reader<T>(game_args: &T) -> Option<Box<dyn std::io::BufRead>>
where
    T: PossiblePaths,
{
    let word_file_reader_result: Result<Box<dyn std::io::BufRead>, io::Error>;
    match &game_args.file_path() {
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
            if let Some(url) = &game_args.url() {
                let reader = reader::net_word_reader::create_net_word_reader(url);
                return Some(reader.unwrap());
            }
        }
    }

    // Other readers to be generated here.

    None
}
