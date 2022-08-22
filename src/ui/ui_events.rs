use crossterm::event;
use crossterm::event::{Event, KeyCode};
use crate::http_storage::HTTPStorage;
use crate::ui::ui_storage::UI;

pub(super) struct UIEvents {
    pub(super) something_changed: bool,
    pub(super) table_state_changed: bool,
    pub(super) help_enabled: bool,
    pub(super) entered_fullscreen: bool,
}

impl Default for UIEvents {
    fn default() -> Self {
        UIEvents {
            something_changed: true,
            table_state_changed: false,
            help_enabled: false,
            entered_fullscreen: false
        }
    }
}

impl UIEvents {
    pub(super) fn process_event(&mut self, ui_storage: & mut UI<'static>, http_storage: &mut HTTPStorage) -> bool {
        if let Event::Key(key) = event::read().unwrap() {
            if let KeyCode::Char('q') = key.code {
                if self.help_enabled {
                    ui_storage.hide_help();
                    self.something_changed = true;
                    self.help_enabled = false;
                }
                else {
                    return true;
                }
            }
            else if let KeyCode::Up = key.code {
                let index = match ui_storage.proxy_history_state.selected() {
                    Some(i) => if i == 0 { 0 } else { i - 1 },
                    None => 0 as usize
                };

                ui_storage.proxy_history_state.select(Some(index));
                self.table_state_changed = true;
                self.something_changed = true
            }
            else if let KeyCode::Down = key.code {
                let index = match ui_storage.proxy_history_state.selected() {
                    Some(i) => if i >= http_storage.len() - 1 { http_storage.len() - 1 } else { i + 1 },
                    None => 0 as usize
                };

                ui_storage.proxy_history_state.select(Some(index));
                self.table_state_changed = true;
                self.something_changed = true
            }
            else if let KeyCode::Char('?') = key.code {
                ui_storage.show_help();
                self.help_enabled = true;
                self.something_changed = true;
            }
            else if let KeyCode::Char('r') = key.code {
                if ! self.help_enabled {
                    if self.entered_fullscreen { ui_storage.cancel_fullscreen() }
                    ui_storage.activate_request();
                    self.something_changed = true;
                    self.table_state_changed = true;
                    if self.entered_fullscreen { ui_storage.show_fullscreen() }
                }
            }
            else if let KeyCode::Char('s') = key.code {
                if ! self.help_enabled {
                    if self.entered_fullscreen { ui_storage.cancel_fullscreen() }
                    ui_storage.activate_response();
                    self.something_changed = true;
                    self.table_state_changed = true;
                    if self.entered_fullscreen { ui_storage.show_fullscreen() }
                }
            }
            else if let KeyCode::Char('p') = key.code {
                if ! self.help_enabled {
                    if self.entered_fullscreen { ui_storage.cancel_fullscreen() }
                    ui_storage.activate_proxy();
                    self.something_changed = true;
                    self.table_state_changed = true;
                    if self.entered_fullscreen { ui_storage.show_fullscreen() }
                }
            }
            else if let KeyCode::Char('f') = key.code {
                if ! self.help_enabled {
                    if self.entered_fullscreen {
                        ui_storage.cancel_fullscreen();
                        self.something_changed = true;
                        self.table_state_changed = true;
                        self.entered_fullscreen = false;
                    }
                    else {
                        ui_storage.show_fullscreen();
                        self.something_changed = true;
                        self.table_state_changed = true;
                        self.entered_fullscreen = true;
                    }
                }
            }
        }

        return false;
    }
}