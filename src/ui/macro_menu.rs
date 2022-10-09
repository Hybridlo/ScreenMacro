use iced::Length;
use iced::pure::{Element, text, button, row, container, column, scrollable};

use crate::macro_logic::{Macro, MacroStep};

use super::components::macro_step_component;
use super::style::{PlusButton, BorderedContainer};

#[derive(Debug, Clone)]
pub enum MacroMenuMessage {
    NewVal(MacroStep, usize),
    Removed(usize),
    Add
}

pub struct MacroMenu {
    macro_data: Macro
}

impl MacroMenu {
    pub fn new() -> MacroMenu {
        MacroMenu { macro_data: Default::default() }
    }

    pub fn update(&mut self, msg: MacroMenuMessage) {
        match msg {
            MacroMenuMessage::NewVal(val, index) => _ = self.macro_data.macro_steps.splice(index..index+1, [val]),
            MacroMenuMessage::Removed(index) => _ = self.macro_data.macro_steps.remove(index),
            MacroMenuMessage::Add => self.macro_data.macro_steps.push(Default::default())
        }
    }

    pub fn view(&self) -> Element<MacroMenuMessage> {
        let mut macro_ui = column();

        for (i, macro_step) in self.macro_data.macro_steps.iter().enumerate() {
            macro_ui = macro_ui.push(
                macro_step_component(
                    i,
                    Some(macro_step.clone()),
                    MacroMenuMessage::NewVal,
                    MacroMenuMessage::Removed
                )
            )
        }

        let macro_container = container(scrollable(
            container(
                macro_ui
                    .push(
                        button(
                            text("+").size(24)
                        )
                        .style(PlusButton::Normal)
                        .width(Length::Shrink)
                        .height(Length::Shrink)
                        .on_press(MacroMenuMessage::Add)
                    )
            )
            .style(BorderedContainer::Nothing)
            .padding(15)
            .width(Length::Fill)
        ).scrollbar_margin(4))
        .width(Length::FillPortion(6))
        .height(Length::Fill);



        row()
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