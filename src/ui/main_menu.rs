use iced::Command;
use iced::pure::{column, text, button, container, text_input, Element};
use iced::{Length, Alignment, alignment::Horizontal};
use iced_aw::pure::{Modal, Card};
use anyhow::Result;

use super::style::TextButton;

#[derive(Default)]
pub struct MainMenu {
    show_new_macro: bool,
    pub new_macro_name: String
}

#[derive(Debug, Clone)]
pub enum MainMenuMessage {
    NewButtonClicked,
    LoadButtonClicked,
    NewMacroDismissed,
    MacroNameUpdate(String),
    NewMacroOk
}

impl MainMenu {
    pub fn update(&mut self, event: MainMenuMessage) -> Result<Command<MainMenuMessage>> {
        match event {
            MainMenuMessage::NewButtonClicked => self.show_new_macro = true,
            MainMenuMessage::LoadButtonClicked => (),
            MainMenuMessage::NewMacroDismissed => { self.show_new_macro = false; self.new_macro_name = "".to_string() },
            MainMenuMessage::MacroNameUpdate(name) => self.new_macro_name = name,
            MainMenuMessage::NewMacroOk => {    // handled both here and above
                self.new_macro_name = String::new();
                self.show_new_macro = false;
            },
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

        let content = column()
            .push(title)
            .push(buttons)
            .width(Length::Fill)
            .height(Length::Fill);

        if self.show_new_macro {
            return Modal::new(true, content, move || {
                Card::new(
                    text("New macro"), 
                    text_input(
                        "Enter macro name",
                        &self.new_macro_name,
                        MainMenuMessage::MacroNameUpdate
                    )
                    .on_submit(MainMenuMessage::NewMacroOk)
                )
                .foot(
                    container(
                        button(
                            text("Ok")
                        )
                        .on_press(MainMenuMessage::NewMacroOk)
                    )
                    .width(Length::Fill)
                    .center_x()
                )
                .max_width(300)
                .on_close(MainMenuMessage::NewMacroDismissed)
                .into()
            })
            .backdrop(MainMenuMessage::NewMacroDismissed)
            .on_esc(MainMenuMessage::NewMacroDismissed)
            .into();
        }

        content.into()
    }
}