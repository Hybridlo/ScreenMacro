use std::sync::Arc;
use std::sync::Mutex;

use anyhow::anyhow;
use anyhow::Result;
use iced::Alignment;
use iced::Command;
use iced::Length;
use iced::alignment::Vertical;
use iced::pure::{Element, column, row, container, text, scrollable, button};

use crate::macro_logic::Macro;
use crate::macro_logic::MacroStep;

use super::components::macro_step_component;
use super::style::BorderedContainer;
use super::style::PlusButton;

#[derive(Default)]
pub struct MacroMenu {
    pub macro_data: Macro,
    macro_should_run: Arc<Mutex<bool>>,
    macro_is_running: Arc<Mutex<bool>>,
}

#[derive(Debug, Clone)]
pub enum MacroMenuMessage {
    NewVal(MacroStep, usize),
    Removed(usize),
    Add,
    EmitError(String),
    BackPressed,
    PlayPressed,
    MacroDone
}

impl MacroMenu {
    pub fn update(&mut self, msg: MacroMenuMessage) -> Result<Command<MacroMenuMessage>> {
        match msg {
            MacroMenuMessage::NewVal(val, index) => _ = self.macro_data.macro_steps.splice(index..index+1, [val]),

            MacroMenuMessage::Removed(index) => _ = self.macro_data.macro_steps.remove(index),

            MacroMenuMessage::Add => _ = self.macro_data.macro_steps.push(Default::default()),
            
            MacroMenuMessage::EmitError(error) => {
                *(self.macro_should_run.lock().unwrap()) = false;
                return Err(anyhow!(error))
            },
            MacroMenuMessage::PlayPressed => {
                let curr_state = *(self.macro_should_run.lock().unwrap());
                match curr_state {
                    true => {
                        *(self.macro_should_run.lock().unwrap()) = false;
                    },
                    false => {
                        *(self.macro_should_run.lock().unwrap()) = true;
                        *(self.macro_is_running.lock().unwrap()) = true;    // toggled to false inside the async task when it's done
                                                                            // so we know when the task reacted to stop signal
                        return Ok(Command::perform(
                            Macro::execute_macro(
                                self.macro_data.clone(),
                                self.macro_should_run.clone(),
                                self.macro_is_running.clone()
                            ),
                            |res| {
                                if let Err(err) = res {
                                    MacroMenuMessage::EmitError(err.to_string())
                                } else {
                                    MacroMenuMessage::MacroDone
                                }
                            }
                        ))
                    },
                }
            },
            MacroMenuMessage::BackPressed => self.macro_data = Default::default(), // processed here and above
            MacroMenuMessage::MacroDone => *(self.macro_should_run.lock().unwrap()) = false,
        }

        Ok(Command::none())
    }

    pub fn view(&self) -> Element<MacroMenuMessage> {
        let mut macro_ui = column().push(
            container(
                text(
                    self.macro_data.macro_name.clone()
                )
                .size(42)
            )
            .padding(5)
        );

        for (i, macro_step) in self.macro_data.macro_steps.iter().enumerate() {
            macro_ui = macro_ui.push(
                macro_step_component(
                    i,
                    Some(macro_step.clone()),
                    MacroMenuMessage::NewVal,
                    MacroMenuMessage::Removed,
                    MacroMenuMessage::EmitError
                )
            )
        }

        let macro_container = container(scrollable(
            container(
                macro_ui
                    .push(
                        button(
                            text("+").size(24)
                        )
                        .style(PlusButton::Normal)
                        .width(Length::Shrink)
                        .height(Length::Shrink)
                        .on_press(MacroMenuMessage::Add)
                    )
            )
            .style(BorderedContainer::Nothing)
            .padding(15)
            .width(Length::Fill)
        ).scrollbar_margin(4))
        .width(Length::FillPortion(6))
        .height(Length::Fill);

        let mut run_stop_button = button(
            text(if *(self.macro_should_run.lock().unwrap()) && *(self.macro_is_running.lock().unwrap()) { "stop" } else { "play" })
        );

        // values 1 and 0 - it should be running, but isn't yet, so shouldn't be clickable (although they become 1 and 1 together, so 1 0 should be impossible)
        // values 0 and 1 - it should not be running, but it's not done yet, so shoudn't be clickable
        if !(*(self.macro_should_run.lock().unwrap()) ^ *(self.macro_is_running.lock().unwrap())) {
            run_stop_button = run_stop_button.on_press(MacroMenuMessage::PlayPressed)
        }

        let side_panel = column().push(
            container(
                column().push(
                    run_stop_button
                )
            )
            .height(Length::Shrink)
        ).push(
            container(
                column().push(
                    button(
                        text("Back")
                    )
                    .on_press(MacroMenuMessage::BackPressed)
                )
            )
            .height(Length::Fill)
            .align_y(Vertical::Bottom)
        )
        .width(Length::Fill)
        .align_items(Alignment::Center);

        row()
        .push(
           macro_container 
        ).push(
            container(
                side_panel
            )
            .width(Length::FillPortion(2))
        ).height(Length::Fill)
        .into()
    }
}