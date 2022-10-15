use iced::{pure::{container, row, pick_list, text, button, column}, Length, Alignment, Font};
use iced_pure::{Element, text_input};
use iced_lazy::pure::{self, Component};
use iced_native::text;
use image::RgbImage;

use crate::{macro_logic::MacroStep, ui::style::BorderedContainer};
use crate::ui::style::TextButton;

use super::{file_choose_component, percent_text_input, image_input_component, time_input};

pub struct MacroStepComponent<Message> {
    my_index: usize,
    value: MacroStep,
    on_change: Box<dyn Fn(MacroStep, usize) -> Message>,
    on_remove: Box<dyn Fn(usize) -> Message>,
    on_error: Box<dyn Fn(String) -> Message>
}

impl<Message> MacroStepComponent<Message> {
    pub fn new(
        step_index: usize,
        value: Option<MacroStep>,
        on_change: impl Fn(MacroStep, usize) -> Message + 'static,
        on_remove: impl Fn(usize) -> Message + 'static,
        on_error: impl Fn(String) -> Message + 'static
    ) -> Self {
        if let Some(value) = value {
            return Self { my_index: step_index, value, on_change: Box::new(on_change), on_remove: Box::new(on_remove), on_error: Box::new(on_error) };
        }

        Self { my_index: step_index, value: Default::default(), on_change: Box::new(on_change), on_remove: Box::new(on_remove), on_error: Box::new(on_error) }
    }
}

#[derive(Clone)]
pub enum MSCEvent {
    ChangeStepType(String),
    ChangeCommand(String),
    ChangeImage(RgbImage),
    ChangePoint,
    ChangeAllowedDifference(u32),
    ChangeTextType(String),
    ChangeWaitTime(u64),
    Remove,
    EmitError(String),
    RunCurrentCommand
}

impl<Message, Renderer> Component<Message, Renderer> for MacroStepComponent<Message>
where
    Renderer: text::Renderer<Font = Font> + 'static + iced_native::svg::Renderer + iced_native::image::Renderer,
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
            
            MSCEvent::ChangeTextType(text) => {
                match &self.value {
                    MacroStep::TypeText(_) => self.value = MacroStep::TypeText(text),
                    _ => unreachable!("MSCEvent::ChangeTextType dispatched when the inner value is {:?}", self.value)
                }
            },
            MSCEvent::ChangeWaitTime(time) => {
                match &self.value {
                    MacroStep::WaitTime(_) => self.value = MacroStep::WaitTime(time),
                    _ => unreachable!("MSCEvent::ChangeWaitTime dispatched when the inner value is {:?}", self.value)
                }
            },

            MSCEvent::Remove => {
                return Some((self.on_remove)(self.my_index));
            },

            MSCEvent::EmitError(error) => return Some((self.on_error)(error)),
            MSCEvent::RunCurrentCommand => {
                if let Err(err) = self.value.dispatch() {
                    return Some((self.on_error)("An error occured while trying to execute the command:\n".to_string() + &err.to_string()))
                }
            },
        }

        Some((self.on_change)(self.value.clone(), self.my_index))
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
                            MSCEvent::ChangeImage,
                            MSCEvent::EmitError
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
                            ((1.0 - allowed_difference) * 100.0).round() as u32, 
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
                            MSCEvent::ChangeImage,
                            MSCEvent::EmitError
                        )
                    )
                    .width(Length::FillPortion(8))
                )
                .push(
                    container(
                        percent_text_input(
                            "0".into(), 
                            ((1.0 - allowed_difference) * 100.0).round() as u32, 
                            MSCEvent::ChangeAllowedDifference
                        )
                        .size(30)
                    )
                    .width(Length::Units(70))
                )

            },

            MacroStep::TypeText(text) => {
                res = res.push(
                    text_input(
                        "Text to type", 
                        text,
                        MSCEvent::ChangeTextType
                    )
                    .size(30)
                )
            },

            MacroStep::WaitTime(time) => {
                res = res.push(
                    time_input(
                        "Time to wait in milliseconds".to_string(),
                        *time,
                        MSCEvent::ChangeWaitTime
                    )
                )
            },
        }

        res = res.push(
            button(
                text(
                    "play"      // TODO: make it an svg probably
                )
            )
            .on_press(MSCEvent::RunCurrentCommand)
        );

        container(
            res
            .push(
                column().push(
                    button(
                        text(
                            "x"
                        )
                    )
                    .on_press(MSCEvent::Remove)
                )
                .height(Length::Fill)
            )
            .spacing(3)
            .align_items(Alignment::Center)
        )
        .height(Length::Units(150))     //replace this wiht max_hight when it's fixed in 0.5
        .style(BorderedContainer::Nothing)
        .padding(8)
        .into()
    }
}

impl<'a, Message, Renderer> From<MacroStepComponent<Message>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'static + text::Renderer<Font = Font> + iced_native::svg::Renderer + iced_native::image::Renderer,
    <Renderer as iced_native::image::Renderer>::Handle: From<iced::image::Handle>
{
    fn from(macro_step_component: MacroStepComponent<Message>) -> Self {
        pure::component(macro_step_component)
    }
}

pub fn macro_step_component<Message>(
    step_index: usize,
    value: Option<MacroStep>,
    on_change: impl Fn(MacroStep, usize) -> Message + 'static,
    on_remove: impl Fn(usize) -> Message + 'static,
    on_error: impl Fn(String) -> Message + 'static
) -> MacroStepComponent<Message> {
    MacroStepComponent::new(step_index, value, on_change, on_remove, on_error)
}

