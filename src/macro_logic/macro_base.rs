use anyhow::{Result, anyhow};
use autopilot::key::Flag;
use iced::widget::svg::{Svg, Handle};
use image::{RgbImage, DynamicImage};
use autopilot::{bitmap, mouse, geometry::Point, key};
use async_std::task::sleep as async_sleep;

use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};

use super::Settings;

#[derive(Default, Clone, Debug)]
pub struct Macro {
    pub macro_name: String,
    version: u64,
    pub settings: Settings,   // for now, there might be macro-specific, macrostep-specific and global settings later, will see
    pub macro_steps: Vec<MacroStep>
}

impl Macro {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn execute_macro(macro_data: Macro, continue_signal: Arc<Mutex<bool>>, is_running: Arc<Mutex<bool>>) -> Result<()> {
        for step in macro_data.macro_steps.iter() {
            match step.dispatch(&macro_data.settings) {
                Ok(stop_early) => if stop_early { break },
                Err(err) => {
                    *(is_running.lock().unwrap()) = false;
                    return Err(err);
                },
            };
            
            async_sleep(Duration::from_secs(1)).await;
            
            if *(continue_signal.lock().unwrap()) == false {
                break;
            }
        }

        *(is_running.lock().unwrap()) = false;
        
        Ok(())
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
    AwaitImage(Option<RgbImage>, f32),                // image name, allowed difference
    TypeText(String, Vec<Flag>),
    PressKey(key::KeyCode, Vec<Flag>),
    Scroll(mouse::ScrollDirection, u32),
    WaitTime(u64)
}

impl MacroStep {
    pub fn dispatch(&self, settings: &Settings) -> Result<bool> {
        match self {
            MacroStep::Launch(command) => MacroStep::execute_launch(command)?,
            MacroStep::ClickImage(img_data, point, allowed_diff) => return MacroStep::execute_click_image(img_data.as_ref().ok_or(anyhow!("Missing image data"))?, point, allowed_diff, settings),
            MacroStep::AwaitImage(img_data, allowed_diff) => return MacroStep::execute_await_image(img_data.as_ref().ok_or(anyhow!("Missing image data"))?, allowed_diff, settings),
            MacroStep::TypeText(text, flags) => MacroStep::execute_type_text(text, flags)?,
            MacroStep::PressKey(key, flags) => MacroStep::execute_press_key(key, flags)?,
            MacroStep::Scroll(direction, amount) => MacroStep::execute_scroll(direction, amount)?,
            MacroStep::WaitTime(milliseconds) => MacroStep::execute_wait(*milliseconds)?,
        }

        Ok(false)
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

    pub fn default_type_text() -> MacroStep {
        MacroStep::TypeText("".to_string(), vec![])
    }

    pub fn default_press_key() -> MacroStep {
        MacroStep::PressKey(key::KeyCode::LeftArrow, vec![])
    }

    pub fn default_scroll() -> MacroStep {
        MacroStep::Scroll(mouse::ScrollDirection::Down, 0)
    }

    pub fn default_wait() -> MacroStep {
        MacroStep::WaitTime(0)
    }

    fn execute_launch(command: &str) -> Result<()> {
        Command::new(command).spawn()?;

        Ok(())
    }

    fn execute_click_image(img_data: &RgbImage, point: &ClickPoint, allowed_diff: &f32, settings: &Settings) -> Result<bool> {
        let start_time = Instant::now();
        let target_img_bitmap = bitmap::Bitmap::new(DynamicImage::ImageRgb8(img_data.clone()), None);

        loop {
            let screen = bitmap::capture_screen()?;
            
            if let Some(found_point) = screen.find_bitmap(&target_img_bitmap, Some(*allowed_diff as f64), None, None) {
                let (mult_x, mult_y) = point.to_mults();
                mouse::move_to(Point::new(found_point.x + mult_x * (img_data.width() as f64), found_point.y + mult_y * (img_data.height() as f64)))?;
                mouse::click(mouse::Button::Left, None);
                break;
            }

            sleep(Duration::from_millis(300));

            println!("{}", start_time.elapsed().as_secs());

            if start_time.elapsed().as_secs() > settings.step_timeout_seconds {
                return Ok(settings.break_whole_macro)
            }
        }

        Ok(false)
    }

    fn execute_await_image(img_data: &RgbImage, allowed_diff: &f32, settings: &Settings) -> Result<bool> {
        let start_time = Instant::now();
        let target_img_bitmap = bitmap::Bitmap::new(DynamicImage::ImageRgb8(img_data.clone()), None);

        loop {
            let screen = bitmap::capture_screen()?;
            
            if let Some(_) = screen.find_bitmap(&target_img_bitmap, Some(*allowed_diff as f64), None, None) {
                break;
            }

            sleep(Duration::from_millis(300));

            if start_time.elapsed().as_secs() > settings.step_timeout_seconds {
                return Ok(settings.break_whole_macro)
            }
        }

        Ok(false)
    }

    fn execute_type_text(text: &str, flags: &Vec<Flag>) -> Result<()> {
        key::type_string(text, &flags[..], 0.0, 0.0);

        Ok(())
    }

    fn execute_press_key(key: &key::KeyCode, flags: &Vec<Flag>) -> Result<()> {
        key::tap(&key::Code(*key), &flags[..], 0, 0);

        Ok(())
    }

    fn execute_scroll(direction: &mouse::ScrollDirection, clicks: &u32) -> Result<()> {
        mouse::scroll(*direction, *clicks);

        Ok(())
    }

    fn execute_wait(time: u64) -> Result<()> {
        sleep(Duration::from_millis(time));

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

    pub fn to_mults(&self) -> (f64, f64) {
        match self {
            ClickPoint::TopLeft => (0.0, 0.0),
            ClickPoint::TopMiddle => (0.5, 0.0),
            ClickPoint::TopRight => (1.0, 0.0),
            ClickPoint::CenterLeft => (0.0, 0.5),
            ClickPoint::CenterMiddle => (0.5, 0.5),
            ClickPoint::CenterRight => (1.0, 0.5),
            ClickPoint::BottomLeft => (0.0, 1.0),
            ClickPoint::BottomMiddle => (0.5, 1.0),
            ClickPoint::BottomRight => (1.0, 1.0),
        }
    }
}