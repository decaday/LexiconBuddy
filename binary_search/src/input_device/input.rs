use crate::app::screen;

pub enum InputMode {
    Disable,
    Lowercase,
    Uppercase,
    NumberAndSymbols,
}

struct Input{
    cur_input_mode: InputMode,
    cur_screen: Option<Box<dyn screen::LbScreen>>,
}

impl Input {
    pub fn new() -> Self{
        Self{
            cur_input_mode : InputMode::Disable,
            cur_screen: None,
        }
    }

    pub fn set_screen(mut self, screen_box: Box<dyn screen::LbScreen>){
        self.cur_input_mode = screen_box.as_ref().get_default_input_mode();
        self.cur_screen = Some(screen_box);
    }

    pub fn device_input(self, id: u8){
        let ch:char = '1';
        let cur_screen = self.cur_screen.as_ref().expect("NO currnet screen");
        cur_screen.input(ch)
    }
}


