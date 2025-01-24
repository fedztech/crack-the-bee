use args::crack_the_bee::CrackTheBeeArgs;
use regex::Regex;
use std::io::{self, BufRead};
use std::rc::Rc;

mod args;
mod reader;

fn print_game_letters(letters: &[char; args::crack_the_bee::NUM_LETTERS]) {
    println!("Letters captured.");
    println!("Main letter: {}", letters[0].to_string());
    for letter_index in 1..letters.len() {
        println!("Letter {} = {}", letter_index, letters[letter_index]);
    }
}

fn filter_words<T>(
    reader: &mut T,
    letters: &[char; args::crack_the_bee::NUM_LETTERS],
) -> Result<Rc<Vec<String>>, std::io::Error>
where
    T: BufRead,
{
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

fn set_game_letter_array(
    letter_string: &String,
    letter_array: &mut [char; args::crack_the_bee::NUM_LETTERS],
) {
    for letter_ix in 0..letter_array.len() {
        let char_conversion = char::from_u32(letter_string.as_bytes()[letter_ix].clone() as u32);
        match char_conversion {
            Some(converted_letter) => {
                letter_array[letter_ix] = converted_letter;
            }
            None => {
                // Something went wrong.
            }
        }
    }
}

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

    let mut letters: [char; args::crack_the_bee::NUM_LETTERS] =
        ['a'; args::crack_the_bee::NUM_LETTERS];
    set_game_letter_array(&crack_the_bee_args.letters, &mut letters);
    print_game_letters(&letters);

    // Get word reader
    let word_reader_result: Option<Box<dyn std::io::BufRead>> =
        get_word_dictionary_reader(&crack_the_bee_args);
    match word_reader_result {
        Some(mut word_reader) => {
            let mut words_result = filter_words(&mut word_reader, &letters);
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
        None => {
            println!("Failed to create a word reader.");
            println!("Use --help to get a description of the usage.");
            std::process::exit(2);}
    }

    std::process::exit(0);
}
