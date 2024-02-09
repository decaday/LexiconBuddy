use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::SeekFrom;
use std::io::prelude::*;

use std::error::Error;
use crate::DictTypes::Ecdict;

// Dictionary Types
enum DictTypes {
    Ecdict,
    Oxford,
}


struct Dict{
    index_file: File,
    index_file_size: u64,
    dict_type: DictTypes,
}
impl Dict{
    fn build(dict_type: DictTypes) -> Result<Dict, Box<dyn Error>>{
        let mut index_file = File::open("././index.txt")?;
        let metadata = index_file.metadata()?;
        let index_file_size = metadata.len();
        Ok(Dict{
            index_file,
            index_file_size,
            dict_type
        })
    }

    fn search_index(mut self, entry: &str) -> Result<u64, Box<dyn Error>> {
        let mut low = 0;
        let mut high = self.index_file_size;
        let mut i = 0;
        while low <= high {
            i = i + 1;
            if i > 30 {
                break
            }
            let mid = low + (high - low) / 2 + 1;
            let mut current_pos = mid;
            self.index_file.seek(SeekFrom::Start(current_pos))?;

            // Check if mid is in the middle of a line and adjust mid accordingly
            let mut ch = [0u8; 1];
            let mut pos = current_pos;
            loop {
                if pos < low {
                    break
                }
                self.index_file.read_exact(&mut ch)?;
                if ch[0] == b'\n' {
                    break;
                }
                pos -= 1;
                self.index_file.seek(SeekFrom::Start(pos))?;
            }
            current_pos = pos + 1;
            self.index_file.seek(SeekFrom::Start(current_pos))?;

            // Read the line
            let mut line = String::new();
            let mut reader = BufReader::<&File>::new(&self.index_file);
            reader.read_line(&mut line)?;
            let parts: Vec<&str> = line.split(',').collect();
            let current_word = parts[0];
            println!("current word: {current_word}");
            match current_word.cmp(&entry) {
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
}



struct EcdictWord{
    word: String,
    phonetic: String,
    translation: String,
    tag: String,
}



impl EcdictWord{
    fn build(entry: &str) -> Result<(), Box<dyn Error>>{
        Ok(())
    }
}


fn run() -> Result<u64, Box<dyn Error>>{
    let entry = "apple";
    let ecdict = Dict::build(Ecdict)?;
    let addr = ecdict.search_index(&entry)?;
    // let word = EcdictWord::build(entry)?;
    Ok(addr)
}


fn main() {

    match run() {
        Ok(addr) => {
            println!("Addresses for it: {addr}");
        },
        Err(error) => println!("Error: {error}")
    };


}
