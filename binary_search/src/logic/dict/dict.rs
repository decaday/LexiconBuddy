use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::{BufRead, BufReader};

/** Dictionary types
Ecdict, Oxford
 */
pub enum DictTypes {
    Ecdict,
    Oxford,
}

#[derive(thiserror::Error, Debug)]
pub enum DictError {
    #[error("Word not found")]
    WordNotFound,
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("Parse error in `{0}`")]
    ParseError(&'static str),
    #[error("Parse int error")]
    ParseIntError(#[from] std::num::ParseIntError),
}

/** Dict
 */
pub struct Dict {
    index_file: File,
    index_file_size: u64,
    dict_type: DictTypes,
}
impl Dict {
    pub fn build(dict_type: DictTypes) -> Result<Dict, DictError> {
        let mut index_file = File::open("././index.txt")?;
        let metadata = index_file.metadata()?;
        let index_file_size = metadata.len();
        Ok(Dict {
            index_file,
            index_file_size,
            dict_type,
        })
    }

    /** Use binary search to find the address of the word in the index.
     */
    pub fn search_index(mut self, entry: &str) -> Result<u64, DictError> {
        let mut low = 0;
        let mut high = self.index_file_size;
        let mut i = 0;
        let mut last_pos: u64 = 0;

        while low <= high {
            i = i + 1;
            if i > 30 {
                break;
            }

            //Calculate the middle location address
            let mid = low + (high - low) / 2 + 1;
            let mut current_pos = mid;
            self.index_file.seek(SeekFrom::Start(current_pos))?;

            // Check if mid is in the middle of a line and adjust mid accordingly
            let mut ch = [0u8; 1];
            let mut pos = current_pos;
            loop {
                if pos < low {
                    break;
                }
                self.index_file.read_exact(&mut ch)?;
                if ch[0] == b'\n' {
                    break;
                }
                pos -= 1;
                self.index_file.seek(SeekFrom::Start(pos))?;
            }
            current_pos = pos + 1;
            if last_pos == current_pos {
                //enters a dead end, indicating that the target word does not exist
                break;
            }
            last_pos = current_pos;
            self.index_file.seek(SeekFrom::Start(current_pos))?;

            // Read the line
            let mut line = String::new();
            let mut reader = BufReader::<&File>::new(&self.index_file);
            reader.read_line(&mut line)?;
            let mut parts = line.split(',');

            let current_word = match parts.next() {
                Some(part) => part,
                None => return Err(DictError::ParseError("part 1")),
            };
            println!("current word: {current_word}");

            match current_word.cmp(&entry) {
                std::cmp::Ordering::Less => low = mid + 1,
                std::cmp::Ordering::Greater => high = mid - 1,
                std::cmp::Ordering::Equal => {
                    let addr = match parts.next() {
                        Some(part) => part,
                        None => return Err(DictError::ParseError("part 2")),
                    }
                    .trim()
                    .parse()?;
                    return Ok(addr);
                }
            }
        }
        Err(DictError::WordNotFound)
    }
}
