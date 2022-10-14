use iced::{pure::{column, text, button, container, scrollable, row}, Length};
use iced_pure::Element;
use iced_lazy::pure::{self, Component};
use iced_native::{text, svg, image};

use crate::{macro_logic::{Macro, MacroStep}, ui::style::{PlusButton, BorderedContainer}};

use super::macro_step_component;

pub struct MacroMenu<Message> {
    macro_name: String,
    on_go_back: Box<dyn Fn() -> Message>,
    on_error: Box<dyn Fn(String) -> Message>
}

impl<Message> MacroMenu<Message> {
    pub fn new(
        macro_name: String,
        on_go_back: impl Fn() -> Message + 'static,
        on_error: impl Fn(String) -> Message + 'static
    ) -> Self {
        MacroMenu { macro_name, on_go_back: Box::new(on_go_back), on_error: Box::new(on_error) }
    }
}

#[derive(Clone)]
pub enum MMEvent {
    NewVal(MacroStep, usize),
    Removed(usize),
    Add,
    EmitError(String)
}

impl<Message, Renderer> Component<Message, Renderer> for MacroMenu<Message>
where
    Renderer: text::Renderer + svg::Renderer + image::Renderer + 'static,
    <Renderer as iced_native::image::Renderer>::Handle: From<iced::image::Handle>
{
    type State = Macro;
    type Event = MMEvent;

    fn update(
        &mut self,
        state: &mut Self::State,
        event: Self::Event,
    ) -> Option<Message> {
        match event {
            MMEvent::NewVal(val, index) => _ = state.macro_steps.splice(index..index+1, [val]),
            MMEvent::Removed(index) => _ = state.macro_steps.remove(index),
            MMEvent::Add => state.macro_steps.push(Default::default()),
            MMEvent::EmitError(error) => return Some((self.on_error)(error)),
        }

        None
    }

    fn view(&self, state: &Self::State) -> Element<Self::Event, Renderer> {
        let mut macro_ui = column().push(
            container(
                text(
                    self.macro_name.clone()
                )
                .size(42)
            )
            .padding(5)
        );

        for (i, macro_step) in state.macro_steps.iter().enumerate() {
            macro_ui = macro_ui.push(
                macro_step_component(
                    i,
                    Some(macro_step.clone()),
                    MMEvent::NewVal,
                    MMEvent::Removed,
                    MMEvent::EmitError
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
                        .on_press(MMEvent::Add)
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

impl<'a, Message, Renderer> From<MacroMenu<Message>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: text::Renderer + svg::Renderer + image::Renderer + 'static,
    <Renderer as iced_native::image::Renderer>::Handle: From<iced::image::Handle>
{
    fn from(macro_menu: MacroMenu<Message>) -> Self {
        pure::component(macro_menu)
    }
}

pub fn macro_menu<Message>(
    macro_name: String,
    on_go_back: impl Fn() -> Message + 'static,
    on_error: impl Fn(String) -> Message + 'static
) -> MacroMenu<Message> {
    MacroMenu::new(macro_name, on_go_back, on_error)
}