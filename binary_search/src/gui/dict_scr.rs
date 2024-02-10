
use crate::gui::gui::{LbScreenError, LbScreen};
use crate::input_device::input_mode::InputMode;
use crate::dict::dict::*;


pub struct DictScr {
    dict: Dict,

}

impl DictScr {
    fn new(dict: Dict) -> Self {
        Self{
            dict,
        }
    }
}

impl LbScreen for DictScr {
    fn load(&self) -> Result<(), LbScreenError> {
        todo!()
    }

    fn get_default_input_mode() -> InputMode {
        InputMode::Lowercase
    }
}