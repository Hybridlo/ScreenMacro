use iced::Command;
use iced::pure::{column, text, button, container, Element};
use iced::{Length, Alignment, alignment::Horizontal};
use anyhow::Result;

use super::style::TextButton;

#[derive(Default)]
pub struct MainMenu {}

#[derive(Debug, Clone)]
pub enum MainMenuMessage {
    NewButtonClicked,
    LoadButtonClicked
}

impl MainMenu {
    pub fn update(&mut self, event: MainMenuMessage) -> Result<Command<MainMenuMessage>> {
        match event {
            MainMenuMessage::NewButtonClicked => (),    // handled both here and above
            MainMenuMessage::LoadButtonClicked => (),
        }

        Ok(Command::none())
    }

    pub fn view(&self) -> Element<MainMenuMessage> {
        let title = container(
            column()
                .push(
                    container(
                        text("Screen")
                            .size(48)
                    )
                    .height(Length::Shrink)
                    .width(Length::Fill)
                    .align_x(Horizontal::Left)
                    .center_y()
                )

                .push(
                    container(
                        text("Macro")
                            .size(48)
                    )
                    .height(Length::Shrink)
                    .width(Length::Fill)
                    .align_x(Horizontal::Right)
                    .center_y()
                )
                
                .height(Length::Units(128))
                .width(Length::Units(168))
        )
        .height(Length::FillPortion(5))
        .width(Length::Fill)
        .center_x()
        .center_y();

        let buttons = container(
            column()
                .push(
                    button(
                        text("New macro")
                            .size(28)
                    )
                    .style(TextButton::Normal)
                    .on_press(MainMenuMessage::NewButtonClicked)
                )

                .push(
                    button(
                            text("Load macro")
                            .size(28)
                    )
                    .style(TextButton::Normal)
                    .on_press(MainMenuMessage::LoadButtonClicked)
                )

                .height(Length::Shrink)
                .align_items(Alignment::Center)
                .spacing(12)
        )
        .height(Length::FillPortion(3))
        .width(Length::Fill)
        .center_x()
        .center_y();

        column()
        .push(title)
        .push(buttons)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}