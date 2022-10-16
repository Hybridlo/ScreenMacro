use anyhow::{Result, anyhow};
use autopilot::key::KeyCode;

use super::MacroStep;

pub trait EnumInterString     // would use ToString and FromStr but can't impl those for KeyCodes from autopilot
where
    Self: Sized
{
    type Err;

    fn all_string_options() -> Vec<String>;
    fn to_string(&self) -> String;
    fn from_str(s: &str) -> Result<Self, Self::Err>;
}

impl EnumInterString for MacroStep {
    type Err = anyhow::Error;

    fn all_string_options() -> Vec<String> {
        vec![
            "Launch program".to_string(),
            "Click an image".to_string(),
            "Wait for image".to_string(),
            "Type text".to_string(),
            "Press key".to_string(),
            "Wait".to_string()
        ]
    }

    fn to_string(&self) -> String {
        match self {
            MacroStep::Launch(_) => "Launch program",
            MacroStep::ClickImage(_, _, _) => "Click an image",
            MacroStep::AwaitImage(_, _) => "Wait for image",
            MacroStep::TypeText(_, _) => "Type text",
            MacroStep::PressKey(_, _) => "Press key",
            MacroStep::WaitTime(_) => "Wait",
        }.to_string()
    }

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Launch program" => MacroStep::default_launch(),
            "Click an image" => MacroStep::default_click_image(),
            "Wait for image" => MacroStep::default_await_image(),
            "Type text" => MacroStep::default_type_text(),
            "Press key" => MacroStep::default_press_key(),
            "Wait" => MacroStep::default_wait(),
            _ => return Err(anyhow!("Failed to convert string to MacroStep enum"))
        })
    }
}

impl EnumInterString for KeyCode {
    type Err = anyhow::Error;

    fn all_string_options() -> Vec<String> {
        vec![
            "Left".to_string(),
            "Right".to_string(),
            "Down".to_string(),
            "Up".to_string(),
            "Enter".to_string(),
            "Esc".to_string(),
            "Space".to_string(),
            "Ctrl".to_string(),
            "Alt".to_string(),
            "Shift".to_string(),
            "Tab".to_string(),
            "Del".to_string(),
            "End".to_string(),
            "Home".to_string(),
            "Page Up".to_string(),
            "Page Down".to_string(),
            "Backspace".to_string(),
            "Caps Lock".to_string(),
            "F1".to_string(),
            "F2".to_string(),
            "F3".to_string(),
            "F4".to_string(),
            "F5".to_string(),
            "F6".to_string(),
            "F7".to_string(),
            "F8".to_string(),
            "F9".to_string(),
            "F10".to_string(),
            "F11".to_string(),
            "F12".to_string(),
            "F13".to_string(),
            "F14".to_string(),
            "F15".to_string(),
            "F16".to_string(),
            "F17".to_string(),
            "F18".to_string(),
            "F19".to_string(),
            "F20".to_string(),
            "F21".to_string(),
            "F22".to_string(),
            "F23".to_string(),
            "F24".to_string(),
            "Meta".to_string(),
        ]
    }

