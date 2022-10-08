use iced::{pure::{text_input, button, text, container}, Length};
use iced_pure::{Element, row};
use iced_lazy::pure::{self, Component};
use iced_native::text;
use rfd::FileDialog;

pub struct FileChooseComponent<Message> {
    path: String,
    on_change: Box<dyn Fn(String) -> Message>
}

impl<Message> FileChooseComponent<Message> {
    pub fn new(
        path: String,
        on_change: impl Fn(String) -> Message + 'static
    ) -> Self {
        FileChooseComponent { path, on_change: Box::new(on_change) }
    }
}

#[derive(Clone, Debug)]
pub enum FCCEvent {
    OpenFilePressed,
    NewFile(String)
}

impl<Message, Renderer> Component<Message, Renderer> for FileChooseComponent<Message>
where
    Renderer: text::Renderer + 'static,
    Message: std::clone::Clone
{
    type State = ();
    type Event = FCCEvent;

    fn update(
        &mut self,
        _state: &mut Self::State,
        event: Self::Event,
    ) -> Option<Message> {
        match event {
            FCCEvent::OpenFilePressed => {
                if let Some(new_path) = FileDialog::new().pick_file() {
                    self.path = new_path.to_string_lossy().into_owned();
                } else {
                    return None;
                }
            },
            FCCEvent::NewFile(new_file_path) => {
                self.path = new_file_path;
            },
        }

        Some((self.on_change)(self.path.clone()))
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event, Renderer> {
        row().push(
            container(
                text_input(
                    "Choose a file to launch",
                    &self.path,
                    FCCEvent::NewFile
                )
                .size(30)
            )
            .width(Length::Fill)
        ).push(
            container(
                button(
                    text("Open")
                ).on_press(FCCEvent::OpenFilePressed)
            )
            .width(Length::Shrink)
        )
        .into()
    }
}

impl <'a, Message, Renderer> From<FileChooseComponent<Message>>
    for Element<'a, Message, Renderer>
where
    Renderer: text::Renderer + 'static,
    Message: std::clone::Clone + 'a
{
    fn from(file_choose_component: FileChooseComponent<Message>) -> Self {
        pure::component(file_choose_component)
    }
}

pub fn file_choose_component<Message>(
    path: String,
    on_change: impl Fn(String) -> Message + 'static
) -> FileChooseComponent<Message> {
    FileChooseComponent::new(path, on_change)
}