use std::cmp::min;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use log::debug;
use crate::http_storage::HTTPStorage;
use crate::ui::ui_storage::UI;
use super::ui_storage::messages;

pub(super) struct UIEvents {
    pub(super) something_changed: bool,
    pub(super) table_state_changed: bool,
    popup_enabled: bool,
    entered_fullscreen: bool,
    confirmation: bool,
    pub(super) input_mode: bool,
    filter_enabled: bool,
}

impl Default for UIEvents {
    fn default() -> Self {
        UIEvents {
            something_changed: true,
            table_state_changed: false,
            popup_enabled: false,
            entered_fullscreen: false,
            confirmation: false,
            input_mode: false,
            filter_enabled: false,
        }
    }
}

impl UIEvents {
    pub(super) fn process_event(&mut self, ui_storage: & mut UI<'static>, http_storage: &mut HTTPStorage) -> bool {
        // Get key pressed in event
        if let Event::Key(key) = event::read().unwrap() {
            if self.filter_enabled {
                if self.input_mode {
                    match key.code {
                        KeyCode::Char(c) => {
                            ui_storage.handle_char_input(c);
                            ui_storage.show_filter();
                        },
                        KeyCode::Backspace => {
                            ui_storage.handle_backspace_input();
                            ui_storage.show_filter();
                        },
                        KeyCode::Delete => {
                            ui_storage.handle_delete_input();
                            ui_storage.show_filter();
                        },
                        KeyCode::Esc => {
                            self.input_mode = false;
                            self.popup_enabled = false;
                            self.filter_enabled = false;
                            self.table_state_changed = true;
                            self.something_changed = true;
                            ui_storage.hide_filter();
                            ui_storage.clear_input();
                            ui_storage.set_statusbar_message::<&str>(None);
                        }
                        KeyCode::Enter => {
                            self.input_mode = false;
                            self.popup_enabled = false;
                            self.filter_enabled = false;
                            self.table_state_changed = true;
                            self.something_changed = true;
                            ui_storage.save_filter();
                            ui_storage.hide_filter();
                            ui_storage.reset_table_state();
                            ui_storage.set_statusbar_message::<&str>(None);
                        },
                        KeyCode::Left => {
                            ui_storage.handle_move_cursor_left();
                        },
                        KeyCode::Right => {
                            ui_storage.handle_move_cursor_right();
                        },
                        KeyCode::End => {
                            ui_storage.handle_move_cursor_end();
                        }
                        KeyCode::Home => {
                            ui_storage.handle_move_cursor_home();
                        }
                        _ => {}
                    }
                }
                else {
                    // Possibly unused branch
                    match key.code {
                        KeyCode::Char(c) => {
                            if c == 'q' {
                                self.popup_enabled = false;
                                self.filter_enabled = false;
                                self.something_changed = true;
                                ui_storage.hide_filter();
                                ui_storage.set_statusbar_message::<&str>(None);
                            }
                            else if c == 'e' {
                                self.input_mode = true;
                                ui_storage.set_statusbar_message(Some(messages::editing_tips()));
                            }
                        },
                        KeyCode::Enter => {
                            self.popup_enabled = false;
                            self.filter_enabled = false;
                            ui_storage.save_filter();
                        },
                        _ => {}
                    }
                }
            }
            else if self.confirmation {
                match key.code {
                    KeyCode::Char(c) => {
                        if c == 'y' && self.confirmation {
                            ui_storage.hide_confirmation();
                            self.confirmation = false;
                            return true;
                        }
                        else if c == 'n' && self.confirmation {
                            ui_storage.hide_confirmation();
                            self.confirmation = false;
                            ui_storage.set_statusbar_message::<&str>(None);
                            return false;
                        }
                    }
                    _ => {}
                }
            }
            else {
                // Matching code of pressed key
                match key.code {
                    // If key pressed is character (printable)
                    KeyCode::Char(c) => {
                        // Quit action
                        if c == 'q' {
                            if self.popup_enabled {
                                // TODO make in a beautiful way
                                ui_storage.hide_help();
                                ui_storage.hide_errors();
                                ui_storage.hide_filter();
                                ui_storage.hide_repeater();
                                self.something_changed = true;
                                self.popup_enabled = false;
                            }
                            else {
                                ui_storage.show_confirmation("Are you sure you want to exit?");
                                self.confirmation = true;
                                ui_storage.set_statusbar_message(Some(messages::confirmation_tips()));
                            }
                        }
                        // Show help popup if no popups shown
                        else if c == '?' && ! self.popup_enabled {
                            ui_storage.show_help();
                            self.popup_enabled = true;
                            self.something_changed = true;
                        }
                        // Show errors popup if no popups shown
                        else if c == 'e' && ! self.popup_enabled {
                            ui_storage.show_errors();
                            self.popup_enabled = true;
                            self.something_changed = true;
                        }
                        // Select (activate) 'Request' window
                        else if c == 'r' && ! self.popup_enabled {
                            if self.entered_fullscreen { ui_storage.cancel_fullscreen() }
                            ui_storage.activate_request();
                            self.something_changed = true;
                            self.table_state_changed = true;
                            if self.entered_fullscreen { ui_storage.show_fullscreen() }
                        }
                        // Select (activate) 'Response' window
                        else if c == 's' && ! self.popup_enabled {
                            if self.entered_fullscreen { ui_storage.cancel_fullscreen() }
                            ui_storage.activate_response();
                            self.something_changed = true;
                            self.table_state_changed = true;
                            if self.entered_fullscreen { ui_storage.show_fullscreen() }
                        }
                        // Select (activate) 'Proxy' window
                        else if c == 'p' && ! self.popup_enabled {
                            if self.entered_fullscreen { ui_storage.cancel_fullscreen() }
                            ui_storage.activate_proxy();
                            self.something_changed = true;
                            self.table_state_changed = true;
                            if self.entered_fullscreen { ui_storage.show_fullscreen() }
                        }
                        else if c == 'f' && ! self.popup_enabled {
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
                        else if c == 'F' && ! self.popup_enabled {
                            ui_storage.show_filter();
                            ui_storage.set_statusbar_message(Some(messages::filter_shown()));
                            self.popup_enabled = true;
                            self.filter_enabled = true;
                            self.input_mode = true;
                            self.something_changed = true;
                        }
                        else if c == 'u' && (ui_storage.is_response_active() || ui_storage.is_request_active()) {
                            ui_storage.reveal_body();
                            self.something_changed = true;
                        }
                        else if c == 'S' {
                            // ui_storage.sort_table()
                        }
                        else if c == 'R' && ! self.popup_enabled {
                            self.something_changed = true;
                            self.popup_enabled = true;
                            ui_storage.show_repeater(http_storage);
                        }
                    }
                    KeyCode::Up => {
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
                    },
                    KeyCode::Down => {
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
                    },
                    KeyCode::PageUp => {
                        if ! self.popup_enabled {
                            if ui_storage.is_table_active() {
                                ui_storage.table_scroll_page_up(http_storage);
                                self.something_changed = true;
                                self.table_state_changed = true;
                            }
                        }
                    },
                    KeyCode::PageDown => {
                        if !self.popup_enabled {
                            if ui_storage.is_table_active() {
                                ui_storage.table_scroll_page_down(http_storage);
                                self.something_changed = true;
                                self.table_state_changed = true;
                            }
                        }
                    },
                    KeyCode::Home => {
                        if ! self.popup_enabled {
                            if ui_storage.is_table_active() {
                                ui_storage.table_scroll_home(http_storage);
                                self.something_changed = true;
                                self.table_state_changed = true;
                            }
                        }
                    },
                    KeyCode::End => {
                        if ! self.popup_enabled {
                            if ui_storage.is_table_active() {
                                ui_storage.table_scroll_end(http_storage);
                                self.something_changed = true;
                                self.table_state_changed = true;
                            }
                        }
                    },
                    _ => {}
                }
            }

        }
        else {
            return false;
        }

        return false;
    }

    pub(super) fn something_changed(&mut self) {
        self.something_changed = true;
    }
}