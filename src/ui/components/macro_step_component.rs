use iced_lazy::pure::{self, Component};
use iced_pure::Element;
use iced::{pure::{container, row, pick_list, text}, Length};
use iced_native::text;

use crate::macro_logic::{MacroStep, ClickPoint};

pub struct MacroStepComponent<Message> {
    step_index: usize,
    value: MacroStep,
    on_change: Box<dyn Fn(MacroStep, usize) -> Message>
}

impl<Message> MacroStepComponent<Message> {
    pub fn new(
        step_index: usize,
        value: Option<MacroStep>,
        on_change: impl Fn(MacroStep, usize) -> Message + 'static
    ) -> Self {
        if let Some(value) = value {
            return Self { step_index, value, on_change: Box::new(on_change) };
        }

        Self { step_index, value: Default::default(), on_change: Box::new(on_change) }
    }
}

#[derive(Clone)]
pub enum MSCEvent {
    ChangeStepType(String),
    ChangeCommand(String),
    ChangeImage(String),
    ChangePoint(ClickPoint),
    ChangeAllowedDifference(f32)
}

impl<Message, Renderer> Component<Message, Renderer> for MacroStepComponent<Message>
where
    Renderer: text::Renderer + 'static,
{
    type Event = MSCEvent;
    type State = ();

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            MSCEvent::ChangeStepType(new_type) => self.value = MacroStep::from_string(&new_type),

            MSCEvent::ChangeCommand(new_command) => {
                match &self.value {
                    MacroStep::Launch(_) => self.value = MacroStep::Launch(new_command),
                    _ => unreachable!("MSCEvent::ChangeCommand dispatched when the inner value is {:?}", self.value)
                }
            },

            MSCEvent::ChangeImage(new_image) => {
                match &self.value {
                    MacroStep::ClickImage(_, click_point, allowed_difference) => self.value = MacroStep::ClickImage(new_image, click_point.clone(), allowed_difference.clone()),
                    MacroStep::AwaitImage(_, allowed_difference) => self.value = MacroStep::AwaitImage(new_image, allowed_difference.clone()),
                    _ => unreachable!("MSCEvent::ChangeImage dispatched when the inner value is {:?}", self.value)
                }
            },

            MSCEvent::ChangePoint(new_point) => {
                match &self.value {
                    MacroStep::ClickImage(image, _, allowed_difference) => self.value = MacroStep::ClickImage(image.clone(), new_point, allowed_difference.clone()),
                    _ => unreachable!("MSCEvent::ChangePoint dispatched when the inner value is {:?}", self.value)
                }
            },

            MSCEvent::ChangeAllowedDifference(new_allowed_diff) => {
                match &self.value {
                    MacroStep::ClickImage(image, click_point, _) => self.value = MacroStep::ClickImage(image.clone(), click_point.clone(), new_allowed_diff),
                    MacroStep::AwaitImage(image, _) => self.value = MacroStep::AwaitImage(image.clone(), new_allowed_diff),
                    _ => unreachable!("MSCEvent::ChangeAllowedDifference dispatched when the inner value is {:?}", self.value)
                }
            },
        }

        Some((self.on_change)(self.value.clone(), self.step_index.clone()))
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event, Renderer> {
        let mut res = row();

        res = res.push(
            container(
                pick_list(
                    MacroStep::all_string_options(),
                    Some(self.value.to_string()),
                    MSCEvent::ChangeStepType
                )
            )
            .padding(5)
            .center_y()
            .width(Length::Shrink)
        );

        match &self.value {
            MacroStep::Launch(_) => {
                res = res.push(
                    container(
                        text("Here is the launch text box")
                    )
                    .width(Length::FillPortion(10))
                )
            },

            MacroStep::ClickImage(_, _, _) => {
                res = res.push(
                    container(
                        text("Here is the image chooser stuff")
                    )
                    .width(Length::FillPortion(4))
                )
                .push(
                    container(
                        text("Here is Point Click chooser")
                    )
                    .width(Length::FillPortion(2))
                )
                .push(
                    container(
                        text("Here is allowed difference chooser")
                    )
                    .width(Length::FillPortion(4))
                )
            },

            MacroStep::AwaitImage(_, _) => {
                res = res.push(
                    container(
                        text("Here is the image chooser stuff")
                    )
                    .width(Length::FillPortion(6))
                )
                .push(
                    container(
                        text("Here is allowed difference chooser")
                    )
                    .width(Length::FillPortion(4))
                )

            },
        }

        res.into()
    }
}

impl<'a, Message, Renderer> From<MacroStepComponent<Message>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'static + text::Renderer,
{
    fn from(macro_step_component: MacroStepComponent<Message>) -> Self {
        pure::component(macro_step_component)
    }
}

pub fn macro_step_component<Message>(
    step_index: usize,
    value: Option<MacroStep>,
    on_change: impl Fn(MacroStep, usize) -> Message + 'static
) -> MacroStepComponent<Message> {
    MacroStepComponent::new(step_index, value, on_change)
}

