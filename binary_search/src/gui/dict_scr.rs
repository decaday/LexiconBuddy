use crate::logic::dict::dict::*;
use crate::gui::gui::{LbScreen, LbScreenError};
use crate::input_device::input_mode::InputMode;

pub struct DictScr {
    dict: Dict,
}

impl DictScr {
    fn new(dict: Dict) -> Self {
        Self { dict }
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
