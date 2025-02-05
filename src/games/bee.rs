//! Provides the logic needed to generate a list for the bee game.
//!
use crate::args;
use regex::Regex;
use std::io::BufRead;
use std::rc::Rc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printing_game_letters() {}

    #[test]
    fn test_set_game_letters_ok() {
        // Given
        let letters_to_set: String = "abcdefg".to_string();
        let mut letter_array: [char; args::game::NUM_LETTERS] = ['a'; 7];
        // When
        let result = set_game_letter_array(&letters_to_set, &mut letter_array);
        // Then
        assert_eq!(result.is_none(), true);
        assert_eq!(letter_array[0], 'a');
        assert_eq!(letter_array[1], 'b');
        assert_eq!(letter_array[2], 'c');
        assert_eq!(letter_array[3], 'd');
        assert_eq!(letter_array[4], 'e');
        assert_eq!(letter_array[5], 'f');
        assert_eq!(letter_array[6], 'g');
    }

    #[test]
    fn test_set_game_letters_smaller_string() {
        // Given
        let letters_to_set: String = "abcdef".to_string();
        let mut letter_array: [char; args::game::NUM_LETTERS] = ['a'; 7];
        // When
        let result = set_game_letter_array(&letters_to_set, &mut letter_array);
        // Then
        assert_eq!(result.is_some(), true);
        if let Some(error) = result {
            assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
        }
    }

    #[test]
    fn test_set_game_letters_unsupported_char() {
        // Given
        let letters_to_set: String = "abcdef-".to_string();
        let mut letter_array: [char; args::game::NUM_LETTERS] = ['a'; 7];
        // When
        let result = set_game_letter_array(&letters_to_set, &mut letter_array);
        // Then
        assert_eq!(result.is_some(), true);
        if let Some(error) = result {
            assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
        }
    }
}

fn print_game_letters(letters: &[char; args::game::NUM_LETTERS]) {
    println!("Letters captured.");
    println!("Main letter: {}", letters[0].to_string());
    for letter_index in 1..letters.len() {
        println!("Letter {} = {}", letter_index, letters[letter_index]);
    }
}

fn set_game_letter_array(
    letter_string: &String,
    letter_array: &mut [char; args::game::NUM_LETTERS],
) -> Option<std::io::Error> {
    if letter_string.len() != args::game::NUM_LETTERS {
        return Some(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid length of letter_string.",
        ));
    }

    for letter_ix in 0..letter_array.len() {
        // Only ASCII lowercase a-z letters are allowed
        let the_utf8_char = letter_string.as_bytes()[letter_ix];
        if the_utf8_char < 97 || the_utf8_char > 122 {
            return Some(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Input letters are not supported.",
            ));
        }

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

    None
}

fn filter_words<T>(
    reader: &mut T,
    letters: &[char; args::game::NUM_LETTERS],
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
    game_args: args::game::BeeSubcommandArgs,
    word_reader: &mut Box<dyn std::io::BufRead>,
) -> Result<Rc<Vec<String>>, std::io::Error> {
    let mut letters: [char; args::game::NUM_LETTERS] = ['a'; args::game::NUM_LETTERS];
    set_game_letter_array(&game_args.letters, &mut letters);
    print_game_letters(&letters);

    filter_words(word_reader, &letters)
}
