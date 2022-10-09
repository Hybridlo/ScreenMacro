use iced::{pure::{container, row, pick_list, text, button}, Length, Alignment};
use iced_pure::Element;
use iced_lazy::pure::{self, Component};
use iced_native::text;
use image::RgbaImage;

use crate::macro_logic::MacroStep;
use crate::ui::style::TextButton;

use super::{file_choose_component, percent_text_input, image_input_component};

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
    ChangeImage(RgbaImage),
    ChangePoint,
    ChangeAllowedDifference(u32)
}

impl<Message, Renderer> Component<Message, Renderer> for MacroStepComponent<Message>
where
    Renderer: text::Renderer + 'static + iced_native::svg::Renderer + iced_native::image::Renderer,
    <Renderer as iced_native::image::Renderer>::Handle: From<iced::image::Handle>
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
                    MacroStep::ClickImage(_, click_point, allowed_difference) => self.value = MacroStep::ClickImage(Some(new_image), click_point.clone(), allowed_difference.clone()),
                    MacroStep::AwaitImage(_, allowed_difference) => self.value = MacroStep::AwaitImage(Some(new_image), allowed_difference.clone()),
                    _ => unreachable!("MSCEvent::ChangeImage dispatched when the inner value is {:?}", self.value)
                }
            },

            MSCEvent::ChangePoint => {
                match &self.value {
                    MacroStep::ClickImage(image, old_point, allowed_difference) => self.value = MacroStep::ClickImage(image.clone(), old_point.next(), allowed_difference.clone()),
                    _ => unreachable!("MSCEvent::ChangePoint dispatched when the inner value is {:?}", self.value)
                }
            },

            MSCEvent::ChangeAllowedDifference(new_allowed_diff) => {
                match &self.value {
                    MacroStep::ClickImage(image, click_point, _) => self.value = MacroStep::ClickImage(image.clone(), click_point.clone(), ((100 - new_allowed_diff) as f32) / 100.0),
                    MacroStep::AwaitImage(image, _) => self.value = MacroStep::AwaitImage(image.clone(), ((100 - new_allowed_diff) as f32) / 100.0),
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
            //.padding(5)
            .center_y()
            .width(Length::Shrink)
        );

        match &self.value {
            MacroStep::Launch(path) => {
                res = res.push(
                    container(
                        file_choose_component(path.clone(), MSCEvent::ChangeCommand)
                    )
                    .width(Length::FillPortion(10))
                )
            },

            MacroStep::ClickImage(curr_image, click_point, allowed_difference) => {
                res = res.push(
                    container(
                        image_input_component(
                            curr_image.clone(),
                            MSCEvent::ChangeImage
                        )
                    )
                    .width(Length::FillPortion(6))
                )
                .push(
                    container(
                        button(
                            click_point.svg()
                        )
                        .on_press(MSCEvent::ChangePoint)
                        .style(TextButton::Normal)
                    )
                    .width(Length::Shrink)
                )
                .push(
                    container(
                        percent_text_input(
                            "0.0".into(), 
                            format!("{}", ((1.0 - allowed_difference) * 100.0).round() as u32), 
                            MSCEvent::ChangeAllowedDifference
                        )
                        .size(30)
                    )
                    .width(Length::Units(70))
                )
            },

            MacroStep::AwaitImage(curr_image, allowed_difference) => {
                res = res.push(
                    container(
                        image_input_component(
                            curr_image.clone(),
                            MSCEvent::ChangeImage
                        )
                    )
                    .width(Length::FillPortion(8))
                )
                .push(
                    container(
                        percent_text_input(
                            "0.0".into(), 
                            format!("{}", ((1.0 - allowed_difference) * 100.0).round() as u32), 
                            MSCEvent::ChangeAllowedDifference
                        )
                        .size(30)
                    )
                    .width(Length::Units(70))
                )

            },
        }

        res
        .spacing(3)
        .align_items(Alignment::Center)
        .into()
    }
}

impl<'a, Message, Renderer> From<MacroStepComponent<Message>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'static + text::Renderer + iced_native::svg::Renderer + iced_native::image::Renderer,
    <Renderer as iced_native::image::Renderer>::Handle: From<iced::image::Handle>
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

