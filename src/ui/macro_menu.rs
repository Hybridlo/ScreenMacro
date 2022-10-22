use std::sync::Arc;
use std::sync::Mutex;

use anyhow::anyhow;
use anyhow::Result;
use iced::Alignment;
use iced::Command;
use iced::Length;
use iced::alignment::Horizontal;
use iced::alignment::Vertical;
use iced::pure::{Element, column, row, container, text, scrollable, button, toggler, text_input};
use iced_aw::pure::{Card, Modal};
use rfd::FileDialog;

use crate::macro_logic::Macro;
use crate::macro_logic::MacroStep;

use super::components::macro_step_component;
use super::style::BorderedContainer;
use super::style::PlusButton;

#[derive(Default)]
pub struct MacroMenu {
    pub macro_data: Macro,
    show_settings: bool,
    is_modified: bool,
    show_confimation: bool,
    macro_should_run: Arc<Mutex<bool>>,
    macro_is_running: Arc<Mutex<bool>>,
}

#[derive(Debug, Clone)]
pub enum MacroMenuMessage {
    NewVal(MacroStep, usize),
    Removed(usize),
    Add,
    EmitError(String),
    BackPressedUnsaved,
    BackDismiss,
    BackConfirmed,
    PlayPressed,
    MacroDone,
    SettingsShow,
    SettingsUpdateBreakWhileMacro(bool),
    SettingsUpdateStepTimeout(String),
    SettingsDismiss,
    SavePressed
}

impl MacroMenu {
    pub fn update(&mut self, msg: MacroMenuMessage) -> Result<Command<MacroMenuMessage>> {
        match msg {
            MacroMenuMessage::NewVal(val, index) => {
                self.macro_data.macro_steps.splice(index..index+1, [val]);
                self.is_modified = true
            },
            MacroMenuMessage::Removed(index) => {
                self.macro_data.macro_steps.remove(index);
                self.is_modified = true
            },
            MacroMenuMessage::Add => {
                self.macro_data.macro_steps.push(Default::default());
                self.is_modified = true;
            },
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
            MacroMenuMessage::BackPressedUnsaved => self.show_confimation = true,
            MacroMenuMessage::BackDismiss => self.show_confimation = false,
            MacroMenuMessage::BackConfirmed => { // processed here and above
                self.macro_data = Default::default();
                self.show_confimation = false;
                self.is_modified = false;
            },
            MacroMenuMessage::MacroDone => *(self.macro_should_run.lock().unwrap()) = false,
            MacroMenuMessage::SettingsShow => self.show_settings = true,
            MacroMenuMessage::SettingsUpdateBreakWhileMacro(break_whole_macro) => {
                self.macro_data.settings.break_whole_macro = break_whole_macro;
                self.is_modified = true;
            },
            MacroMenuMessage::SettingsUpdateStepTimeout(text) => {
                if text == "" {
                    self.macro_data.settings.step_timeout_seconds = 0;
                }
                
                if let Ok(num_res) = text.parse() {
                    self.macro_data.settings.step_timeout_seconds = num_res;
                }

                self.is_modified = true;
            },
            MacroMenuMessage::SettingsDismiss => self.show_settings = false,
            MacroMenuMessage::SavePressed => {
                let path = FileDialog::new()
                                                    .add_filter("ScreenMacro binary file", &["smbf"])
                                                    .save_file()
                                                    .ok_or(anyhow!("Failed to save"))?;

                self.macro_data.macro_name = path.file_stem()
                                                 .ok_or(anyhow!("Invalid filename"))?
                                                 .to_str()
                                                 .ok_or(anyhow!("Invalid filename"))?
                                                 .to_string();

                self.macro_data.save_file(&path)?;

                self.is_modified = false;
            },
        }

        Ok(Command::none())
    }

    pub fn view(&self) -> Element<MacroMenuMessage> {
        let content = row()
        .push(
           self.macro_container() 
        ).push(
            container(
                self.side_panel()
            )
            .width(Length::FillPortion(2))
        ).height(Length::Fill);

        let settings_modal = Modal::new(self.show_settings, content, || {
            self.settings_card()
        })
        .backdrop(MacroMenuMessage::SettingsDismiss)
        .on_esc(MacroMenuMessage::SettingsDismiss);

        return Modal::new(self.show_confimation, settings_modal, || {
            self.confirm_card()
        })
        .backdrop(MacroMenuMessage::BackDismiss)
        .on_esc(MacroMenuMessage::BackDismiss)
        .into()
    }

