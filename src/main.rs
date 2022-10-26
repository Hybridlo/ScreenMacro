#![windows_subsystem = "windows"]

mod ui;
mod macro_logic;

use anyhow::Result;
use ui::Base;
use iced::{Settings, pure::Application};


fn main() -> Result<()> {
    Base::run(Settings::default())?;

    Ok(())
}
