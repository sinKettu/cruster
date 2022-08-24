use std::cmp::min;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use log::debug;
use crate::http_storage::HTTPStorage;
use crate::ui::ui_storage::UI;

pub(super) struct UIEvents {
    pub(super) something_changed: bool,
    pub(super) table_state_changed: bool,
    pub(super) popup_enabled: bool,
    pub(super) entered_fullscreen: bool,
}

impl Default for UIEvents {
    fn default() -> Self {
        UIEvents {
            something_changed: true,
            table_state_changed: false,
            popup_enabled: false,
            entered_fullscreen: false,
        }
    }
}

impl UIEvents {
    pub(super) fn process_event(&mut self, ui_storage: & mut UI<'static>, http_storage: &mut HTTPStorage) -> bool {
        if let Event::Key(key) = event::read().unwrap() {
            if let KeyCode::Char('q') = key.code {
                if self.popup_enabled {
                    // TODO make in a beautiful way
                    ui_storage.hide_help();
                    ui_storage.hide_errors();
                    self.something_changed = true;
                    self.popup_enabled = false;
                }
                else {
                    return true;
                }
            }
            else if let KeyCode::Up = key.code {
                if ui_storage.is_table_active() {
                   ui_storage.table_step_up(http_storage);
                    self.table_state_changed = true;
                    self.something_changed = true
                }
                else if ui_storage.is_request_active() {
                    ui_storage.scroll_request(Some(-5), None);
                    self.something_changed = true;
                }
                else if ui_storage.is_response_active() {
                    ui_storage.scroll_response(Some(-5), None);
                    self.something_changed = true;
                }
            }
            else if let KeyCode::Down = key.code {
                if ui_storage.is_table_active() {
                    ui_storage.table_step_down(http_storage);
                    self.table_state_changed = true;
                    self.something_changed = true
                }
                else if ui_storage.is_request_active() {
                    ui_storage.scroll_request(Some(5), None);
                    self.something_changed = true;
                }
                else if ui_storage.is_response_active() {
                    ui_storage.scroll_response(Some(5), None);
                    self.something_changed = true;
                }
            }
            else if let KeyCode::PageDown = key.code {
                if !self.popup_enabled {
                    if ui_storage.is_table_active() {
                        ui_storage.table_scroll_page_down(http_storage);
                        self.something_changed = true;
                        self.table_state_changed = true;
                    }
                }
            }
            else if let KeyCode::PageUp = key.code {
                if ! self.popup_enabled {
                    if ui_storage.is_table_active() {
                        ui_storage.table_scroll_page_up(http_storage);
                        self.something_changed = true;
                        self.table_state_changed = true;
                    }
                }
            }
            else if let KeyCode::End = key.code {
                debug!("process_event: End hit");
                if ! self.popup_enabled {
                    if ui_storage.is_table_active() {
                        ui_storage.table_scroll_end(http_storage);
                        self.something_changed = true;
                        self.table_state_changed = true;
                    }
                }
            }
            else if let KeyCode::Home = key.code {
                debug!("process_event: Home hit");
                if ! self.popup_enabled {
                    if ui_storage.is_table_active() {
                        ui_storage.table_scroll_home(http_storage);
                        self.something_changed = true;
                        self.table_state_changed = true;
                    }
                }
            }
            else if let KeyCode::Char('?') = key.code {
                if ! self.popup_enabled {
                    ui_storage.show_help();
                    self.popup_enabled = true;
                    self.something_changed = true;
                }
            }
            else if let KeyCode::Char('e') = key.code {
                if ! self.popup_enabled {
                    ui_storage.show_errors();
                    self.popup_enabled = true;
                    self.something_changed = true;
                }
            }
            else if let KeyCode::Char('r') = key.code {
                if ! self.popup_enabled {
                    if self.entered_fullscreen { ui_storage.cancel_fullscreen() }
                    ui_storage.activate_request();
                    self.something_changed = true;
                    self.table_state_changed = true;
                    if self.entered_fullscreen { ui_storage.show_fullscreen() }
                }
            }
            else if let KeyCode::Char('s') = key.code {
                if ! self.popup_enabled {
                    if self.entered_fullscreen { ui_storage.cancel_fullscreen() }
                    ui_storage.activate_response();
                    self.something_changed = true;
                    self.table_state_changed = true;
                    if self.entered_fullscreen { ui_storage.show_fullscreen() }
                }
            }
            else if let KeyCode::Char('p') = key.code {
                if ! self.popup_enabled {
                    if self.entered_fullscreen { ui_storage.cancel_fullscreen() }
                    ui_storage.activate_proxy();
                    self.something_changed = true;
                    self.table_state_changed = true;
                    if self.entered_fullscreen { ui_storage.show_fullscreen() }
                }
            }
            else if let KeyCode::Char('f') = key.code {
                if ! self.popup_enabled {
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