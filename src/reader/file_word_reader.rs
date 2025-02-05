#![warn(missing_docs)]

use std::fmt::Write;

// Todo: Create a function that returns a vector of File BufRead objects
// If a file has X lines, and Y readers are requested
// Each reader shall return a reader which is able to return X/Y lines
// This can be used to parellize the processing of big files.


/// Given a valid file path, returns a BufRead trait object to read it.
/// # Arguments
/// * `file_path` : Valid path to a file containing a word in every line.
/// # Returns
/// Result with the BufRead trait object or Error
pub fn create_file_word_reader(
    file_path: &str,
) -> Result<Box<dyn std::io::BufRead>, std::io::Error> {
    let word_file_result = std::fs::File::open(file_path);


    match word_file_result {
        Ok(word_file) => {
            let reader = std::io::BufReader::new(word_file);
            return Ok(Box::new(reader));
        }
        Err(e) => {
            let mut error = "Error. ".to_string();
            write!(
                &mut error,
                "{}",
                e
            )
            .unwrap_or(());
            let open_file_error =
                std::io::Error::new(std::io::ErrorKind::NotFound, "Failed to open file");
            return Err(open_file_error);
        }
    }
}
