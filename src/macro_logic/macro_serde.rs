use std::io::Cursor;

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use autopilot::key::{KeyCode, Flag};
use autopilot::mouse::ScrollDirection;
use serde_with::{SerializeAs, DeserializeAs, serde_as};
use image::io::Reader as ImageReader;
use image::DynamicImage;

use super::{MacroStep, ClickPoint, Settings, Macro};

#[derive(Serialize, Deserialize)]
pub struct MacroSerializable {
    pub macro_name: String,
    pub settings: Settings,
    pub macro_steps: Vec<MacroStepSerializable>
}

impl MacroSerializable {
    pub fn new(macro_name: String, settings: Settings, macro_steps: Vec<MacroStepSerializable>) -> Self {
        MacroSerializable { macro_name, settings, macro_steps }
    }

    pub fn to_normal(self) -> Result<Macro> {
        let mut normal_macro_steps: Vec<MacroStep> = Vec::new();

        for step in self.macro_steps {
            normal_macro_steps.push(step.to_normal()?);
        }

        Ok(Macro::new(
            self.macro_name,
            self.settings,
            normal_macro_steps
        ))
    }

    pub fn from_normal(macro_data: Macro) -> Result<Self> {
        let mut serializable_macro_steps: Vec<MacroStepSerializable> = Vec::new();

        for step in macro_data.macro_steps {
            serializable_macro_steps.push(MacroStepSerializable::from_normal(step)?);
        }

        Ok(Self::new(
            macro_data.macro_name,
            macro_data.settings,
            serializable_macro_steps
        ))
    }
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub enum MacroStepSerializable {
    Launch(String),                         // has the command
    ClickImage(Vec<u8>, ClickPoint, f32),    // image name, click point, allowed difference
    MoveToImage(Vec<u8>, ClickPoint, f32),   // image name, click point which is a move point here, allowed difference
    TypeText(
        String,
        #[serde_as(as = "Vec<FlagSerializable>")]
        Vec<Flag>
    ),
    PressKey(
        #[serde_as(as = "KeyCodeSerializable")]
        KeyCode,
        #[serde_as(as = "Vec<FlagSerializable>")]
        Vec<Flag>
    ),
    Scroll(
        #[serde_as(as = "ScrollDirectionSerializable")]
        ScrollDirection,
        u32
    ),
    WaitTime(u64)
}

impl MacroStepSerializable {
    pub fn to_normal(self) -> Result<MacroStep> {
        match self {
            MacroStepSerializable::Launch(command) => Ok(MacroStep::Launch(command)),
            MacroStepSerializable::ClickImage(image, click_point, allowed_difference) => {
                let parsed_image = ImageReader::new(Cursor::new(image))
                                                                .with_guessed_format()?
                                                                .decode()?
                                                                .into_rgb();

                Ok(MacroStep::ClickImage(Some(parsed_image), click_point, allowed_difference))
            },
            MacroStepSerializable::MoveToImage(image, move_point, allowed_difference) => {
                let parsed_image = ImageReader::new(Cursor::new(image))
                                                                .with_guessed_format()?
                                                                .decode()?
                                                                .into_rgb();

                Ok(MacroStep::MoveToImage(Some(parsed_image), move_point, allowed_difference))
            },
            MacroStepSerializable::TypeText(text, flags) => Ok(MacroStep::TypeText(text, flags)),
            MacroStepSerializable::PressKey(key, flags) => Ok(MacroStep::PressKey(key, flags)),
            MacroStepSerializable::Scroll(direction, amount) => Ok(MacroStep::Scroll(direction, amount)),
            MacroStepSerializable::WaitTime(time) => Ok(MacroStep::WaitTime(time)),
        }
    }

    pub fn from_normal(macro_step: MacroStep) -> Result<Self> {
        match macro_step {
            MacroStep::Launch(command) => Ok(Self::Launch(command)),
            MacroStep::ClickImage(image, click_point, allowed_difference) => {
                let mut byte_image: Vec<u8> = Vec::new();

                DynamicImage::ImageRgb8(
                    image.ok_or(anyhow!("Missing image data"))?
                ).write_to(&mut Cursor::new(&mut byte_image), image::ImageFormat::PNG)?;
                        

                Ok(Self::ClickImage(byte_image, click_point, allowed_difference))
            },
            MacroStep::MoveToImage(image, move_point, allowed_difference) => {
                let mut byte_image: Vec<u8> = Vec::new();

                DynamicImage::ImageRgb8(
                    image.ok_or(anyhow!("Missing image data"))?
                ).write_to(&mut Cursor::new(&mut byte_image), image::ImageFormat::PNG)?;

                Ok(Self::MoveToImage(byte_image, move_point, allowed_difference))
            },
            MacroStep::TypeText(text, flags) => Ok(Self::TypeText(text, flags)),
            MacroStep::PressKey(key, flags) => Ok(Self::PressKey(key, flags)),
            MacroStep::Scroll(direction, amount) => Ok(Self::Scroll(direction, amount)),
            MacroStep::WaitTime(time) => Ok(Self::WaitTime(time)),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "KeyCode")]
enum KeyCodeSerializable {
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    LeftArrow,
    Control,
    RightArrow,
    DownArrow,
    End,
    UpArrow,
    PageUp,
    Alt,
    Return,
    PageDown,
    Delete,
    Home,
    Escape,
    Backspace,
    Meta,
    CapsLock,
    Shift,
    Tab,
    Space,
}

impl SerializeAs<KeyCode> for KeyCodeSerializable {
    fn serialize_as<S>(source: &KeyCode, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        KeyCodeSerializable::serialize(source, serializer)
    }
}

impl<'de> DeserializeAs<'de, KeyCode> for KeyCodeSerializable {
    fn deserialize_as<D>(deserializer: D) -> Result<KeyCode, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        KeyCodeSerializable::deserialize(deserializer)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ScrollDirection")]
enum ScrollDirectionSerializable {
    Up,
    Down,
}

impl SerializeAs<ScrollDirection> for ScrollDirectionSerializable {
    fn serialize_as<S>(source: &ScrollDirection, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        ScrollDirectionSerializable::serialize(source, serializer)
    }
}

impl<'de> DeserializeAs<'de, ScrollDirection> for ScrollDirectionSerializable {
    fn deserialize_as<D>(deserializer: D) -> Result<ScrollDirection, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        ScrollDirectionSerializable::deserialize(deserializer)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Flag")]
enum FlagSerializable {
    Shift,
    Control,
    Alt,
    Meta,
    Help,
}

impl SerializeAs<Flag> for FlagSerializable {
    fn serialize_as<S>(source: &Flag, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        FlagSerializable::serialize(source, serializer)
    }
}

impl<'de> DeserializeAs<'de, Flag> for FlagSerializable {
    fn deserialize_as<D>(deserializer: D) -> Result<Flag, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        FlagSerializable::deserialize(deserializer)
    }
}