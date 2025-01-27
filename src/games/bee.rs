use crate::args;
use regex::Regex;
use std::io::{self, BufRead};
use std::rc::Rc;

fn print_game_letters(letters: &[char; args::crack_the_bee::NUM_LETTERS]) {
    println!("Letters captured.");
    println!("Main letter: {}", letters[0].to_string());
    for letter_index in 1..letters.len() {
        println!("Letter {} = {}", letter_index, letters[letter_index]);
    }
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

    return Ok(Rc::new(word_list));
}

pub fn get_spelling_bee_suggestions(
    crack_the_bee_args: args::crack_the_bee::CrackTheBeeArgs,
    word_reader: &mut Box<dyn std::io::BufRead>,
) -> Result<Rc<Vec<String>>, std::io::Error> {
    let mut letters: [char; args::crack_the_bee::NUM_LETTERS] =
        ['a'; args::crack_the_bee::NUM_LETTERS];
    set_game_letter_array(&crack_the_bee_args.letters, &mut letters);
    print_game_letters(&letters);

    filter_words(word_reader, &letters)
  
}
