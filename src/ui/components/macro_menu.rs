use iced::{pure::{column, text, button, container, scrollable, row}, Length, alignment::Vertical, Alignment, Font};
use iced_pure::Element;
use iced_lazy::pure::{self, Component};
use iced_native::{text, svg, image};

use crate::{macro_logic::{Macro, MacroStep}, ui::style::{PlusButton, BorderedContainer}};

use super::macro_step_component;

pub struct MacroMenu<Message> {
    macro_data: Macro,
    on_go_back: Box<dyn Fn() -> Message>,
    on_error: Box<dyn Fn(String) -> Message>,
    on_change: Box<dyn Fn(Macro) -> Message>
}

impl<Message> MacroMenu<Message> {
    pub fn new(
        macro_data: Macro,
        on_go_back: impl Fn() -> Message + 'static,
        on_error: impl Fn(String) -> Message + 'static,
        on_change: impl Fn(Macro) -> Message + 'static
    ) -> Self {
        MacroMenu { macro_data, on_go_back: Box::new(on_go_back), on_error: Box::new(on_error), on_change: Box::new(on_change) }
    }
}

#[derive(Clone)]
pub enum MMEvent {
    NewVal(MacroStep, usize),
    Removed(usize),
    Add,
    EmitError(String),
    BackPressed,
    PlayPressed
}

impl<Message, Renderer> Component<Message, Renderer> for MacroMenu<Message>
where
    Renderer: text::Renderer<Font = Font> + svg::Renderer + image::Renderer + 'static,
    <Renderer as iced_native::image::Renderer>::Handle: From<iced::image::Handle>
{
    type State = ();
    type Event = MMEvent;

    fn update(
        &mut self,
        _state: &mut Self::State,
        event: Self::Event,
    ) -> Option<Message> {
        match event {
            MMEvent::NewVal(val, index) => {
                self.macro_data.macro_steps.splice(index..index+1, [val]);
                return Some((self.on_change)(self.macro_data.clone()))
            },

            MMEvent::Removed(index) => {
                self.macro_data.macro_steps.remove(index);
                return Some((self.on_change)(self.macro_data.clone()))
            },

            MMEvent::Add => {
                self.macro_data.macro_steps.push(Default::default());
                return Some((self.on_change)(self.macro_data.clone()))
            },
            
            MMEvent::EmitError(error) => return Some((self.on_error)(error)),
            MMEvent::BackPressed => return Some((self.on_go_back)()),
            MMEvent::PlayPressed => {
                if let Err(error) = self.macro_data.execute_macro() {
                    return Some((self.on_error)(error.to_string()));
                }
            },
        }

        None
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event, Renderer> {
        let mut macro_ui = column().push(
            container(
                text(
                    self.macro_data.macro_name.clone()
                )
                .size(42)
            )
            .padding(5)
        );

        for (i, macro_step) in self.macro_data.macro_steps.iter().enumerate() {
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

        let side_panel = column().push(
            container(
                column().push(
                    button(
                        text("play")
                    )
                    .on_press(MMEvent::PlayPressed)
                )
            )
            .height(Length::Shrink)
        ).push(
            container(
                column().push(
                    button(
                        text("Back")
                    )
                    .on_press(MMEvent::BackPressed)
                )
            )
            .height(Length::Fill)
            .align_y(Vertical::Bottom)
        )
        .width(Length::Fill)
        .align_items(Alignment::Center);

        row()
        .push(
           macro_container 
        ).push(
            container(
                side_panel
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
    Renderer: text::Renderer<Font = Font> + svg::Renderer + image::Renderer + 'static,
    <Renderer as iced_native::image::Renderer>::Handle: From<iced::image::Handle>
{
    fn from(macro_menu: MacroMenu<Message>) -> Self {
        pure::component(macro_menu)
    }
}

pub fn macro_menu<Message>(
    macro_data: Macro,
    on_go_back: impl Fn() -> Message + 'static,
    on_error: impl Fn(String) -> Message + 'static,
    on_change: impl Fn(Macro) -> Message + 'static
) -> MacroMenu<Message> {
    MacroMenu::new(macro_data, on_go_back, on_error, on_change)
}