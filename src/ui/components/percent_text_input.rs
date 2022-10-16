use iced::pure::{text_input, row, text};
use iced_pure::Element;
use iced_lazy::pure::{self, Component};
use iced_native::text;

pub struct PercentTextInput<Message> {
    size: u16,
    placeholder: String,
    value: u32,
    on_change: Box<dyn Fn(u32) -> Message>,
}

impl<Message> PercentTextInput<Message> {
    pub fn new(
        placeholder: String,
        value: u32,
        on_change: impl Fn(u32) -> Message + 'static
    ) -> Self {
        PercentTextInput { size: 24, placeholder, value, on_change: Box::new(on_change) }
    }

    pub fn size(self, size: u16) -> Self {
        Self {
            size,
            ..self
        }
    }
}

#[derive(Debug, Clone)]
pub enum FTIEvent {
    InputChanged(String)
}

impl<Message, Renderer> Component<Message, Renderer> for PercentTextInput<Message>
where
    Renderer: text::Renderer + 'static
{
    type State = ();
    type Event = FTIEvent;

    fn update(
        &mut self,
        _state: &mut Self::State,
        event: Self::Event,
    ) -> Option<Message> {
        match event {
            FTIEvent::InputChanged(text) => {
                if text == "" {
                    return Some((self.on_change)(0));
                }
                
                let num_res = text.parse::<u32>().ok()?;
                
                if num_res > 100 {      // this is the only reason for a seperate component, maybe i could have it in my_numeric_input, but not yet
                    return None;
                }

                Some((self.on_change)(num_res))
            },
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event, Renderer> {
        row().push(
            text_input(
                &self.placeholder,
                &format!("{}", self.value),
                FTIEvent::InputChanged
            )
            .size(self.size)
        ).push(
            text("%").size(self.size)
        )
        .into()
    }
}

impl<'a, Message, Renderer> From<PercentTextInput<Message>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: text::Renderer + 'static
{
    fn from(percent_text_input: PercentTextInput<Message>) -> Self {
        pure::component(percent_text_input)
    }
}

pub fn percent_text_input<Message>(
    placeholder: String,
    value: u32,
    on_change: impl Fn(u32) -> Message + 'static
) -> PercentTextInput<Message> {
    PercentTextInput::new(placeholder, value, on_change) 
}