    fn confirm_card(&self) -> Element<MacroMenuMessage> {
        Card::new(
            text("There are unsaved changes"),
            column().push(
                container(
                    text("Are you sure you want to exit?")
                    .horizontal_alignment(Horizontal::Center)
                )
                .width(Length::Fill)
                .center_x()
            ).push(
                container(
                    text("There are unsaved changes in your macro")
                    .horizontal_alignment(Horizontal::Center)
                )
                .width(Length::Fill)
                .center_x()
            )
            .spacing(5)
        )
        .foot(
            container(
                row().push(
                    container(
                        button(
                            text("Yes")
                        )
                        .on_press(MacroMenuMessage::BackConfirmed)
                    )
                    .width(Length::Fill)
                    .center_x()
                ).push(
                    container(
                        button(
                            text("Go back")
                        )
                        .on_press(MacroMenuMessage::BackDismiss)
                    )
                    .width(Length::Fill)
                    .center_x()
                )
            )
            .center_x()
            .width(Length::Fill)
        )
        .max_width(300)
        .on_close(MacroMenuMessage::BackDismiss)
        .into()
    }

    fn settings_card(&self) -> Element<MacroMenuMessage> {
        let settings_body = column().push(
            toggler(
                Some("Stop whole macro on timeout".to_string()), 
                self.macro_data.settings.break_whole_macro, 
                MacroMenuMessage::SettingsUpdateBreakWhileMacro
            )
        ).push(
            row().push(
                text("Timeout in seconds")
            ).push(
                self.my_numeric_input()
            )
        )
        .spacing(7);

        Card::new(
            text("Settings"),
            settings_body
        )
        .foot(
            container(
                button(
                    text(
                        "Ok"
                    )
                )
                .on_press(MacroMenuMessage::SettingsDismiss)
            )
            .center_x()
            .width(Length::Fill)
        )
        .max_width(300)
        .on_close(MacroMenuMessage::SettingsDismiss)
        .into()
    }

    /// because Modal is quirky like that and panics if you provide a custom component
    fn my_numeric_input(&self) -> Element<MacroMenuMessage> {
        row().push(
            text_input(
                "seconds",
                &format!("{}", self.macro_data.settings.step_timeout_seconds),
                MacroMenuMessage::SettingsUpdateStepTimeout
            )
        )
        .into()
    }

    fn macro_container(&self) -> Element<MacroMenuMessage> {
        container(scrollable(
            container(
                self.macro_ui()
            )
            .style(BorderedContainer::Nothing)
            .padding(15)
            .width(Length::Fill)
        ).scrollbar_margin(4))
        .width(Length::FillPortion(6))
        .height(Length::Fill)
        .into()
    }

    fn macro_ui(&self) -> Element<MacroMenuMessage> {
        let mut name_text = self.macro_data.macro_name.clone();

        if self.is_modified {
            name_text += "*";
        }

        let mut macro_ui = column().push(
            container(
                text(
                    name_text
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

        macro_ui.push(
            button(
                text("+").size(24)
            )
            .style(PlusButton::Normal)
            .width(Length::Shrink)
            .height(Length::Shrink)
            .on_press(MacroMenuMessage::Add)
        )
        .into()
    }

    fn side_panel(&self) -> Element<MacroMenuMessage> {
        let mut back_button = button(
            text("Back")
        );

        if !*(self.macro_is_running.lock().unwrap()) && !*(self.macro_should_run.lock().unwrap()) {
            if self.is_modified {
                back_button = back_button.on_press(MacroMenuMessage::BackPressedUnsaved)
            } else {
                back_button = back_button.on_press(MacroMenuMessage::BackConfirmed)
            }
        }

        column().push(
            container(
                column().push(
                    self.run_stop_button()
                ).push(
                    button(
                        text("Settings")
                    )
                    .on_press(MacroMenuMessage::SettingsShow)
                ).push(
                    button(
                        text("Save as")
                    )
                    .on_press(MacroMenuMessage::SavePressed)
                )
                .spacing(10)
                .align_items(Alignment::Center)
            )
            .height(Length::Shrink)
        ).push(
            container(
                column().push(
                    back_button
                )
                .align_items(Alignment::Center)
                .spacing(10)
            )
            .height(Length::Fill)
            .align_y(Vertical::Bottom)
        )
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .padding(20)
        .into()
    }

    fn run_stop_button(&self) -> Element<MacroMenuMessage> {
        let mut run_stop_button = button(
            text(if *(self.macro_should_run.lock().unwrap()) && *(self.macro_is_running.lock().unwrap()) { "Stop" } else { "Play" })
        );

        // values 1 and 0 - it should be running, but isn't yet, so shouldn't be clickable (although they become 1 1 together, so 1 0 should be impossible)
        // values 0 and 1 - it should not be running, but it's not done yet, so shoudn't be clickable
        if !(*(self.macro_should_run.lock().unwrap()) ^ *(self.macro_is_running.lock().unwrap())) {
            run_stop_button = run_stop_button.on_press(MacroMenuMessage::PlayPressed)
        }

        run_stop_button.into()
    }
}