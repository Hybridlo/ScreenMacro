use iced::pure::{Application, container, text, Element};
use iced::{Length, executor, Command};
use iced_aw::pure::{Card, Modal};
use iced_pure::button;
use rfd::FileDialog;

use crate::macro_logic::Macro;

use super::{MacroMenu, MacroMenuMessage, MainMenu, MainMenuMessage};


pub struct Base {
    selected: WindowShowing,
    curr_error: Option<String>,

    macro_menu: MacroMenu,
    main_menu: MainMenu
}

enum WindowShowing {
    Start,
    MacroMenu
}

#[derive(Debug, Clone)]
pub enum BaseMessage {
    DismissError,
    MacroMessage(MacroMenuMessage),
    MainMessage(MainMenuMessage)
}

impl Application for Base {
    type Executor = executor::Default;
    type Flags = ();
    type Message = BaseMessage;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Base {
            selected: WindowShowing::Start,
            curr_error: Default::default(),
            macro_menu: Default::default(),
            main_menu: Default::default()
        }, Command::none())
    }

    fn title(&self) -> String {
        "ScreenMacro".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            BaseMessage::DismissError => self.curr_error = None,
            BaseMessage::MainMessage(MainMenuMessage::NewButtonClicked) => {
                self.selected = WindowShowing::MacroMenu;

                match self.main_menu.update(MainMenuMessage::NewButtonClicked) {
                    Ok(com) => return com.map(BaseMessage::MainMessage),
                    Err(err) => self.curr_error = Some(err.to_string()),
                }
            },
            BaseMessage::MainMessage(MainMenuMessage::LoadButtonClicked) => {
                self.selected = WindowShowing::MacroMenu;

                if let Some(path) = FileDialog::new().add_filter("ScreenMacro binary file", &["smbf"]).pick_file() {
                    match Macro::load_file(&path) {
                        Ok(macro_data) => self.macro_menu.macro_data = macro_data,
                        Err(err) => self.curr_error = Some(err.to_string()),
                    }
                }

                match self.main_menu.update(MainMenuMessage::LoadButtonClicked) {
                    Ok(com) => return com.map(BaseMessage::MainMessage),
                    Err(err) => self.curr_error = Some(err.to_string()),
                }
            }
            BaseMessage::MacroMessage(MacroMenuMessage::BackConfirmed) => {
                self.selected = WindowShowing::Start;

                match self.macro_menu.update(MacroMenuMessage::BackConfirmed) {
                    Ok(com) => return com.map(BaseMessage::MacroMessage),
                    Err(err) => self.curr_error = Some(err.to_string()),
                }
            },
            BaseMessage::MacroMessage(msg) => {
                match self.macro_menu.update(msg) {
                    Ok(com) => return com.map(BaseMessage::MacroMessage),
                    Err(err) => self.curr_error = Some(err.to_string()),
                }
            },
            BaseMessage::MainMessage(msg) => {
                match self.main_menu.update(msg) {
                    Ok(com) => return com.map(BaseMessage::MainMessage),
                    Err(err) => self.curr_error = Some(err.to_string()),
                }
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let content = match &self.selected {
            WindowShowing::Start => container(self.main_menu.view().map(BaseMessage::MainMessage)),
            WindowShowing::MacroMenu => container(self.macro_menu.view().map(BaseMessage::MacroMessage)),     // might have more later
        }
        .height(Length::Fill)
        .width(Length::Fill)
        .center_x()
        .padding(10);

        return Modal::new(self.curr_error.is_some(), content, || {
            Card::new(text("Error!"), text(self.curr_error.clone().unwrap_or("".to_string())))   // weird solution, but unwrapping panics despite loading only on .is_some()
            .foot(
                container(
                    button(
                        text(
                            "Ok"
                        )
                    )
                    .on_press(BaseMessage::DismissError)
                )
                .center_x()
                .width(Length::Fill)
            )
            .max_width(300)
            .on_close(BaseMessage::DismissError)
            .into()
        })
        .backdrop(BaseMessage::DismissError)
        .on_esc(BaseMessage::DismissError)
        .into();
    }
}