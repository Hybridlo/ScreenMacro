use iced::{pure::{row, image, button, text, container}, Length, Alignment};
use iced::pure::widget::image::Handle;
use iced_pure::Element;
use iced_lazy::pure::{self, Component};
use iced_native;
use arboard::Clipboard;
use image::RgbaImage;
use image::io::Reader;
use rfd::FileDialog;

pub struct ImageInputComponent<Message> {
    image: Option<RgbaImage>,
    on_change: Box<dyn Fn(RgbaImage) -> Message>,
    on_error: Box<dyn Fn(String) -> Message>
}

impl<Message> ImageInputComponent<Message> {
    pub fn new(
        image: Option<RgbaImage>,
        on_change: impl Fn(RgbaImage) -> Message + 'static,
        on_error: impl Fn(String) -> Message + 'static
    ) -> Self {
        ImageInputComponent { image, on_change: Box::new(on_change), on_error: Box::new(on_error) }
    }
}

#[derive(Clone)]
pub enum IICEvent {
    OpenImage,
    PasteClipboard
}

impl<Message, Renderer> Component<Message, Renderer> for ImageInputComponent<Message>
where
    Renderer: iced_native::text::Renderer + iced_native::image::Renderer + 'static,
    <Renderer as iced_native::image::Renderer>::Handle: From<iced::image::Handle>
{
    type State = ();
    type Event = IICEvent;

    fn update(
        &mut self,
        _state: &mut Self::State,
        event: Self::Event,
    ) -> Option<Message> {
        // there might be a better way to handle errors
        // but this works for now
        
        match event {
            IICEvent::OpenImage => {
                let img_path = FileDialog::new().add_filter("pngs", &["png"]).pick_file()?;

                if let Ok(png_file) = Reader::open(img_path) {
                    if let Ok(png_data) = png_file.decode() {
                        self.image = Some(png_data.into_rgba8());
                    } else {
                        return Some((self.on_error)("Could not read the image".to_string()));
                    }
                } else {
                    return Some((self.on_error)("Failed to open the file".to_string()));
                }
            },
            IICEvent::PasteClipboard => {
                if let Ok(mut clipboard) = Clipboard::new() {
                    if let Ok(img_data) = clipboard.get_image() {
                        if let Some(img_buffer) = RgbaImage::from_raw(img_data.width as u32, img_data.height as u32, img_data.bytes.into()) {
                            self.image = Some(img_buffer);

                        } else {
                            return Some((self.on_error)("Image has unsupported format".to_string()))
                        }
                    } else {
                        return Some((self.on_error)("Clipboard does not contain an image".to_string()))
                    }
                } else {
                    return Some((self.on_error)("Failed to access the clipboard".to_string()))
                }
            },
        }

        // TODO: we need some proper error handling here!

        if let Some(img) = &self.image{
            return Some((self.on_change)(img.clone()));
        };

        None
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event, Renderer> {
        let mut res = row();

        if let Some(img) = &self.image {
            let handle = Handle::from_pixels(
                img.width(),
                img.height(),
                img.pixels().flat_map(|p| [p.0[2], p.0[1], p.0[0], p.0[3]]).collect()
            );

            res = res.push(
                container(
                    image(handle)
                )
                .width(Length::Fill)
                .center_x()
                .center_y()
            );
        } else {
            res = res.push(
                container(
                    text("")
                )
                .width(Length::Fill)
            )
        }

        res.push(
            container(
                button(
                    text(
                        "From clipboard"
                    )
                )
                .on_press(IICEvent::PasteClipboard)
            )
            .width(Length::Shrink)
        ).push(
            container(
                button(
                    text(
                        "File..."
                    )
                )
                .on_press(IICEvent::OpenImage)
            )
            .width(Length::Shrink)
        )
        .align_items(Alignment::Center)
        .into()
    }
}

impl<'a, Message, Renderer> From<ImageInputComponent<Message>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: iced_native::text::Renderer + iced_native::image::Renderer + 'static,
    <Renderer as iced_native::image::Renderer>::Handle: From<iced::image::Handle>
{
    fn from(image_input_component: ImageInputComponent<Message>) -> Self {
        pure::component(image_input_component)
    }
}

pub fn image_input_component<Message>(
    image: Option<RgbaImage>,
    on_change: impl Fn(RgbaImage) -> Message + 'static,
    on_error: impl Fn(String) -> Message + 'static
) -> ImageInputComponent<Message> {
    ImageInputComponent::new(image, on_change, on_error)
}