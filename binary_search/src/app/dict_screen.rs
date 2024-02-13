use crate::app::screen::{LbScreen, LbScreenError};
use crate::gui::dict_scr;
use crate::gui::dict_scr::*;
use crate::input_device::input::InputMode;

use crate::logic::dict::dict;

#[cfg(feature = "win_sim")]
const ECDICT_INDEX_PATH: &str = "././index.txt";

struct DictScreen{
    current_input_string: String,
    dict: dict::Dict,
}

impl DictScreen{
    fn build() -> Result<Self, LbScreenError> {
        let ecdict = dict::Dict::build(dict::DictTypes::Ecdict,
                                       ECDICT_INDEX_PATH)?;
        Ok(Self{
            dict: ecdict,
            current_input_string:String::new(),
        })
    }
}

impl LbScreen for DictScreen{
    fn init(&self) -> Result<(), LbScreenError> {

        Ok(())

        //let dict_scr = DictScr::new(ecdict);
        //let entry = "appendix";

        //let addr = ecdict.search_index(&entry)?;
        // let word = EcdictWord::build(entry)?;
        //Ok(addr)
    }

    fn input(&self, ch: char) {
        todo!()
    }

    fn click_ok(&mut self) {
        let entry = "appendix";
        
        match self.dict.search_index(&entry) {
            Ok(addr) => {
                println!("Addresses for it: {addr}");
            }
            Err(error) => match error {
                dict::DictError::IOError(e) => println!("IO error! {e}"),
                dict::DictError::ParseError(e) => println!("ParseError {e}"),
                dict::DictError::ParseIntError(e) => println!("ParseError {e}"),
                dict::DictError::WordNotFound => println!("Word not fond"),
            },
        };

    }

    fn get_default_input_mode(&self) -> InputMode {
        InputMode::Lowercase
    }
}