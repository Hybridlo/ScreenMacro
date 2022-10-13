use anyhow::{Result, anyhow};
use iced::widget::svg::{Svg, Handle};
use image::{RgbImage, DynamicImage, io::Reader};
use autopilot::bitmap;

use std::{process::Command, fs::File};

#[derive(Default, Clone, Debug)]
pub struct Macro {
    pub macro_name: String,
    version: u64,
    settings: String,   // for now, there might be macro-specific, macrostep-specific and global settings later, will see
    pub macro_steps: Vec<MacroStep>
}

impl Macro {
    pub fn new(name: String) -> Self {
        Macro { macro_name: name, ..Default::default() }
    }
}

#[derive(Default, Clone, Debug)]
pub enum ClickPoint {
    TopLeft,
    TopMiddle,
    TopRight,
    CenterLeft,
    #[default]
    CenterMiddle,
    CenterRight,
    BottomLeft,
    BottomMiddle,
    BottomRight
}

#[derive(Debug, Clone)]
pub enum MacroStep {
    Launch(String),                         // has the command
    ClickImage(Option<RgbImage>, ClickPoint, f32),    // image name, click point, allowed difference
    AwaitImage(Option<RgbImage>, f32)                 // image name, allowed difference
}

impl MacroStep {
    pub fn dispatch(&self) -> Result<()> {
        match self {
            MacroStep::Launch(command) => MacroStep::execute_launch(command)?,
            MacroStep::ClickImage(img_data, point, allowed_diff) => MacroStep::execute_click_image(img_data.as_ref().ok_or(anyhow!("Missing image data"))?, point, allowed_diff)?,
            MacroStep::AwaitImage(img_data, allowed_diff) => MacroStep::execute_await_image(img_data.as_ref().ok_or(anyhow!("Missing image data"))?, allowed_diff)?,
        }

        Ok(())
    }

    pub fn default_launch() -> MacroStep {
        MacroStep::Launch("".into())
    }

    pub fn default_click_image() -> MacroStep {
        MacroStep::ClickImage(None, Default::default(), 0.0)
    }

    pub fn default_await_image() -> MacroStep {
        MacroStep::AwaitImage(None, 0.0)
    }

    pub fn to_string(&self) -> String {
        match self {
            MacroStep::Launch(_) => "Launch program",
            MacroStep::ClickImage(_, _, _) => "Click an image",
            MacroStep::AwaitImage(_, _) => "Wait for image",
        }.into()
    }

    pub fn from_string(input: &str) -> Self {
        match input {
            "Launch program" => MacroStep::default_launch(),
            "Click an image" => MacroStep::default_click_image(),
            "Wait for image" => MacroStep::default_await_image(),
            _ => unreachable!()
        }
    }

    pub fn all_string_options() -> Vec<String> {
        vec![
            "Launch program".into(),
            "Click an image".into(),
            "Wait for image".into()
        ]
    }

    fn execute_launch(command: &String) -> Result<()> {
        Command::new(command).spawn()?;

        Ok(())
    }

    fn execute_click_image(img_data: &RgbImage, point: &ClickPoint, allowed_diff: &f32) -> Result<()> {
        let target_img_bitmap = bitmap::Bitmap::new(DynamicImage::ImageRgb8(img_data.clone()), None);

        loop {
            let screen = bitmap::capture_screen()?;
            
            if let Some(found_point) = screen.find_bitmap(&target_img_bitmap, Some(*allowed_diff as f64), None, None) {
                println!("{}", found_point);
                break;
            }
        }

        Ok(())
    }

    fn execute_await_image(img_data: &RgbImage, allowed_diff: &f32) -> Result<()> {
        Ok(())
    }
}

impl Default for MacroStep {
    fn default() -> Self {
        Self::default_launch()
    }
}

impl ClickPoint {
    pub fn svg(&self) -> Svg {
        let (x, y) = match self {
            ClickPoint::TopLeft => {
                (3.5, 3.5)
            },
            ClickPoint::TopMiddle => {
                (20.0, 3.5)
            },
            ClickPoint::TopRight => {
                (36.5, 3.5)
            },
            ClickPoint::CenterLeft => {
                (3.5, 15.0)
            },
            ClickPoint::CenterMiddle => {
                (20.0, 15.0)
            },
            ClickPoint::CenterRight => {
                (36.5, 15.0)
            },
            ClickPoint::BottomLeft => {
                (3.5, 26.5)
            },
            ClickPoint::BottomMiddle => {
                (20.0, 26.5)
            },
            ClickPoint::BottomRight => {
                (36.5, 26.5)
            },
        };

        let svg_body = format!(
            r#"<svg viewBox="0 0 40 30" xmlns="http://www.w3.org/2000/svg">
                    <rect y="28" width="39" height="1.5" style="stroke: rgb(0, 0, 0);" x="0.5"></rect>
                    <rect y="0.5" width="39" height="1.5" style="stroke: rgb(0, 0, 0);" x="0.5"></rect>
                    <rect x="0.5" y="0.5" width="1.5" height="29" style="stroke: rgb(0, 0, 0);"></rect>
                    <rect x="38" y="0.5" width="1.5" height="29" style="stroke: rgb(0, 0, 0);"></rect>
                    <ellipse style="stroke: rgb(0, 0, 0);" cx="{}" cy="{}" rx="3" ry="3"></ellipse>
                </svg>"#,
            x, y);

        Svg::new(Handle::from_memory(svg_body))
    }

    pub fn next(&self) -> Self {
        match &self {
            ClickPoint::TopLeft => Self::TopMiddle,
            ClickPoint::TopMiddle => Self::TopRight,
            ClickPoint::TopRight => Self::CenterRight,
            ClickPoint::CenterLeft => Self::CenterMiddle,
            ClickPoint::CenterMiddle => Self::TopLeft,
            ClickPoint::CenterRight => Self::BottomRight,
            ClickPoint::BottomLeft => Self::CenterLeft,
            ClickPoint::BottomMiddle => Self::BottomLeft,
            ClickPoint::BottomRight => Self::BottomMiddle,
        }
    }
}