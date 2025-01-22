

use super::word_reader::WordReader;
use std::fs::File;
use std::io::{BufRead, BufReader};


pub struct FileWordReader {

    pub reader : BufReader<std::fs::File>,

}

impl FileWordReader {

    pub fn new(file_path : &str) -> Result<Self, std::io::Error>{
        let word_file_result = File::open(file_path);

        match word_file_result {
            Ok(word_file) => {
                let reader = BufReader::new(word_file);
                return Ok(Self{reader});

            }
            Err(e) => {
                let open_file_error =
                    std::io::Error::new(std::io::ErrorKind::NotFound, "Failed to open file");
                return Err(open_file_error);
            }
        }
    }
}

//impl WordReader<BufReader<File>> for FileWordReader {
//
//    fn lines(self) -> std::io::Lines<BufReader<File>> {
//        return self.reader.lines();
//    }
//
//}

impl std::io::Read for FileWordReader{

    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl BufRead for FileWordReader{

    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt);
    }

    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.reader.fill_buf()
    }

}