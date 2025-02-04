#![warn(missing_docs)]

use reqwest::blocking::Client;

pub fn create_net_word_reader(
    url: &str,
) -> Result<Box<dyn std::io::BufRead>, std::io::Error> {
    let my_client = Client::new();
    let res = my_client.get(url).send();
    match res {
        Ok(val) => {
            let bufread = std::io::BufReader::new(val);

            return Ok(Box::new(bufread));
        }
        Err(e) => {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, e.to_string()));
        }
    }
}
