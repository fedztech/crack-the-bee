use crate::args;
use regex::Regex;
use std::io::BufRead;
use std::rc::Rc;
use crate::games::word_state::WordState;


fn filter_words<T>(reader: &mut T, state: &mut WordState) -> Result<Rc<Vec<String>>, std::io::Error>
where
    T: BufRead,
{
    let mut word_list: Vec<String> = Vec::new();

    // First word: adieu

    let empty_string: String = String::new();

    println!("Try the word 'adieu'");

    let mut not_present_letters = String::new();
    let mut present_letters = String::new();

    word_list.clear();

    println!("Enter the letters not in the word.");
    not_present_letters.clear();
    std::io::stdin()
        .read_line(&mut not_present_letters)
        .expect("Failed to read letters no in the word.");
    for letter_position in &mut state.letter_positions {
        letter_position.letters_not_to_use += &not_present_letters.trim();
    }

    println!("Enter the letters that are in the word (comma separated), but we do not know the position, or leave empty.");
    let mut available_letters = String::new();
    std::io::stdin()
        .read_line(&mut available_letters)
        .expect("Failed to read letters no in the word.");
    if available_letters.trim().len() > 0 {
        for letter in available_letters.trim().split(",") {
            present_letters += &"|".to_string();
            present_letters += letter;
        }
    }

    for letter_position in &mut state.letter_positions {
        if letter_position.found_letter {
            continue;
        }
        println!(
            "Is the letter in position {} known (y/n)?",
            letter_position.position
        );
        let mut answer = String::new();
        std::io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read letter in position.");
        if answer.starts_with("y") {
            println!(
                "What is the letter in position {}?",
                letter_position.position
            );
            let mut present_letter = String::new();
            std::io::stdin()
                .read_line(&mut present_letter)
                .expect("Failed to read letter in position.");
            // Todo: check input to be valid.
            letter_position.found_letter = true;
            letter_position.letter_to_use += &present_letter.trim();
        }
    }

    let mut regular_expression = r"^".to_string();
    for letter_position in &mut state.letter_positions {
        if letter_position.found_letter {
            regular_expression += &letter_position.letter_to_use;
        } else {
            regular_expression += "([^";
            regular_expression += &letter_position.letters_not_to_use;
            regular_expression += "]";
            regular_expression += &present_letters;
            regular_expression += "{1})"
        }
    }
    regular_expression += "$";

    println!("The regex {}", regular_expression);

    reader
        .lines()
        .filter(|line| line.as_ref().unwrap_or(&empty_string).len() == 5)
        .filter(|line| {
            let re = Regex::new(regular_expression.as_str()).unwrap();
            return re.is_match(line.as_ref().unwrap_or(&empty_string));
        })
        .for_each(|line| {
            word_list.push(line.expect("blah"));
        });

    let mut count = 10;
    for value in word_list.iter() {
        if count > 0 {
            print!("{}     ", value);
            count = count - 1;
        } else {
            println!("{}", value);
            count = 10;
        }
    }

    return Ok(Rc::new(word_list));
}

pub fn get_wordle_suggestions(game_args: args::game::WordSubcommandArgs) {
    let mut word_state = WordState::new();

    for _ in 1..6 {
        let word_reader_result: Option<Box<dyn std::io::BufRead>> =
            crate::reader::factory::get_word_dictionary_reader(&game_args);

        match word_reader_result {
            Some(mut word_reader) => {
                // Print all the words with exactly 5 letters

                let res = filter_words(&mut word_reader, &mut word_state);
                match res {
                    Ok(words) => {
                        //for value in words.iter() {
                        //    println!("{}", value);
                        //}
                    }
                    Err(e) => {
                        // Todo: propagate
                        println!("{}", e);
                        std::process::exit(3);
                    }
                }
            },
            None => {
                // Todo: propagate
                std::process::exit(4);
            }
        }
    }
}
