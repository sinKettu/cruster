use std::cmp::min;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use log::debug;
use crate::http_storage::HTTPStorage;
use crate::ui::ui_storage::UI;

pub(super) struct UIEvents {
    something_changed: bool,
    pub(super) table_state_changed: bool,
    popup_enabled: bool,
    entered_fullscreen: bool,
    input_mode: bool,
}

impl Default for UIEvents {
    fn default() -> Self {
        UIEvents {
            something_changed: true,
            table_state_changed: false,
            popup_enabled: false,
            entered_fullscreen: false,
            input_mode: false,
        }
    }
}

impl UIEvents {
    pub(super) fn process_event(&mut self, ui_storage: & mut UI<'static>, http_storage: &mut HTTPStorage) -> bool {
        // Get key pressed in event
        if let Event::Key(key) = event::read().unwrap() {
            // Matching code of pressed key
            match key.code {
                // If key pressed is character (printable)
                KeyCode::Char(c) => {
                    // If user input enabled
                    if self.input_mode {

                    }
                    else {
                        // Quit action
                        if c == 'q' {
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
            // if let KeyCode::Char('q') = key.code {
            //     if self.popup_enabled {
            //         // TODO make in a beautiful way
            //         ui_storage.hide_help();
            //         ui_storage.hide_errors();
            //         self.something_changed = true;
            //         self.popup_enabled = false;
            //     }
            //     else {
            //         return true;
            //     }
            // }
            // else if let KeyCode::Up = key.code {
            //     if ui_storage.is_table_active() {
            //        ui_storage.table_step_up(http_storage);
            //         self.table_state_changed = true;
            //         self.something_changed = true
            //     }
            //     else if ui_storage.is_request_active() {
            //         ui_storage.scroll_request(Some(-5), None);
            //         self.something_changed = true;
            //     }
            //     else if ui_storage.is_response_active() {
            //         ui_storage.scroll_response(Some(-5), None);
            //         self.something_changed = true;
            //     }
            // }
            // else if let KeyCode::Down = key.code {
            //     if ui_storage.is_table_active() {
            //         ui_storage.table_step_down(http_storage);
            //         self.table_state_changed = true;
            //         self.something_changed = true
            //     }
            //     else if ui_storage.is_request_active() {
            //         ui_storage.scroll_request(Some(5), None);
            //         self.something_changed = true;
            //     }
            //     else if ui_storage.is_response_active() {
            //         ui_storage.scroll_response(Some(5), None);
            //         self.something_changed = true;
            //     }
            // }
            // else if let KeyCode::PageDown = key.code {
            //     if !self.popup_enabled {
            //         if ui_storage.is_table_active() {
            //             ui_storage.table_scroll_page_down(http_storage);
            //             self.something_changed = true;
            //             self.table_state_changed = true;
            //         }
            //     }
            // }
            // else if let KeyCode::PageUp = key.code {
            //     if ! self.popup_enabled {
            //         if ui_storage.is_table_active() {
            //             ui_storage.table_scroll_page_up(http_storage);
            //             self.something_changed = true;
            //             self.table_state_changed = true;
            //         }
            //     }
            // }
            // else if let KeyCode::End = key.code {
            //     debug!("process_event: End hit");
            //     if ! self.popup_enabled {
            //         if ui_storage.is_table_active() {
            //             ui_storage.table_scroll_end(http_storage);
            //             self.something_changed = true;
            //             self.table_state_changed = true;
            //         }
            //     }
            // }
            // else if let KeyCode::Home = key.code {
            //     debug!("process_event: Home hit");
            //     if ! self.popup_enabled {
            //         if ui_storage.is_table_active() {
            //             ui_storage.table_scroll_home(http_storage);
            //             self.something_changed = true;
            //             self.table_state_changed = true;
            //         }
            //     }
            // }
            // else if let KeyCode::Char('?') = key.code {
            //     if ! self.popup_enabled {
            //         ui_storage.show_help();
            //         self.popup_enabled = true;
            //         self.something_changed = true;
            //     }
            // }
            // else if let KeyCode::Char('e') = key.code {
            //     if ! self.popup_enabled {
            //         ui_storage.show_errors();
            //         self.popup_enabled = true;
            //         self.something_changed = true;
            //     }
            // }
            // else if let KeyCode::Char('r') = key.code {
            //     if ! self.popup_enabled {
            //         if self.entered_fullscreen { ui_storage.cancel_fullscreen() }
            //         ui_storage.activate_request();
            //         self.something_changed = true;
            //         self.table_state_changed = true;
            //         if self.entered_fullscreen { ui_storage.show_fullscreen() }
            //     }
            // }
            // else if let KeyCode::Char('s') = key.code {
            //     if ! self.popup_enabled {
            //         if self.entered_fullscreen { ui_storage.cancel_fullscreen() }
            //         ui_storage.activate_response();
            //         self.something_changed = true;
            //         self.table_state_changed = true;
            //         if self.entered_fullscreen { ui_storage.show_fullscreen() }
            //     }
            // }
            // else if let KeyCode::Char('p') = key.code {
            //     if ! self.popup_enabled {
            //         if self.entered_fullscreen { ui_storage.cancel_fullscreen() }
            //         ui_storage.activate_proxy();
            //         self.something_changed = true;
            //         self.table_state_changed = true;
            //         if self.entered_fullscreen { ui_storage.show_fullscreen() }
            //     }
            // }
            // else if let KeyCode::Char('f') = key.code {
            //     if ! self.popup_enabled {
            //         if self.entered_fullscreen {
            //             ui_storage.cancel_fullscreen();
            //             self.something_changed = true;
            //             self.table_state_changed = true;
            //             self.entered_fullscreen = false;
            //         }
            //         else {
            //             ui_storage.show_fullscreen();
            //             self.something_changed = true;
            //             self.table_state_changed = true;
            //             self.entered_fullscreen = true;
            //         }
            //     }
            // }
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