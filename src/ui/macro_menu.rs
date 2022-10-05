use iced::Length;
use iced::pure::{Element, text, button, row, container};

use crate::style::{PlusButton, BorderedContainer};

use super::{BaseMessage, MainMenuMessage};

pub struct MacroMenu {}

impl MacroMenu {
    pub fn new() -> MacroMenu {
        MacroMenu {}
    }

    pub fn view(&self) -> Element<BaseMessage> {
        row()
        .push(
            container(
                text("yup")
            )
            .width(Length::FillPortion(1))
        )
        .push(
            container(
                button(
                    text("+").size(24)
                )
                .style(PlusButton::Normal)
                .on_press(BaseMessage::MainMenuMessage(MainMenuMessage::LoadButtonClicked))
                .width(Length::Shrink)
                .height(Length::Shrink)
            )
            .width(Length::FillPortion(6))
            .height(Length::Fill)
            .style(BorderedContainer::Nothing)
        ).push(
            container(
                text("lol")
            )
            .width(Length::FillPortion(2))
        ).height(Length::Fill)
        .into()

    }
}