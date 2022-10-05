use iced::{button, Color};

pub enum TextButton {
    Normal
}

impl button::StyleSheet for TextButton {
    fn active(&self) -> button::Style {
        button::Style {
            border_color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
            text_color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.85 },
            ..Default::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
            ..Default::default()
        }
    }
}

pub enum PlusButton {
    Normal
}

impl button::StyleSheet for PlusButton {
    fn active(&self) -> iced::button::Style {
        button::Style {
            border_color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
            text_color: Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
            ..Default::default()
        }
    }

    fn hovered(&self) -> iced::button::Style {
        button::Style {
            text_color: Color { r: 0.4, g: 0.4, b: 1.0, a: 1.0 },
            ..Default::default()
        }
    }
}