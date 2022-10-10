use iced::{pure::{Sandbox, container, text, Element}, Length};
use iced_aw::pure::{Card, Modal};
use iced_pure::button;

use super::{MainMenu, MainMenuMessage, MacroMenu, MacroMenuMessage};


pub struct Base {
    selected: WindowShowing,
    mainmenu: MainMenu,
    macromenu: MacroMenu,
    curr_error: Option<String>,
}

enum WindowShowing {
    Start,
    MacroMenu
}

#[derive(Debug, Clone)]
pub enum BaseMessage {
    MainMenuMessage(MainMenuMessage),
    MacroMenuMessage(MacroMenuMessage),
    DismissError
}

impl Sandbox for Base {
    type Message = BaseMessage;

    fn new() -> Self {
        Base {
            selected: WindowShowing::Start,
            mainmenu: MainMenu::new(),
            macromenu: MacroMenu::new(),
            curr_error: Default::default()
        }
    }

    fn title(&self) -> String {
        "ScreenMacro".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            BaseMessage::MainMenuMessage(_) => self.selected = WindowShowing::MacroMenu,
            BaseMessage::MacroMenuMessage(MacroMenuMessage::EmitError(error)) => self.curr_error = Some(error),
            BaseMessage::MacroMenuMessage(msg) => self.macromenu.update(msg),
            BaseMessage::DismissError => self.curr_error = None,
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let content = container(
            match self.selected {
                WindowShowing::Start => self.mainmenu.view().map(BaseMessage::MainMenuMessage),
                WindowShowing::MacroMenu => self.macromenu.view().map(BaseMessage::MacroMenuMessage),     // might have more later
            }
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .center_x()
        .padding(10);

        if let Some(error) = &self.curr_error {
            return Modal::new(true, content, || {
                Card::new(text("Error!"), text(error.clone()))
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

        return content.into();
    }
}