use crate::input_device::input::InputMode;

#[derive(thiserror::Error, Debug)]
pub enum LbScreenError {
    #[error(transparent)]
    DictError(#[from] crate::logic::dict::dict::DictError),
}

pub trait LbScreen {

    //#[cfg(feature = "win_sim")]
    //#[cfg(feature = "embedded")]
    fn init(&self) -> Result<(), LbScreenError>;

    fn input(&self, ch: char);

    fn click_ok(&mut self) {
        ()
    }

    fn get_default_input_mode(&self) -> InputMode;
}