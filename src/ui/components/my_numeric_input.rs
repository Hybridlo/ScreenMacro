use std::{str::FromStr, fmt::Display};

use iced::pure::{text_input, row, text};
use iced_pure::Element;
use iced_lazy::pure::{self, Component};
use iced_native::text;
use num::PrimInt;

pub struct MyNumericInput<Message, T: PrimInt> {
    size: u16,
    placeholder: String,
    second_text: String,
    value: T,
    on_change: Box<dyn Fn(T) -> Message>,
}

impl<Message, T: PrimInt> MyNumericInput<Message, T> {
    pub fn new(
        placeholder: String,
        second_text: String,
        value: T,
        on_change: impl Fn(T) -> Message + 'static
    ) -> Self {
        MyNumericInput { size: 18, placeholder, second_text, value, on_change: Box::new(on_change) }
    }

    pub fn size(self, size: u16) -> Self {
        Self {
            size,
            ..self
        }
    }
}

#[derive(Debug, Clone)]
pub enum TIEvent {
    InputChanged(String)
}

impl<Message, Renderer, T> Component<Message, Renderer> for MyNumericInput<Message, T>
where
    Renderer: text::Renderer + 'static,
    T: PrimInt + FromStr + Display
{
    type State = ();
    type Event = TIEvent;

    fn update(
        &mut self,
        _state: &mut Self::State,
        event: Self::Event,
    ) -> Option<Message> {
        match event {
            TIEvent::InputChanged(text) => {
                if text == "" {
                    return Some((self.on_change)(T::zero()));
                }
                
                let num_res = text.parse::<T>().ok()?;

                Some((self.on_change)(num_res))
            },
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event, Renderer> {
        row().push(
            text_input(
                &self.placeholder,
                &format!("{}", self.value),
                TIEvent::InputChanged
            )
            .size(self.size)
        ).push(
            text(self.second_text.clone()).size(self.size)
        )
        .into()
    }
}

impl<'a, Message, Renderer, T> From<MyNumericInput<Message, T>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: text::Renderer + 'static,
    T: PrimInt + FromStr + Display + 'a
{
    fn from(percent_text_input: MyNumericInput<Message, T>) -> Self {
        pure::component(percent_text_input)
    }
}

pub fn my_numeric_input<Message, T: PrimInt>(
    placeholder: String,
    second_text: String,
    value: T,
    on_change: impl Fn(T) -> Message + 'static
) -> MyNumericInput<Message, T> {
    MyNumericInput::new(placeholder, second_text, value, on_change) 
}