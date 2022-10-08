#[derive(Default)]
pub struct Macro {
    version: u64,
    settings: String,   // for now, there might be macro-specific, macrostep-specific and global settings later, will see
    macro_steps: Vec<MacroStep>
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
    ClickImage(String, ClickPoint, f32),    // image name, click point, allowed difference
    AwaitImage(String, f32)                 // image name, allowed difference
}

impl MacroStep {
    pub fn dispatch(&self) {
        match self {
            MacroStep::Launch(command) => MacroStep::execute_launch(command),
            MacroStep::ClickImage(img_name, point, allowed_diff) => MacroStep::execute_click_image(img_name, point, allowed_diff),
            MacroStep::AwaitImage(img_name, allowed_diff) => MacroStep::execute_await_image(img_name, allowed_diff),
        }
    }

    pub fn default_launch() -> MacroStep {
        MacroStep::Launch("".into())
    }

    pub fn default_click_image() -> MacroStep {
        MacroStep::ClickImage("".into(), Default::default(), 0.0)
    }

    pub fn default_await_image() -> MacroStep {
        MacroStep::AwaitImage("".into(), 0.0)
    }

    pub fn to_string(&self) -> String {
        println!("{:?}", self);
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

    fn execute_launch(command: &String) {
        ()
    }

    fn execute_click_image(img_name: &String, point: &ClickPoint, allowed_diff: &f32) {
        ()
    }

    fn execute_await_image(img_name: &String, allowed_diff: &f32) {
        ()
    }
}

impl Default for MacroStep {
    fn default() -> Self {
        Self::default_launch()
    }
}