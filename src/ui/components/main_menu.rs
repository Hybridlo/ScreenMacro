use iced::{pure::{column, text, button, container, text_input}, Length, Alignment, alignment::Horizontal};
use iced_pure::Element;
use iced_lazy::pure::{self, Component};
use iced_native::text;
use iced_aw::pure::{Modal, Card};

use crate::ui::style::TextButton;

pub struct MainMenu<Message> {
    show_new_macro: bool,
    new_macro_name: String,
    on_new_macro: Box<dyn Fn(String) -> Message>
}

impl<Message> MainMenu<Message> {
    pub fn new(
        on_new_macro: impl Fn(String) -> Message + 'static
    ) -> Self {
        MainMenu { show_new_macro: false, new_macro_name: Default::default(), on_new_macro: Box::new(on_new_macro) }
    }
}

#[derive(Clone)]
pub enum MMEvent {
    NewButtonClicked,
    LoadButtonClicked,
    NewMacroDismissed,
    MacroNameUpdate(String),
    NewMacroOk
}

impl<Message, Renderer> Component<Message, Renderer> for MainMenu<Message>
where
    Renderer: text::Renderer + iced_native::text::Renderer<Font = iced_native::Font> + 'static
{
    type State = ();
    type Event = MMEvent;

    fn update(
        &mut self,
        _state: &mut Self::State,
        event: Self::Event,
    ) -> Option<Message> {
        match event {
            MMEvent::NewButtonClicked => self.show_new_macro = true,
            MMEvent::LoadButtonClicked => (),
            MMEvent::NewMacroDismissed => { self.show_new_macro = false; self.new_macro_name = "".to_string() },
            MMEvent::MacroNameUpdate(name) => self.new_macro_name = name,
            MMEvent::NewMacroOk => return Some((self.on_new_macro)(self.new_macro_name.clone())),
        }

        return None;
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event, Renderer> {
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
                    .on_press(MMEvent::NewButtonClicked)
                )

                .push(
                    button(
                            text("Load macro")
                            .size(28)
                    )
                    .style(TextButton::Normal)
                    .on_press(MMEvent::LoadButtonClicked)
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
                        MMEvent::MacroNameUpdate
                    )
                    .on_submit(MMEvent::NewMacroOk)
                )
                .foot(
                    container(
                        button(
                            text("Ok")
                        )
                        .on_press(MMEvent::NewMacroOk)
                    )
                    .width(Length::Fill)
                    .center_x()
                )
                .max_width(300)
                .on_close(MMEvent::NewMacroDismissed)
                .into()
            })
            .backdrop(MMEvent::NewMacroDismissed)
            .on_esc(MMEvent::NewMacroDismissed)
            .into();
        }

        content.into()
    }
}

impl<'a, Message, Renderer> From<MainMenu<Message>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: text::Renderer + iced_native::text::Renderer<Font = iced_native::Font> + 'static
{
    fn from(main_menu: MainMenu<Message>) -> Self {
        pure::component(main_menu)
    }
}

pub fn main_menu<Message>(
    on_new_macro: impl Fn(String) -> Message + 'static
) -> MainMenu<Message> {
    MainMenu::new(on_new_macro)
}