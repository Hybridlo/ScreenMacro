use iced::{pure::{Sandbox, container, Element}, Length};

use super::{MainMenu, MainMenuMessage, MacroMenu};


pub struct Base {
    selected: WindowShowing,
    mainmenu: MainMenu,
    macromenu: MacroMenu,
}

enum WindowShowing {
    Start,
    MacroMenu
}

#[derive(Debug, Clone)]
pub enum BaseMessage {
    MainMenuMessage(MainMenuMessage)
}

impl Sandbox for Base {
    type Message = BaseMessage;

    fn new() -> Self {
        Base {
            selected: WindowShowing::Start,
            mainmenu: MainMenu::new(),
            macromenu: MacroMenu::new()
        }
    }

    fn title(&self) -> String {
        "ScreenMacro".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            BaseMessage::MainMenuMessage(_) => self.selected = WindowShowing::MacroMenu
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        return container(
            match self.selected {
                WindowShowing::Start => self.mainmenu.view().map(BaseMessage::MainMenuMessage),
                WindowShowing::MacroMenu => self.macromenu.view(),     // might have more later
            }
        )
            .height(Length::Fill)
            .width(Length::Fill)
            .center_x()
            .into();
    }
}