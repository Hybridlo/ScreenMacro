use iced::Length;
use iced::pure::{Element, text, button, row, container, column};

use crate::macro_logic::MacroStep;

use super::components::macro_step_component;
use super::style::{PlusButton, BorderedContainer};

#[derive(Debug, Clone)]
pub enum MacroMenuMessage {
    NewVal(MacroStep, usize)
}

pub struct MacroMenu {
    macro_step: MacroStep
}

impl MacroMenu {
    pub fn new() -> MacroMenu {
        MacroMenu { macro_step: Default::default() }
    }

    pub fn update(&mut self, msg: MacroMenuMessage) {
        println!("{:?}", msg);
        match msg {
            MacroMenuMessage::NewVal(val, index) => self.macro_step = val,
        }
    }

    pub fn view(&self) -> Element<MacroMenuMessage> {
        let macro_container = container(
            column()
                .push(
                    button(
                        text("+").size(24)
                    )
                    .style(PlusButton::Normal)
                    .width(Length::Shrink)
                    .height(Length::Shrink)
                )
                .push(
                    macro_step_component(0, Some(self.macro_step.clone()), MacroMenuMessage::NewVal)
                )
        )
        .width(Length::FillPortion(6))
        .height(Length::Fill)
        .style(BorderedContainer::Nothing);



        row()
        .push(
            container(
                text("yup")
            )
            .width(Length::FillPortion(1))
        )
        .push(
           macro_container 
        ).push(
            container(
                text("lol")
            )
            .width(Length::FillPortion(2))
        ).height(Length::Fill)
        .into()

    }
}