use crate::input_device::input_mode::InputMode;

#[derive(thiserror::Error, Debug)]
pub enum LbScreenError {

}

// pub struct LbScreen {
//     input_types:u8,
// }

pub trait LbScreen{
    fn load(&self) -> Result<(), LbScreenError>;

    fn get_default_input_mode() ->InputMode;

}