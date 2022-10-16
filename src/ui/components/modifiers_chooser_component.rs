use autopilot::key::Flag;
use iced::{pure::{toggler, column, text, row}, Alignment, Length};
use iced_pure::Element;
use iced_native;
use iced_lazy::pure::{self, Component};

pub struct ModifiersChooserComponent<Message> {
    flags: Vec<Flag>,
    on_change: Box<dyn Fn(Vec<Flag>) -> Message>
}

impl<Message> ModifiersChooserComponent<Message> {
    pub fn new(
        flags: Vec<Flag>,
        on_change: impl Fn(Vec<Flag>) -> Message + 'static
    ) -> Self {
        ModifiersChooserComponent { flags, on_change: Box::new(on_change) }
    }
}

pub enum MCCEvent {
    CtrlFlagFlipped(bool),
    AltFlagFlipped(bool),
    ShiftFlagFlipped(bool)
}

impl<Message, Renderer> Component<Message, Renderer> for ModifiersChooserComponent<Message>
where
    Renderer: iced_native::text::Renderer + 'static
{
    type State = ();
    type Event = MCCEvent;

    fn update(
        &mut self,
        _state: &mut Self::State,
        event: Self::Event,
    ) -> Option<Message> {
        match event {
            MCCEvent::CtrlFlagFlipped(flag) => {
                if flag {
                    self.flags.push(Flag::Control);
                } else {
                    self.flags.remove(self.flags.iter().position(|elem| *elem == Flag::Control).unwrap());  // flag has to always exist at this point
                }
            },
            MCCEvent::AltFlagFlipped(flag) => {
                if flag {
                    self.flags.push(Flag::Alt);
                } else {
                    self.flags.remove(self.flags.iter().position(|elem| *elem == Flag::Alt).unwrap());  // flag has to always exist at this point
                }
            },
            MCCEvent::ShiftFlagFlipped(flag) => {
                if flag {
                    self.flags.push(Flag::Shift);
                } else {
                    self.flags.remove(self.flags.iter().position(|elem| *elem == Flag::Shift).unwrap());  // flag has to always exist at this point
                }
            },
        }

        Some((self.on_change)(self.flags.clone()))
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event, Renderer> {
        column().push(
            text(
                "Modifiers"
            )
        ).push(
            row().push(
                toggler(
                    Some("Ctrl".to_string()),
                    self.flags.contains(&Flag::Control),
                    MCCEvent::CtrlFlagFlipped
                )
                .width(Length::Shrink)
            ).push(
                toggler(
                    Some("Alt".to_string()),
                    self.flags.contains(&Flag::Alt),
                    MCCEvent::AltFlagFlipped
                )
                .width(Length::Shrink)
            ).push(
                toggler(
                    Some("Shift".to_string()),
                    self.flags.contains(&Flag::Shift),
                    MCCEvent::ShiftFlagFlipped
                )
                .width(Length::Shrink)
            )
        )
        .align_items(Alignment::Center)
        .into()
    }
}

impl <'a, Message, Renderer> From<ModifiersChooserComponent<Message>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: iced_native::text::Renderer + 'static
{
    fn from(modifiers_chooser_component: ModifiersChooserComponent<Message>) -> Self {
        pure::component(modifiers_chooser_component)
    }
}

pub fn modifiers_chooser_component<Message>(
    flags: Vec<Flag>,
    on_change: impl Fn(Vec<Flag>) -> Message + 'static
) -> ModifiersChooserComponent<Message> {
    ModifiersChooserComponent::new(flags, on_change)
}