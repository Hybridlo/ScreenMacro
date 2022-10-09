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
    on_change: Box<dyn Fn(RgbaImage) -> Message>
}

impl<Message> ImageInputComponent<Message> {
    pub fn new(image: Option<RgbaImage>, on_change: impl Fn(RgbaImage) -> Message + 'static) -> Self {
        ImageInputComponent { image, on_change: Box::new(on_change) }
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
        match event {
            IICEvent::OpenImage => {
                let img_path = FileDialog::new().add_filter("pngs", &["png"]).pick_file()?;
                self.image = Some(Reader::open(img_path).ok()?.decode().ok()?.into_rgba8());
            },
            IICEvent::PasteClipboard => {
                let img_data = Clipboard::new().ok()?.get_image().ok()?;
                let img_buffer = RgbaImage::from_raw(img_data.width as u32, img_data.height as u32, img_data.bytes.into())?;
                self.image = Some(img_buffer);
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
    on_change: impl Fn(RgbaImage) -> Message + 'static
) -> ImageInputComponent<Message> {
    ImageInputComponent::new(image, on_change)
}