    fn to_string(&self) -> String {
        match self {
            KeyCode::F1 => "F1",
            KeyCode::F2 => "F2",
            KeyCode::F3 => "F3",
            KeyCode::F4 => "F4",
            KeyCode::F5 => "F5",
            KeyCode::F6 => "F6",
            KeyCode::F7 => "F7",
            KeyCode::F8 => "F8",
            KeyCode::F9 => "F9",
            KeyCode::F10 => "F10",
            KeyCode::F11 => "F11",
            KeyCode::F12 => "F12",
            KeyCode::F13 => "F13",
            KeyCode::F14 => "F14",
            KeyCode::F15 => "F15",
            KeyCode::F16 => "F16",
            KeyCode::F17 => "F17",
            KeyCode::F18 => "F18",
            KeyCode::F19 => "F19",
            KeyCode::F20 => "F20",
            KeyCode::F21 => "F21",
            KeyCode::F22 => "F22",
            KeyCode::F23 => "F23",
            KeyCode::F24 => "F24",
            KeyCode::LeftArrow => "Left",
            KeyCode::Control => "Ctrl",
            KeyCode::RightArrow => "Right",
            KeyCode::DownArrow => "Down",
            KeyCode::End => "End",
            KeyCode::UpArrow => "Up",
            KeyCode::PageUp => "Page Up",
            KeyCode::Alt => "Alt",
            KeyCode::Return => "Enter",
            KeyCode::PageDown => "Page Down",
            KeyCode::Delete => "Del",
            KeyCode::Home => "Home",
            KeyCode::Escape => "Esc",
            KeyCode::Backspace => "Backspace",
            KeyCode::Meta => "Meta",
            KeyCode::CapsLock => "Caps Lock",
            KeyCode::Shift => "Shift",
            KeyCode::Tab => "Tab",
            KeyCode::Space => "Space",
        }.to_string()
    }

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "F1" => KeyCode::F1,
            "F2" => KeyCode::F2,
            "F3" => KeyCode::F3,
            "F4" => KeyCode::F4,
            "F5" => KeyCode::F5,
            "F6" => KeyCode::F6,
            "F7" => KeyCode::F7,
            "F8" => KeyCode::F8,
            "F9" => KeyCode::F9,
            "F10" => KeyCode::F10,
            "F11" => KeyCode::F11,
            "F12" => KeyCode::F12,
            "F13" => KeyCode::F13,
            "F14" => KeyCode::F14,
            "F15" => KeyCode::F15,
            "F16" => KeyCode::F16,
            "F17" => KeyCode::F17,
            "F18" => KeyCode::F18,
            "F19" => KeyCode::F19,
            "F20" => KeyCode::F20,
            "F21" => KeyCode::F21,
            "F22" => KeyCode::F22,
            "F23" => KeyCode::F23,
            "F24" => KeyCode::F24,
            "Left" => KeyCode::LeftArrow,
            "Ctrl" => KeyCode::Control,
            "Right" => KeyCode::RightArrow,
            "Down" => KeyCode::DownArrow,
            "End" => KeyCode::End,
            "Up" => KeyCode::UpArrow,
            "Page Up" => KeyCode::PageUp,
            "Alt" => KeyCode::Alt,
            "Enter" => KeyCode::Return,
            "Page Down" => KeyCode::PageDown,
            "Del" => KeyCode::Delete,
            "Home" => KeyCode::Home,
            "Esc" => KeyCode::Escape,
            "Backspace" => KeyCode::Backspace,
            "Meta" => KeyCode::Meta,
            "Caps Lock" => KeyCode::CapsLock,
            "Shift" => KeyCode::Shift,
            "Tab" => KeyCode::Tab,
            "Space" => KeyCode::Space,
            _ => return Err(anyhow!("Failed to convert string to KeyCode enum"))
        })
    }
}

#[cfg(test)]
mod tests {
    // tests module to make sure that there are no typos n stuff in EnumInterString implementations
    // it doesn't check whether all_string_options is exaustive, because i'm not sure if that's possible

    use autopilot::key::KeyCode;

    use super::MacroStep;
    use super::EnumInterString;

    #[test]
    fn check_macro_step_from_str() {
        let options = MacroStep::all_string_options();

        for option in options {
            let a = MacroStep::from_str(&option);
            assert!(a.is_ok());
        }
    }

    #[test]
    fn check_macro_step_to_string() {
        let options = MacroStep::all_string_options();

        for option in options {
            let a = MacroStep::from_str(&option).unwrap();
            
            let res = a.to_string();

            assert!(res == option);
        }
    }

    #[test]
    fn check_keycode_from_str() {
        let options = KeyCode::all_string_options();

        for option in options {
            let a = KeyCode::from_str(&option);
            assert!(a.is_ok());
        }
    }

    #[test]
    fn check_keycode_to_string() {
        let options = KeyCode::all_string_options();

        for option in options {
            let a = KeyCode::from_str(&option).unwrap();
            
            let res = a.to_string();

            assert!(res == option);
        }
    }
}