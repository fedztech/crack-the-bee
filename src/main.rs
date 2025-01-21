use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path;
use std::rc::Rc;

mod args;

use args::crack_the_bee::CrackTheBeeArgs;

fn print_game_letters(letters: &[char; args::crack_the_bee::NUM_LETTERS]) {
    println!("Letters captured.");
    println!("Main letter: {}", letters[0].to_string());
    for letter_index in 1..letters.len() {
        println!("Letter {} = {}", letter_index, letters[letter_index]);
    }
}

fn get_word_reader_for_file(file_path: &str) -> Result<BufReader<File>, std::io::Error> {
    let word_file_result = File::open(file_path);

    match word_file_result {
        Ok(word_file) => {
            let file_reader = io::BufReader::new(word_file);
            //let file_reader_ptr = Rc::new(file_reader);
            return Ok(file_reader);
        }
        Err(e) => {
            let open_file_error =
                std::io::Error::new(std::io::ErrorKind::NotFound, "Failed to open file");
            return Err(open_file_error);
        }
    }
}

fn filter_words(
    reader: &mut BufReader<File>,
    letters: &[char; args::crack_the_bee::NUM_LETTERS],
) -> Result<Rc<Vec<String>>, std::io::Error> {
    let mut word_list: Vec<String> = Vec::new();

    let mut regular_expression = r"^[".to_string();
    for letter in letters {
        regular_expression += &letter.to_string();
    }
    regular_expression += "]+$";

    let empty_string: String = String::new();
    reader
        .lines()
        .filter(|line| line.as_ref().unwrap_or(&empty_string).len() >= 4)
        .filter(|line| {
            line.as_ref()
                .unwrap_or(&empty_string)
                .contains(letters.as_slice()[0])
                == true
        })
        .filter(|line| line.as_ref().unwrap_or(&empty_string).contains("'") == false)
        .filter(|line| line.as_ref().unwrap_or(&empty_string).starts_with(letters) == true)
        .filter(|line| {
            let re = Regex::new(regular_expression.as_str()).unwrap();
            return re.is_match(line.as_ref().unwrap_or(&empty_string));
        })
        .for_each(|line| {
            word_list.push(line.expect("blah"));
        });

    let mut ret_val: Rc<Vec<String>> = Rc::new(word_list);
    return Ok(ret_val);
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

    let mut letters: [char; args::crack_the_bee::NUM_LETTERS] =
        ['a'; args::crack_the_bee::NUM_LETTERS];
    for letter_ix in 0..letters.len() {
        let char_conversion =
            char::from_u32(crack_the_bee_args.letters.as_bytes()[letter_ix].clone() as u32);
        match char_conversion {
            Some(converted_letter) => {
                letters[letter_ix] = converted_letter;
            }
            None => {
                // Something went wrong.
            }
        }
    }
    print_game_letters(&letters);

    // Get word reader
    let mut word_reader: Option<BufReader<File>> = None;

    let mut word_file_reader_result: Result<BufReader<File>, io::Error>;
    match crack_the_bee_args.file_path {
        Some(path) => {
            word_file_reader_result = get_word_reader_for_file(path.as_str());
            word_reader = match word_file_reader_result {
                Ok(reader) => {
                    // Ok, we will proceed below to avoid too much nesting of pattern matchings.
                    Some(reader)
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

    // Filter words
    //let word_reader = word_reader_result.as_mut().unwrap();
    if word_reader.is_some() {
        let mut words_result = filter_words(&mut word_reader.unwrap(), &letters);
        match words_result {
            Ok(ref mut words) => {
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

    std::process::exit(0);
}
