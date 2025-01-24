use std::fmt::Write;


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
