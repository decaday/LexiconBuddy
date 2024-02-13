use crate::input_device::input::InputMode;

#[derive(thiserror::Error, Debug)]
pub enum LbScreenError {}

// pub struct LbScreen {
//     input_types:u8,
// }

pub trait LbScreen {
    #[cfg(feature = "win_sim")]
    fn load(&self) -> Result<(), LbScreenError>
    where
        Self: std::fmt::Display,
    {
        println!("Load {self}");
        Ok(())
    }

    #[cfg(feature = "embedded")]
    fn load(&self) -> Result<(), LbScreenError>;

    fn get_default_input_mode() -> InputMode;
}
