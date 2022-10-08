mod ui;
mod macro_logic;

use ui::Base;
use iced::{Settings, pure::Sandbox};


fn main() {
    Base::run(Settings::default());
}
