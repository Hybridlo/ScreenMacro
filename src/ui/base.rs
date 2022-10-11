use iced::{pure::{Sandbox, container, text, Element}, Length};
use iced_aw::pure::{Card, Modal};
use iced_pure::button;

use crate::macro_logic::Macro;

use super::components::{main_menu, macro_menu};


pub struct Base {
    macro_info: Option<Macro>,      // TODO: if there's ever a way to show errors without re-rendering macro_menu this shouldn't be needed
    selected: WindowShowing,
    curr_error: Option<String>,
}

enum WindowShowing {
    Start,
    MacroMenu(String)
}

#[derive(Debug, Clone)]
pub enum BaseMessage {
    NewMacro(String),
    DismissError,
    ShowError(String),
    ShowMainMenu,
    MacroInfoUpdate(Macro)
}

impl Sandbox for Base {
    type Message = BaseMessage;

    fn new() -> Self {
        Base {
            macro_info: Default::default(),
            selected: WindowShowing::Start,
            curr_error: Default::default()
        }
    }

    fn title(&self) -> String {
        "ScreenMacro".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            BaseMessage::DismissError => self.curr_error = None,
            BaseMessage::NewMacro(name) => self.selected = WindowShowing::MacroMenu(name),
            BaseMessage::ShowError(error) => self.curr_error = Some(error),
            BaseMessage::ShowMainMenu => self.selected = WindowShowing::Start,
            BaseMessage::MacroInfoUpdate(macro_info) => self.macro_info = Some(macro_info),
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let content = match &self.selected {
            WindowShowing::Start => container(main_menu(BaseMessage::NewMacro)),
            WindowShowing::MacroMenu(name) => container(
                macro_menu(
                    name.clone(),
                    self.macro_info.clone(),
                    || BaseMessage::ShowMainMenu,
                    BaseMessage::ShowError,
                    BaseMessage::MacroInfoUpdate
                )
            ),     // might have more later
        }
        .height(Length::Fill)
        .width(Length::Fill)
        .center_x()
        .padding(10);

        println!("{}", self.curr_error.is_some());

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