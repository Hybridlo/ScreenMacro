use iced::pure::{text_input, row, text};
use iced_pure::Element;
use iced_lazy::pure::{self, Component};
use iced_native::text;

pub struct TimeInput<Message> {
    size: u16,
    placeholder: String,
    value: u64,
    on_change: Box<dyn Fn(u64) -> Message>,
}

impl<Message> TimeInput<Message> {
    pub fn new(
        placeholder: String,
        value: u64,
        on_change: impl Fn(u64) -> Message + 'static
    ) -> Self {
        TimeInput { size: 12, placeholder, value, on_change: Box::new(on_change) }
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

impl<Message, Renderer> Component<Message, Renderer> for TimeInput<Message>
where
    Renderer: text::Renderer + 'static
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
                    return Some((self.on_change)(0));
                }
                
                let num_res = text.parse::<u64>().ok()?;

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
            text("ms").size(self.size)
        )
        .into()
    }
}

impl<'a, Message, Renderer> From<TimeInput<Message>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: text::Renderer + 'static
{
    fn from(percent_text_input: TimeInput<Message>) -> Self {
        pure::component(percent_text_input)
    }
}

pub fn time_input<Message>(
    placeholder: String,
    value: u64,
    on_change: impl Fn(u64) -> Message + 'static
) -> TimeInput<Message> {
    TimeInput::new(placeholder, value, on_change) 
}