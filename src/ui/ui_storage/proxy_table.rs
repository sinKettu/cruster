use std::cmp::{max, min};
use http::header::HeaderName;
use http::HeaderValue;
use log::debug;
use tui::layout::{Alignment, Constraint, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders, Row, Table};
use crate::http_storage::HTTPStorage;
use crate::ui::ui_storage::render_units::RenderUnit;
use super::UI;

impl UI<'static> {
    pub(crate) fn make_table(&mut self, storage: &mut HTTPStorage, size: Rect) {
        if storage.len() == 0 {
            return
        }

        debug!("Making table with width {} and height {}", size.width, size.height);

        let re = match self.filter.as_ref() {
            // Try to create regex if there is some filter
            Some(f) => {
                match regex::Regex::new(f) {
                    // Save regex option if creation was successful
                    Ok(re) => {
                        Some(re)
                    },
                    // Otherwise log error, clear filter and set None as filter
                    Err(e) => {
                        self.log_error(e.into());
                        self.filter = None;
                        None
                    }
                }
            }
            // No regex if no filter string
            None => {
                None
            }
        };

        let cache = storage.get_cached_data(
            self.table_start_index,
            self.table_window_size,
            re,
            storage.len() <= self.table_window_size
        );

        let mut rows: Vec<Row> = Vec::new();
        for pair in cache {
            let index = pair.index;
            let request = pair.request.as_ref().unwrap();

            let (code, length) = match pair.response.as_ref() {
                Some(response) => {
                    (response.status.clone(), response.get_length())
                }
                None => {
                    ("".to_string(), "".to_string())
                }
            };

            let row = vec![
                // Number of record in table
                (index + 1).to_string(),
                // Method
                request.method.clone(),
                // Host
                request.get_host(),
                // Path
                request.get_request_path(),
                // HTTP Status Code
                code,
                // Response Body Length
                length
            ];
            rows.push(Row::new(row));
        }

        let header_style = if self.proxy_block == self.active_widget {
            self.active_widget_header_style
        }
        else {
            self.default_widget_header_style
        };

        let proxy_history_table = Table::new(rows)
            .header(Row::new(vec!["№", "Method", "Host", "Path", "Code", "Length"]))
            .style(Style::default().fg(Color::White))
            .widths(&[
                // Index
                Constraint::Percentage(5),
                // Method
                Constraint::Percentage(5),
                // Host
                Constraint::Percentage(10),
                // Path
                Constraint::Percentage(60),
                // Status Code
                Constraint::Percentage(10),
                // Length
                Constraint::Percentage(5)
            ])
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD))
            .block(
                Block::default()
                    .title(Span::styled("Proxy History", header_style))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL));

        self.widgets[self.proxy_block] = RenderUnit::new_table(
            proxy_history_table,
            self.proxy_area,
            true
        );
    }

    pub(crate) fn table_step_down(&mut self, storage: &HTTPStorage) {
        debug!("table_step_down_1: start_index - {}, end_index - {}, state - {:?}", self.table_start_index, self.table_end_index, self.proxy_history_state.selected());
        let end = self.table_end_index.clone();
        let window = self.table_window_size.clone();
        let storage_len = storage.len();
        let cache_len = storage.cache_len();
        match self.proxy_history_state.selected() {
            Some(i) => {
                let initial_index = i;

                if cache_len == 0 {
                    self.proxy_history_state.select(None);
                }
                else if initial_index < cache_len - 1 {
                    self.proxy_history_state.select(Some(initial_index + 1))
                }
                else {
                    if self.table_end_index + 1 < storage_len {
                        self.table_start_index += 1;
                        self.table_end_index += 1;
                    }
                }
                //
                //
                //
                // if storage_len < window && i == storage_len - 1 {
                //     self.proxy_history_state.select(
                //         Some(storage_len.saturating_sub(1))
                //     );
                // }
                // else if i == window - 1 && end >= storage_len - 1 {
                //     self.table_end_index = storage_len.saturating_sub(1);
                //     self.table_start_index = (self.table_end_index + 1).saturating_sub(window);
                //     self.proxy_history_state.select(Some(min(storage_len, window).saturating_sub(1)));
                // }
                // else if i == window - 1 && end < storage_len - 1 {
                //     self.table_end_index += 1;
                //     self.table_start_index += 1;
                // }
                // else {
                //     self.proxy_history_state.select(Some(i + 1));
                // }
                // let final_index = self.proxy_history_state.selected().unwrap();
                // if final_index != initial_index {
                //     self.reset_scrolling();
                // }
            },
            None => {
                if storage.len() > 0 {
                    self.proxy_history_state.select(Some(0));
                }
            }
        }
        debug!("table_step_down_2: start_index - {}, end_index - {}, state - {:?}", self.table_start_index, self.table_end_index, self.proxy_history_state.selected());
    }

    pub(crate) fn table_step_up(&mut self, storage: &HTTPStorage) {
        debug!("table_step_up_1: start_index - {}, end_index - {}, state - {:?}", self.table_start_index, self.table_end_index, self.proxy_history_state.selected());
        let start = self.table_start_index.clone();
        let window = self.table_window_size.clone();
        let storage_len = storage.len();
        let cache_len = storage.cache_len();
        match self.proxy_history_state.selected() {
            Some(i) => {
                let initial_index = i;

                if cache_len == 0 {
                    self.proxy_history_state.select(None);
                }
                else if initial_index > 0 {
                    self.proxy_history_state.select(Some(initial_index.saturating_sub(1)));
                }
                else {
                    if start > 0 {
                        self.table_start_index -= 1;
                        self.table_end_index -= 1;
                    }
                }
                //
                //
                //
                // if i == 0 && start == 0 {
                //     self.table_end_index = window - 1;
                // }
                // else if i == 0 && start > 0 {
                //     self.table_end_index -= 1;
                //     self.table_start_index -= 1;
                // }
                // else {
                //     self.proxy_history_state.select(Some(i.saturating_sub(1)));
                // }
                // let final_index = self.proxy_history_state.selected().unwrap();
                // if initial_index != final_index {
                //     self.reset_scrolling();
                // }
            },
            None => {
                if storage.len() > 0 {
                    self.proxy_history_state.select(Some(0));
                }
            }
        }
        debug!("table_step_up_2: start_index - {}, end_index - {}, state - {:?}", self.table_start_index, self.table_end_index, self.proxy_history_state.selected());
    }

    pub(crate) fn table_scroll_page_down(&mut self, storage: &HTTPStorage) {
        debug!("table_scroll_page_down_1: start_index - {}, end_index - {}, state - {:?}", self.table_start_index, self.table_end_index, self.proxy_history_state.selected());
        if self.table_end_index == storage.len() - 1 {
            let new_state = min(self.table_window_size - 1, storage.len() - 1);
            self.proxy_history_state.select(Some(new_state));
            self.table_start_index = (self.table_end_index + 1).saturating_sub(self.table_window_size);
        } else if self.table_end_index + self.table_window_size >= storage.len() - 1 {
            self.table_end_index = storage.len() - 1;
            self.table_start_index = (self.table_end_index + 1).saturating_sub(self.table_window_size);
            // let new_state = min(self.table_window_size - 1, storage.len() - 1);
            // self.proxy_history_state.select(Some(new_state));
        } else {
            self.table_start_index += self.table_window_size;
            self.table_end_index += self.table_window_size;
        }
        debug!("table_scroll_page_down_2: start_index - {}, end_index - {}, state - {:?}", self.table_start_index, self.table_end_index, self.proxy_history_state.selected());
    }

    pub(crate) fn table_scroll_page_up(&mut self, storage: &HTTPStorage) {
        debug!("table_scroll_page_up_1: start_index - {}, end_index - {}, state - {:?}", self.table_start_index, self.table_end_index, self.proxy_history_state.selected());
        if self.table_start_index == 0 {
            self.proxy_history_state.select(Some(0));
            self.table_end_index = min(self.table_window_size, storage.len()).saturating_sub(1);
        } else if self.table_start_index.saturating_sub(self.table_window_size) == 0 {
            self.table_start_index = 0;
            self.table_end_index = min(self.table_window_size, storage.len()).saturating_sub(1);
            // self.proxy_history_state.select(Some(0));
        } else {
            self.table_start_index -= self.table_window_size;
            self.table_end_index -= self.table_window_size;
        }
        debug!("table_scroll_page_up_2: start_index - {}, end_index - {}, state - {:?}", self.table_start_index, self.table_end_index, self.proxy_history_state.selected());
    }

    pub(crate) fn table_scroll_end(&mut self, storage: &HTTPStorage) {
        debug!(
            "table_scroll_end_1: storage_len - {}, end_index -  {}, selected - {:?}",
            storage.len(),
            self.table_end_index,
            self.proxy_history_state.selected());

        if self.proxy_history_state.selected().is_none() {
            return;
        }

        self.table_end_index = max(storage.len(), self.table_window_size).saturating_sub(1);
        self.table_start_index = (self.table_end_index + 1).saturating_sub(self.table_window_size);
        let new_state = min(self.table_window_size, storage.len()).saturating_sub(1);
        self.proxy_history_state.select(Some(new_state));
        self.reset_scrolling();

        debug!(
            "table_scroll_end_2: storage_len - {}, end_index -  {}, selected - {:?}",
            storage.len(),
            self.table_end_index,
            self.proxy_history_state.selected());
    }

    pub(crate) fn table_scroll_home(&mut self, storage: &HTTPStorage) {
        debug!(
            "table_scroll_home_1: storage_len - {}, end_index -  {}, selected - {:?}",
            storage.len(),
            self.table_end_index,
            self.proxy_history_state.selected());

        if self.proxy_history_state.selected().is_none() {
            return;
        }

        self.table_start_index = 0;
        self.table_end_index = self.table_window_size - 1;
        self.proxy_history_state.select(Some(0));
        self.reset_scrolling();

        debug!(
            "table_scroll_home_2: storage_len - {}, end_index -  {}, selected - {:?}",
            storage.len(),
            self.table_end_index,
            self.proxy_history_state.selected());
    }
}