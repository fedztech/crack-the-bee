//! Provides the logic needed to generate a list for the bee game.
//!
use crate::args;
use regex::Regex;
use std::io::BufRead;
use std::rc::Rc;

#[cfg(test)]
mod tests {
    use std::cmp::min_by;

    use super::*;

    #[test]
    fn test_printing_game_letters() {
        // Given array of 7 letters
        let letter_array: [char; args::game::NUM_LETTERS] = ['a'; 7];
        // When the print_game_letters function is called upon it.
        print_game_letters(&letter_array);
        // Then The letters are printed in the console

        // This should not crash. So we leave this as it is.
    }

    #[test]
    fn test_set_game_letters_ok() {
        // Given
        let letters_to_set: String = "abcdefg".to_string();
        let mut letter_array: [char; args::game::NUM_LETTERS] = [' '; 7];
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
        let mut letter_array: [char; args::game::NUM_LETTERS] = [' '; 7];
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
        let mut letter_array: [char; args::game::NUM_LETTERS] = [' '; 7];
        // When
        let result = set_game_letter_array(&letters_to_set, &mut letter_array);
        // Then
        assert_eq!(result.is_some(), true);
        if let Some(error) = result {
            assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
        }
    }

    static WORDS: [&'static str; 20] = [
        "Alice", "Bob", "Charlie", "one", "two", "Alice", "Bob", "Charlie", "one", "two", "Alice",
        "Bob", "Charlie", "one", "two", "Alice", "Bob", "Charlie", "one", "two",
    ];

    static DATA: &'static str =
        "academia\nacid\nboat\ncar\ncyamid\ndalmatian\ndecayed\nimmediacy\nzen\n";

    struct TwentyWordReaderMock {
        // We need an internal buffer of some 20 characters
        internal_buffer: [u8; 20],
        // The position of the data read.
        read_index: usize,
        // How many bytes of the buffer contain valid data
        internal_buffer_length: usize,
    }

    impl TwentyWordReaderMock {
        fn new() -> Self {
            TwentyWordReaderMock {
                internal_buffer: [0; 20],
                read_index: 0,
                internal_buffer_length: 0,
            }
        }
    }

    impl std::io::Read for TwentyWordReaderMock {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            // No need to implement
            Ok(0)

            // // Low level read
            // let internal_buf_res = self.fill_buf();
            //
            // // Copy the data
            // // How much data to copy? The minimum between buffer and what was given
            // let internal_buf = internal_buf_res.unwrap();
            // println!("read fill_buf: {:?}", internal_buf);
            // let min_data_to_copy = min_by(internal_buf.len(), buf.len(), |a,b|{a.cmp(b)});
            // for ix in 0..min_data_to_copy {
            //     buf[ix] = internal_buf[ix].clone();
            // }
            // println!("read consume {}", min_data_to_copy);
            // self.consume(min_data_to_copy);
            // Ok(min_data_to_copy)
        }
    }

    impl std::io::BufRead for TwentyWordReaderMock {
        fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
            // Fill the buffer with the next chars, if there is space available
            let mut new_internal_buffer_length = self.internal_buffer_length;
            for ix in self.internal_buffer_length..self.internal_buffer.len() {
                if self.read_index < DATA.len() {
                    self.internal_buffer[ix] = DATA.as_bytes()[self.read_index].clone();
                    self.read_index += 1;
                    new_internal_buffer_length += 1;
                }
            }

            // Contract requires to return empty slice if EOF is found, and the buffer is consumed.
            if new_internal_buffer_length == self.internal_buffer_length
                && self.read_index == DATA.len()
                && self.internal_buffer_length == 0
            {
                return Ok(&[]);
            }

            self.internal_buffer_length = new_internal_buffer_length;

            Ok(&self.internal_buffer[0..self.internal_buffer_length])
        }

        fn consume(&mut self, amount: usize) {
            // The amount has to be smaller than the length returned by the fill_buf
            // and fill_buff always returns internal_buffer_length as length
            if amount < self.internal_buffer_length {
                // Shift all the data amount of bytes to the left (to place it starting zero)
                let mut index_to = 0;
                for index_from in amount..self.internal_buffer.len() {
                    self.internal_buffer[index_to] = self.internal_buffer[index_from];
                    index_to += 1;
                }
                // The lenght of the buffer is reduced accordingly
                self.internal_buffer_length -= amount;
            }
        }
    }

    #[test]
    fn test_filter_words() {
        // Given a BufReader that returns a list of 20 words, and the letters cadeimy
        // The BufReader is a mock that deliver 20 words, some can be used for the letters given, others not.
        let letters_to_play: String = "cadeimy".to_string();
        let mut letter_array: [char; args::game::NUM_LETTERS] = [' '; args::game::NUM_LETTERS];
        drop(set_game_letter_array(&letters_to_play, &mut letter_array));
        // When the filter_words is called
        let mut reader = TwentyWordReaderMock::new();
        let result = filter_words(&mut reader, &letter_array);
        // the words that match (from the 20 words) shall be returned.
        assert!(result.is_ok());
        if let Ok(word_list) = result {
            assert!(word_list.len() > 0);
            for word in &*word_list {
                println!("{}", word);
            }
            assert!(word_list.len() == 5);
        }
    }
}

// Given an array of 7 letters, prints information about them in the console.
fn print_game_letters(letters: &[char; args::game::NUM_LETTERS]) {
    println!("Letters captured.");
    println!("Main letter: {}", letters[0].to_string());
    for letter_index in 1..letters.len() {
        println!("Letter {} = {}", letter_index, letters[letter_index]);
    }
}

// Given a String with 7 lowercase letters (a-z), sets the letter arraz
// * Return
//   An error is returned if the operation was not successful.
//   InvalidInput error returned if the lenght of the String is not ok
//   InvalidData error returned if the string contains invalid characters
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

        // Conversion should always be ok as we checked the range before.
        if let Some(converted_letter) = char::from_u32(the_utf8_char as u32) {
            letter_array[letter_ix] = converted_letter;
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
        .for_each(|line_res| {
            if let Ok(line) = line_res {
                word_list.push(line);
            }
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
