pub mod dict;
pub mod gui;
pub mod input_device;

pub mod lexicon_buddy{
    use crate::dict::dict;

    pub fn run() -> Result<u64, dict::DictError> {
        let entry = "apple";
        let ecdict = dict::Dict::build(dict::DictTypes::Ecdict)?;
        let addr = ecdict.search_index(&entry)?;
        // let word = EcdictWord::build(entry)?;
        Ok(addr)
    }
}