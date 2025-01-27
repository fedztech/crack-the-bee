use argh::FromArgs;
use std::fmt::Write;

pub static NUM_LETTERS: usize = 7;

#[derive(FromArgs)]
/// Create the data set for a spelling bee game.
pub struct GameArgs {
    #[argh(option)]
    #[argh(
        description = "if given, the source of the word list is a valid and readable file in the file system. For example /usr/share/dict/american-english-huge."
    )]
    pub file_path: Option<String>,

    #[argh(option)]
    #[argh(
        description = "if given, the source of the word list is a web address url to a publically available file that can be downloaded"
    )]
    pub url: Option<String>,
    #[argh(
        option,
        description = "the letters to use, the first letter shall be in all generated words. 7 unique letters shall be given in total "
    )]
    pub letters: String,
    #[argh(switch, short = 's', description = "to solve the spelling bee game")]
    pub spellingbee: bool,
    #[argh(switch, short = 'w', description = "to solve the wordle game")]
    pub wordle: bool,
    
}

impl GameArgs {
    pub fn validate(&self) -> Option<std::io::Error> {
        let file_path_available: bool;
        let mut url_available = false;

        if self.spellingbee == true && self.wordle == true {
            return Some(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Either spellingbee (-s) or wordle (-w) need to be selected, not both.",
            ));
        }

        if self.spellingbee == false && self.wordle == false {
            return Some(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Please select a game to play, spellingbee (-s) or wordle (-w).",
            ));
        }


        match self.file_path.clone() {
            Some(file_path) => {
                println!("Provided path: {}", file_path);
                let path = std::path::Path::new(&file_path);
                if !path.exists() {
                    return Some(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "File does not exist.",
                    ));
                }
                file_path_available = true;
            }
            None => {
                file_path_available = false;
            }
        }

        // File path has priority, so we do not check url if file path is available and valid
        if !file_path_available {
            match self.url.clone() {
                Some(url) => {
                    println!("Provided url: {}", url);
                    url_available = true;
                }
                None => {
                    url_available = false;
                }
            }
        }

        if !file_path_available && !url_available {
            return Some(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Either a file path or an url have to be provided. None were provided.",
            ));
        }

        if self.letters.len() != NUM_LETTERS {
            let mut error = "Error. ".to_string();
            write!(
                &mut error,
                "Invalid number of letters provided: {}, shall be {}",
                self.letters.len(),
                NUM_LETTERS
            )
            .unwrap_or(());
            return Some(std::io::Error::new(std::io::ErrorKind::InvalidInput, error));
        }

        // Check that the letters do not repeat.
        for letter_to_check in self.letters.as_bytes() {
            let mut count = 0;
            for letter_to_compare in self.letters.as_bytes() {
                if letter_to_check == letter_to_compare {
                    count += 1;
                }
            }
            if count > 1 {
                let mut error = "Error. ".to_string();
                let the_char: char;
                let letter_char_conversion = char::from_u32(*letter_to_check as u32);
                match letter_char_conversion {
                    Some(character) => {
                        the_char = character;
                    }
                    None => {
                        the_char = ' ';
                    }
                }
                write!(
                    &mut error,
                    "Letter: '{}', appears {} times.",
                    the_char, count
                )
                .unwrap_or(());
                return Some(std::io::Error::new(std::io::ErrorKind::InvalidInput, error));
            }
            // Check that it is only letters
            if *letter_to_check < "a".as_bytes()[0] || *letter_to_check > "z".as_bytes()[0] {
                let mut error = "Error. ".to_string();
                let the_char: char;
                let letter_char_conversion = char::from_u32(*letter_to_check as u32);
                match letter_char_conversion {
                    Some(character) => {
                        the_char = character;
                    }
                    None => {
                        the_char = ' ';
                    }
                }
                write!(
                    &mut error,
                    "Invalid letter: '{}', only a-z accepted.",
                    the_char
                )
                .unwrap_or(());
                return Some(std::io::Error::new(std::io::ErrorKind::InvalidInput, error));
            }
        }

        return None;
    }
}
