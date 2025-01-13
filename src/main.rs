use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::rc::Rc;

static NUM_LETTERS: usize = 7;

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

fn capture_game_letters(letters: &mut [char; NUM_LETTERS]) -> Result<usize, std::io::Error> {
    for letter_position in 0..NUM_LETTERS {
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
                    println!(
                        "Failed to capture letter {}, {}. Try again.",
                        letter_position, e
                    );
                }
            }
        }
    }
    Ok(NUM_LETTERS)
}

fn print_game_letters(letters: &[char; NUM_LETTERS]) {
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
    letters: &[char; NUM_LETTERS],
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

    // Capture letters
    let mut letters: [char; NUM_LETTERS] = ['a'; NUM_LETTERS];
    let capture_result = capture_game_letters(&mut letters);
    match capture_result {
        Ok(num_letters) => {
            print_game_letters(&letters);
        }
        Err(e) => {
            println!("Failed to capture the letters: {}", e.to_string());
            std::process::exit(1);
        }
    }

    // Get word reader
    let mut word_reader_result = get_word_reader_for_file("/usr/share/dict/american-english-huge");
    match &mut word_reader_result {
        Ok(_) => {
            // Ok, we will proceed below to avoid too much nesting of pattern matchings.
        },
        Err(e) => {
            println!("Failed to capture the letters: {}", e.to_string());
            std::process::exit(2);
        }
    }

    // Filter words
    let word_reader = word_reader_result.as_mut().unwrap();
    let mut words_result = filter_words(word_reader, &letters);
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
    std::process::exit(0);
}
