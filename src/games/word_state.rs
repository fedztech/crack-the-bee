

pub struct LetterPosition {
    pub position: usize,
    pub found_letter: bool,
    pub letter_to_use: String,
    pub letters_not_to_use: String,
}

pub struct WordState {
    pub letter_positions: [LetterPosition; 5],
    pub present_letters: String,
    pub not_present_letters: String,
}

impl WordState {
    pub fn new() -> Self {
        WordState {
            letter_positions: core::array::from_fn(|index| LetterPosition {
                position: index + 1,
                found_letter: false,
                letter_to_use: "".to_string(),
                letters_not_to_use: "".to_string(),
            }),
            present_letters: "".to_string(),
            not_present_letters: "".to_string(),
        }
    }
}