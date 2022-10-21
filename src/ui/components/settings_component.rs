/// i wanted to use this in macro_menu, but Modal freaks out if you provide
/// a custom component, but i don't want to remove this so it'll stay here
/// for now, maybe iced 0.5 will have it working

use iced_pure::Element;
use iced_lazy::pure::{self, Component};
use iced_native;
use iced::pure::{text, column, row, toggler};

use crate::macro_logic::Settings;

use super::my_numeric_input;

pub struct SettingsComponent<Message> {
    settings: Settings,
    on_change: Box<dyn Fn(Settings) -> Message>
}

impl<Message> SettingsComponent<Message> {
    pub fn new(
        settings: Settings,
        on_change: impl Fn(Settings) -> Message + 'static
    ) -> Self {
        SettingsComponent { settings, on_change: Box::new(on_change) }
    }
}

pub enum SCEvent {
    BreakWholeMacroChanged(bool),
    StepTimeoutChanges(u64)
}

impl<Message, Renderer> Component<Message, Renderer> for SettingsComponent<Message>
where
    Renderer: iced_native::text::Renderer + 'static
{
    type State = ();
    type Event = SCEvent;

    fn update(
        &mut self,
        _state: &mut Self::State,
        event: Self::Event,
    ) -> Option<Message> {
        match event {
            SCEvent::BreakWholeMacroChanged(break_whole_macro) => self.settings.break_whole_macro = break_whole_macro,
            SCEvent::StepTimeoutChanges(timeout) => self.settings.step_timeout_seconds = timeout,
        }

        Some((self.on_change)(self.settings.clone()))
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event, Renderer> {
        column().push(
            toggler(
                Some("Stop whole macro on timeout".to_string()), 
                self.settings.break_whole_macro, 
                SCEvent::BreakWholeMacroChanged
            )
        ).push(
            row().push(
                text("Timeout in seconds")
            ).push(
                my_numeric_input(
                    "seconds".to_string(), 
                    "s".to_string(), 
                    self.settings.step_timeout_seconds, 
                    SCEvent::StepTimeoutChanges
                )
            )
        ).into()
    }
}

impl<'a, Message, Renderer> From<SettingsComponent<Message>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: iced_native::text::Renderer + 'static
{
    fn from(settings_component: SettingsComponent<Message>) -> Self {
        pure::component(settings_component)
    }
}

pub fn settings_component<Message>(
    settings: Settings,
    on_change: impl Fn(Settings) -> Message + 'static
) -> SettingsComponent<Message> {
    SettingsComponent::new(settings, on_change)
}