use std::io::{self, Error};
use std::process::exit;

fn capture_and_validate_one_letter() -> Result<String, std::io::Error> {
    println!("Please input 1 ascii letter between with range 'a' to 'z'.");
    let mut letter_read = String::new();

    let res_read_op = io::stdin().read_line(&mut letter_read);
    match res_read_op {
        Ok(num_bytes_read) => {
            if letter_read.trim().len() != 1 {
                let invalid_size_error = std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "The input shall be only 1 character from a to z.",
                );
                return Err(invalid_size_error);
            }
            if letter_read.to_ascii_lowercase().as_bytes()[0] < "a".as_bytes()[0]
                || letter_read.to_ascii_lowercase().as_bytes()[0] > "z".as_bytes()[0]
            {
                let invalid_range_error = std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "The input shall be only contain characters from a to z.",
                );
                return Err(invalid_range_error);
            }
        }
        Err(e) => {
            return Err(e);
        }
    }

    return Ok(letter_read.clone());
}

fn capture_game_letters(letters: &mut [char; 7]) -> Result<usize, std::io::Error> {
    for letter_position in 0..7 {
        let mut letter_captured_correctly: bool = false;
        while false == letter_captured_correctly {
            letter_captured_correctly = true;
            let capture_result = capture_and_validate_one_letter();
            match capture_result {
                Ok(letter) => {
                    let the_letter: char = letter
                        .trim()
                        .to_string()
                        .pop()
                        .expect("Already validated.")
                        .to_ascii_lowercase();
                    //Check that the letter is not already inserted.
                    if letter_position > 0 {
                        for duplicate_check_position in 0..letter_position - 1 {
                            if letters[duplicate_check_position] == the_letter {
                                println!(
                                    "Input letter {} is a duplicate of position {}. Try again.",
                                    the_letter, duplicate_check_position
                                );
                                letter_captured_correctly = false;
                            }
                        }
                    }
                    if letter_captured_correctly == true {
                        letters[letter_position] = the_letter;
                    }
                }
                Err(e) => {
                    letter_captured_correctly = false;
                    println!("Failed to capture letter {}, {}. Try again.", letter_position, e);
                }
            }
        }
    }
    Ok(7)
}

fn print_game_letters(letters: &[char; 7]) {
    println!("Letters captured.");
    println!("Main letter: {}", letters[0].to_string());
    for letter_index in 1..letters.len() {
        println!("Letter {} = {}", letter_index, letters[letter_index]);
    }
}

fn main() {
    println!("crack-the-bee");

    let mut letters: [char; 7] = ['a'; 7];
    capture_game_letters(&mut letters);
    print_game_letters(&letters);
}
