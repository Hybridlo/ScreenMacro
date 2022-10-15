mod ui;
mod macro_logic;

use anyhow::Result;
use ui::Base;
use iced::{Settings, pure::Sandbox};


fn main() -> Result<()> {
    Base::run(Settings::default())?;

    Ok(())
}
