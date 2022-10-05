use iced::{container, Color};

pub enum BorderedContainer {
    Nothing
}

impl container::StyleSheet for BorderedContainer {
    fn style(&self) -> iced::container::Style {
        container::Style {
            border_color: Color { r: 0.2, g: 0.2, b: 0.2, a: 0.8 },
            border_width: 3.0,
            border_radius: 10.0,
            ..Default::default()
        }
    }
}