mod ui;
mod style;
use ui::Base;
use iced::{Settings, pure::Sandbox};


fn main() {
    Base::run(Settings::default());
}
