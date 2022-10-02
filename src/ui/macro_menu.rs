use iced::pure::{Element, text};

use super::BaseMessage;

pub struct MacroMenu {}

impl MacroMenu {
    pub fn new() -> MacroMenu {
        MacroMenu {}
    }

    pub fn view(&self) -> Element<BaseMessage> {
        text("Hi").into()
    }
}