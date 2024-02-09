use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::SeekFrom;
use std::io::prelude::*;

use std::error::Error;

fn binary_search_word(file: &mut File, target_word: &str, file_size: u64) -> Result<u64, Box<dyn Error>> {
    let mut low = 0;
    let mut high = file_size;
    let mut i = 0;
    while low <= high {
        i = i + 1;
        if i > 30 {
            break
        }
        let mid = low + (high - low) / 2 + 1;
        let mut current_pos = mid;
        file.seek(SeekFrom::Start(current_pos))?;

        // Check if mid is in the middle of a line and adjust mid accordingly
        let mut ch = [0u8; 1];
        file.read_exact(&mut ch)?;
        if ch[0] != b'\n' {
            let mut pos = current_pos;
            while pos > 0 {
                pos -= 1;
                file.seek(SeekFrom::Start(pos))?;
                file.read_exact(&mut ch)?;
                if ch[0] == b'\n' {
                    break;
                }
            }
            current_pos = pos + 1;
        }
        file.seek(SeekFrom::Start(current_pos))?;

        // Read the line
        let mut line = String::new();
        let mut reader = BufReader::<&File>::new(file);
        reader.read_line(&mut line)?;
        let parts: Vec<&str> = line.split(',').collect();
        let current_word = parts[0];
        println!("current word: {current_word}");
        match current_word.cmp(&target_word) {
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid - 1,
            std::cmp::Ordering::Equal => {
                let addr = parts[1].trim().parse()?;
                return Ok(addr);
            }
        }
    }
    Err("Target word not found in file".into())
}
struct EcdictWord{
    word: String,
    phonetic: String,
    translation: String,
    tag: String
}


fn main() {
    let mut file = File::open("././index.txt").unwrap();
    let metadata = file.metadata().unwrap();
    let file_size = metadata.len();

    let word_to_search = "bad";
    match binary_search_word(&mut file, word_to_search, file_size) {
        Ok(addr) => {
            println!("Addresses for {word_to_search}: {addr}");
        },
        Err(error) => println!("An error occurred: {}", error),
    };
}
