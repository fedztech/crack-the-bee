use argh::FromArgs;
use std::fmt::Write;

/// Maximum number of letters that the spelling bee game uses.
pub static NUM_LETTERS: usize = 7;

#[derive(FromArgs, PartialEq, Debug)]
/// Crack entry command
pub struct CrackArgs {
    #[argh(subcommand)]
    pub game: GameSubcommands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum GameSubcommands {
    Bee(BeeSubcommandArgs),
    Word(WordSubcommandArgs),
}

pub trait PossiblePaths {
    fn url(&self) -> Option<String>;
    fn file_path(&self) -> Option<String>;
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(
    subcommand,
    name = "bee",
    description = "Generates possible word list for the Bee game."
)]
pub struct BeeSubcommandArgs {
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
}

impl PossiblePaths for BeeSubcommandArgs {
    fn url(&self) -> Option<String> {
        self.url.clone()
    }

    fn file_path(&self) -> Option<String> {
        self.file_path.clone()
    }
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(
    subcommand,
    name = "word",
    description = "Helps to solve the word game."
)]
pub struct WordSubcommandArgs {
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
}

impl PossiblePaths for WordSubcommandArgs {
    fn url(&self) -> Option<String> {
        self.url.clone()
    }

    fn file_path(&self) -> Option<String> {
        self.file_path.clone()
    }
}

fn validate_paths<T>(args_to_validate: &T) -> Option<std::io::Error>
where
    T: PossiblePaths,
{
    let file_path_available: bool;
    let mut url_available = false;

    match args_to_validate.file_path() {
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
        match args_to_validate.url() {
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

    return None;
}

impl BeeSubcommandArgs {
    pub fn validate(&self) -> Option<std::io::Error> {
        if let Some(error) = validate_paths(self) {
            return Some(error);
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

impl WordSubcommandArgs {
    pub fn validate(&self) -> Option<std::io::Error> {
        return validate_paths(self);
    }
}
