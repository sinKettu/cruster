pub(crate) mod render_units;
pub(crate) mod help;

use render_units::*;
use help::make_help_menu;
use crate::http_storage::*;

use flate2::write::GzDecoder;
use std::io::prelude::*;
use bstr::ByteSlice;

use std::{
    cmp::min,
    collections::HashMap
};
use std::borrow::{Borrow, BorrowMut};
use std::cmp::max;
use std::env::{temp_dir, var};
use std::os::macos::raw::stat;
use crossterm::style::style;
use log::debug;

use crate::cruster_proxy::request_response::{
    BodyCompressedWith,
    HyperRequestWrapper,
    HyperResponseWrapper
};

use tui::{
    widgets::{Clear, Block, Borders, Paragraph, Wrap, Table, Row, TableState, Widget},
    layout::{Alignment, Constraint},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    // Terminal,
    // text,
    // Frame,
    self
};
use tui::layout::Rect;
use tui::text::Text;
use crate::CrusterError;

const DEFAULT_PROXY_AREA: usize = 0_usize;
const DEFAULT_REQUEST_AREA: usize = 1_usize;
const DEFAULT_RESPONSE_AREA: usize = 2_usize;
const DEFAULT_STATUSBAR_AREA: usize = 3_usize;
const DEFAULT_HELP_AREA: usize = 4_usize;
const DEFAULT_ERRORS_AREA: usize = 4_usize;
const DEFAULT_CONFIRMATION_AREA: usize = 6_usize;

const DEFAULT_FULLSCREEN_AREA: usize = 5_usize;

const DEFAULT_PROXY_BLOCK: usize = 0_usize;
const DEFAULT_REQUEST_BLOCK: usize = 1_usize;
const DEFAULT_RESPONSE_BLOCK: usize = 2_usize;
const DEFAULT_STATUSBAR_BLOCK: usize = 3_usize;
const DEFAULT_HELP_BLOCK: usize = 4_usize;
const DEFAULT_ERRORS_BLOCK: usize = 5_usize;
const DEFAULT_CONFIRMATION_BLOCK: usize = 4_usize;

const HEADER_NAME_COLOR: Color = Color::LightBlue;


pub(crate) struct UI<'ui_lt> {
    // 0 - Rect for requests log,
    // 1 - Rect for requests
    // 2 - Rect for responses
    // 3 - Rect for statusbar
    // 4 - Rect for help menu
    pub(crate) widgets: Vec<RenderUnit<'ui_lt>>,

    // List of error messages
    errors: Vec<Spans<'ui_lt>>,

    // Position of block with proxy history in rectangles array (see ui/cruster_proxy:new:ui)
    proxy_area: usize,

    // Position of block with request data
    request_area: usize,

    // Position of block with response data
    response_area: usize,

    // Statusbar area index,
    statusbar_area: usize,

    // Position of area for help message in rectangles array
    help_area: usize,

    // Position of area for errors list in rectangles array
    errors_area: usize,

    // Position of confirm window (rect)
    confirm_area: usize,

    // State of table with proxy history
    pub(crate) proxy_history_state: TableState,

    // Index of block with proxy history in vector above
    proxy_block: usize,

    // Index of block with request text in vector above
    request_block: usize,

    // Index of block with response text in vector above
    response_block: usize,

    // Index of Statusbar in vector above
    statusbar_block: usize,

    // Index of help menu in vector above
    help_block: usize,

    // Index of errors block in vector above
    errors_block: usize,

    // Index of confirm widget
    confirm_block: usize,

    // Index of request/response in HTTPStorage.ui_storage which is current table's first element
    table_start_index: usize,

    // Index of request/response in HTTPStorage.ui_storage which is current table's last element
    table_end_index: usize,

    // Size in number of elements of table's sliding window
    table_window_size: usize,

    // Step size in number of items to take after cursor reaches current window's border
    table_step: usize,

    // Widgets which is active (chosen) now
    active_widget: usize,

    // Default widget header style
    default_widget_header_style: Style,

    // Active widget header style
    active_widget_header_style: Style,

    // Additional message to print in statusbar
    statusbar_message: Option<Text<'ui_lt>>,
}

impl UI<'static> {
    pub(crate) fn new() -> Self {
        let request_block = Block::default()
            .title("REQUEST").title_alignment(Alignment::Center)
            .borders(Borders::ALL);

        let response_block = Block::default()
            .title("RESPONSE").title_alignment(Alignment::Center)
            .borders(Borders::ALL);

        let proxy_history_block = Block::default()
            .title(Span::styled(
                "Proxy History",
                Style::default()
                    .bg(Color::White)
                    .fg(Color::Black)))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL);

        let statusbar_block = Block::default()
            .borders(Borders::TOP);

        UI {
            widgets: vec![
                RenderUnit::new_block(proxy_history_block, DEFAULT_PROXY_AREA, true),
                RenderUnit::new_block(request_block, DEFAULT_REQUEST_AREA, true),
                RenderUnit::new_block(response_block, DEFAULT_RESPONSE_AREA, true),
                RenderUnit::new_block(statusbar_block, DEFAULT_STATUSBAR_AREA, true),
                RenderUnit::PLACEHOLDER,
                RenderUnit::PLACEHOLDER,
            ],

            errors: Vec::new(),

            proxy_area: DEFAULT_PROXY_AREA,
            request_area: DEFAULT_REQUEST_AREA,
            response_area: DEFAULT_RESPONSE_AREA,
            statusbar_area: DEFAULT_STATUSBAR_AREA,
            help_area: DEFAULT_HELP_AREA,
            errors_area: DEFAULT_ERRORS_AREA,
            confirm_area: DEFAULT_CONFIRMATION_AREA,

            proxy_history_state: {
                let mut table_state = TableState::default();
                table_state.select(None);
                table_state
            },

            proxy_block: DEFAULT_PROXY_BLOCK,
            request_block: DEFAULT_REQUEST_BLOCK,
            response_block: DEFAULT_RESPONSE_BLOCK,
            statusbar_block: DEFAULT_STATUSBAR_BLOCK,
            help_block: DEFAULT_HELP_BLOCK,
            errors_block: DEFAULT_ERRORS_BLOCK,
            confirm_block: DEFAULT_CONFIRMATION_BLOCK,

            table_start_index: 0,
            table_end_index: 59,
            table_window_size: 60,
            table_step: 5,

            active_widget: DEFAULT_PROXY_BLOCK,        // Table
            active_widget_header_style: Style::default()
                .bg(Color::White)
                .fg(Color::Black),
            default_widget_header_style: Style::default(),

            statusbar_message: None
        }
    }

    fn draw_request(&mut self, storage: &HTTPStorage) {
        let header_style = if self.active_widget == self.request_block {
            self.active_widget_header_style
        }
        else {
            self.default_widget_header_style
        };

        let mut request_placeholder = || {
            self.widgets[self.request_block] = RenderUnit::new_block(
                Block::default()
                    .title(Span::styled("REQUEST", header_style))
                    .borders(Borders::ALL)
                    .title_alignment(Alignment::Center),
                self.request_area,
                true
            );
        };

        let selected_index = match self.proxy_history_state.selected() {
            Some(index) => index + self.table_start_index,
            None => {
                request_placeholder();
                return;
            }
        };

        if selected_index >= storage.len() {
            request_placeholder();
            return;
        }

        let request = storage.storage[selected_index].request.as_ref().unwrap();
        let mut request_list: Vec<Spans> = Vec::new();
        let tmp: Vec<Span> = vec![
            Span::styled(request.method.clone(), Style::default().add_modifier(Modifier::BOLD)),
            Span::from(" "),
            Span::from(request.get_request_path()),
            Span::from(" "),
            Span::from(format!("{}", request.version)),
        ];
        request_list.push(Spans::from(tmp));

        for (k, v) in request.headers.iter() {
            let mut tmp: Vec<Span> = Vec::new();
            tmp.push(Span::styled(k.to_string(), Style::default().fg(HEADER_NAME_COLOR)));
            tmp.push(Span::from(": ".to_string()));
            tmp.push(Span::from(format!("{}", v.to_str().unwrap())));
            request_list.push(Spans::from(tmp));
        }

        request_list.push(Spans::from(Span::from("")));

        for line in request.body.to_str_lossy().split("\n") {
            request_list.push(Spans::from(line.to_string()));
        }

        let new_block = Block::default()
            .title(Span::styled("REQUEST", header_style))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL);

        let scroll = self.widgets[self.request_block].paragraph_get_scroll().unwrap_or((0, 0));
        let request_paragraph = Paragraph::new(request_list)
            .block(new_block)
            .wrap(Wrap { trim: true })
            .scroll(scroll);

        let is_active = self.widgets[self.request_block].is_widget_active();
        self.widgets[self.request_block] =  RenderUnit::new_paragraph(
            request_paragraph,
            self.request_area,
            is_active,
            scroll
        );
    }

    fn draw_response(&mut self, storage: &HTTPStorage) {
        let header_style = if self.active_widget == self.response_block {
            self.active_widget_header_style
        }
        else {
            self.default_widget_header_style
        };

        let mut response_placeholder = || {
            self.widgets[self.response_block] = RenderUnit::new_block(
                Block::default()
                    .title(Span::styled("RESPONSE", header_style))
                    .borders(Borders::ALL)
                    .title_alignment(Alignment::Center),
                self.response_area,
                true
            );
        };

        let selected_index = match self.proxy_history_state.selected() {
            Some(index) => index + self.table_start_index,
            None => {
                response_placeholder();
                return;
            }
        };

        if selected_index >= storage.len() {
            response_placeholder();
            return;
        }

        let response = match storage.storage[selected_index].response.as_ref() {
            Some(rsp) => rsp,
            None => {
                let is_active = self.widgets[self.response_block].is_widget_active();
                self.widgets[self.response_block] = RenderUnit::new_block(
                    Block::default()
                        .title(Span::styled("RESPONSE", header_style))
                        .borders(Borders::ALL)
                        .title_alignment(Alignment::Center),
                    self.response_area,
                    is_active
                );
                return;
            }
        };

        let mut response_content: Vec<Spans> = vec![];
        // Status and version, like '200 OK HTTP/2'
        let first_line = Spans::from(vec![
            Span::styled(
                response.status.clone(),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            ),
            Span::from(" "),
            Span::from(response.version.clone())
        ]);
        response_content.push(first_line);

        // Response Headers
        for (k, v) in &response.headers {
            let header_line = Spans::from(vec![
                Span::styled(
                    k.clone().to_string(),
                    Style::default().fg(HEADER_NAME_COLOR)
                ),
                Span::from(": "),
                Span::from(v.clone().to_str().unwrap().to_string())
            ]);
            response_content.push(header_line)
        }

        // Empty line
        response_content.push(Spans::default());

        // Body
        let body = Spans::from(
            match response.body_compressed {
                BodyCompressedWith::NONE => {
                    match response.body.as_slice().to_str() {
                        Ok(s) => s.to_string(),
                        Err(e) => {
                            String::from_utf8_lossy(response.body.as_slice()).to_string()
                        }
                    }
                },
                BodyCompressedWith::GZIP => {
                    let writer = Vec::new();
                    let mut decoder = GzDecoder::new(writer);
                    decoder.write_all(response.body.as_slice()).unwrap();
                    decoder.finish().unwrap().to_str_lossy().to_string()
                },
                BodyCompressedWith::DEFLATE => {
                    todo!()
                },
                BodyCompressedWith::BR => {
                    // TODO: remove err when will support 'br'
                    let result = "'Brotli' encoding is not implemented yet".to_string();
                    self.log_error(CrusterError::NotImplementedError(result.clone()));
                    result
                }
        });
        response_content.push(body);

        let new_block = Block::default()
            .title(Span::styled("RESPONSE", header_style))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL);

        let scroll = self.widgets[self.response_block].paragraph_get_scroll().unwrap_or((0, 0));
        let response_paragraph = Paragraph::new(response_content)
            .block(new_block)
            .wrap(Wrap { trim: false })
            .scroll(scroll);

        let is_active = self.widgets[self.response_block].is_widget_active();
        self.widgets[self.response_block] = RenderUnit::new_paragraph(
            response_paragraph,
            self.response_area,
            is_active,
            scroll
        );
    }

    fn draw_statusbar(&mut self, storage: &HTTPStorage) {
        // TODO: seaprate messages to left and right
        //
        // -----------------------------------------------------------------------------------------
        // | Errors: N | Requests: K                                             Type "?" for help |
        // -----------------------------------------------------------------------------------------
        let status_block = Block::default()
            .borders(Borders::TOP);

        let raw_status = vec![
            Span::styled("Errors: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                self.errors.len().to_string(),
                Style::default()
                    .fg(Color::LightRed)
                    .add_modifier(Modifier::BOLD)
            ),
            Span::from(" | "),
            Span::styled("Requests: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::from(storage.len().to_string()),
            Span::from(" | "),
            Span::from("Type '"),
            Span::styled(
                "?",
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD)
            ),
            Span::from("' for help"),
        ];

        let status = match &self.statusbar_message {
            Some(message) => {
                let mut tmp = message.lines[0].0.clone();
                tmp.push(Span::from(" | "));
                tmp.extend(raw_status);
                Spans::from(tmp)
            },
            None => { Spans::from(raw_status) }
        };

        let status_paragraph = Paragraph::new(status)
            .block(status_block)
            .alignment(Alignment::Right);

        self.widgets[self.statusbar_block] = RenderUnit::new_paragraph(
            status_paragraph,
            self.statusbar_area,
            true,
            (0, 0)
        );
    }

    pub(crate) fn draw_state(&mut self, storage: & HTTPStorage) {
        self.draw_request(storage);
        self.draw_response(storage);
        self.draw_statusbar(storage);
    }

    pub(crate) fn set_statusbar_message<T: Into<Text<'static>>>(&mut self, message: Option<T>) {
        self.statusbar_message = match message {
            Some(m) => {
                Some(m.into())
            },
            None =>  { None }
        };
    }

    pub(crate) fn show_help(&mut self) {
        self.widgets[self.help_block] = make_help_menu(self.help_area);
    }

    pub(crate) fn hide_help(&mut self) {
        self.widgets[self.help_block] = RenderUnit::PLACEHOLDER;
    }

    pub(crate) fn show_errors(&mut self) {
        let errors = Paragraph::new(self.errors.clone())
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title(
                        Span::styled(
                            " ERRORS ",
                            Style::default()
                                .bg(Color::Red)
                                .fg(Color::Black)))

                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
            );

        self.widgets[self.errors_block] = RenderUnit::new_paragraph(
            errors,
            self.errors_area,
            true,
            (0, 0)
        );
    }

    pub(crate) fn hide_errors(&mut self) {
        self.widgets[self.errors_block] = RenderUnit::PLACEHOLDER;
    }

    pub(crate) fn log_error(&mut self, error: CrusterError) {
        self.errors.push(
            Spans::from(vec![
                Span::styled("[ERROR] ", Style::default().fg(Color::Red)),
                Span::from(error.to_string())
            ])
        )
    }

    pub(crate) fn show_confirmation(&mut self, text: &str) {
        let confirm_paragraph = Paragraph::new(
            vec![
                // \n
                Spans::from(
                    Span::from("")
                ),
                // The thing to confirm,
                Spans::from(
                    Span::styled(
                    text.to_string(),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD))
                ),
                // \n
                Spans::from(
                    Span::from("")
                ),
                // Enter [y]es or [n]o.
                Spans::from(
                    vec![
                        Span::styled(
                            "Enter ",
                            Style::default().add_modifier(Modifier::UNDERLINED)
                        ),
                        Span::styled(
                            "y",
                            Style::default().add_modifier(Modifier::UNDERLINED | Modifier::BOLD).fg(Color::LightYellow)
                        ),
                        Span::styled(
                            "es or ",
                            Style::default().add_modifier(Modifier::UNDERLINED)
                        ),
                        Span::styled(
                            "n",
                            Style::default().add_modifier(Modifier::UNDERLINED | Modifier::BOLD).fg(Color::LightYellow)
                        ),
                        Span::styled(
                            "o.",
                            Style::default().add_modifier(Modifier::UNDERLINED)
                        ),
                    ]
                ),
            ]
        )
            .alignment(Alignment::Center)
            .wrap(Wrap {trim: true})
            .block(
                Block::default()
                    .title(
                        Span::styled(
                            " CONFIRM ",
                            Style::default()
                                .bg(Color::Yellow)
                                .fg(Color::Black)))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL));

        let confirmation = RenderUnit::new_paragraph(
            confirm_paragraph,
            self.confirm_area,
            true,
            (0, 0)
        );

        self.widgets[self.confirm_block] = confirmation;
    }

    pub(crate) fn hide_confirmation(&mut self) {
        self.widgets[self.confirm_block] = RenderUnit::PLACEHOLDER;
    }

    pub(super) fn make_table(&mut self, storage: &HTTPStorage, size: Rect) {
        if storage.len() == 0 {
            return
        }

        debug!("Making table with width {} and height {}", size.width, size.height);

        let mut rows: Vec<Row> = Vec::new();
        for (index, pair) in storage.storage
            .iter()
            .skip(self.table_start_index)
            .take(self.table_window_size)
            .enumerate()
        {
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
                (index + self.table_start_index + 1).to_string(),
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
        match self.proxy_history_state.selected() {
            Some(i) => {
                let initial_index = i;
                if storage_len < window && i == storage_len - 1 {
                    self.proxy_history_state.select(
                        Some(storage_len.saturating_sub(1))
                    );
                }
                else if i == window - 1 && end >= storage_len - 1 {
                    self.table_end_index = storage_len.saturating_sub(1);
                    self.table_start_index = (self.table_end_index + 1).saturating_sub(window);
                    self.proxy_history_state.select(Some(min(storage_len, window).saturating_sub(1)));
                }
                else if i == window - 1 && end < storage_len - 1 {
                    self.table_end_index += 1;
                    self.table_start_index += 1;
                }
                else {
                    self.proxy_history_state.select(Some(i + 1));
                }
                let final_index = self.proxy_history_state.selected().unwrap();
                if final_index != initial_index {
                    self.reset_scrolling();
                }
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
        match self.proxy_history_state.selected() {
            Some(i) => {
                let initial_index = i;
                if i == 0 && start == 0 {
                    self.table_end_index = window - 1;
                }
                else if i == 0 && start > 0 {
                    self.table_end_index -= 1;
                    self.table_start_index -= 1;
                }
                else {
                    self.proxy_history_state.select(Some(i.saturating_sub(1)));
                }
                let final_index = self.proxy_history_state.selected().unwrap();
                if initial_index != final_index {
                    self.reset_scrolling();
                }
            },
            None => {
                if storage.len() > 0 {
                    self.proxy_history_state.select(Some(0));
                }
            }
        }
        debug!("table_step_up_2: start_index - {}, end_index - {}, state - {:?}", self.table_start_index, self.table_end_index, self.proxy_history_state.selected());
    }

    pub(super) fn activate_proxy(&mut self) {
        self.active_widget = self.proxy_block;
    }

    pub(super) fn activate_request(&mut self) {
        self.active_widget = self.request_block;
    }

    pub(super) fn activate_response(&mut self) {
        self.active_widget = self.response_block;
    }

    pub(super) fn is_table_active(&self) -> bool {
        return self.active_widget == self.proxy_block;
    }

    pub(crate) fn is_request_active(&self) -> bool {
        return self.active_widget == self.request_block;
    }

    pub(crate) fn is_response_active(&self) -> bool {
        return self.active_widget == self.response_block;
    }

    pub(super) fn show_fullscreen(&mut self) {
        let show_routine = |active_widget_index: usize, widgets: &mut Vec<RenderUnit>| {
            for i in 0..widgets.len() {
                // Handling widget and it's clear block
                if i == active_widget_index {
                    widgets[i].set_rect_index(DEFAULT_FULLSCREEN_AREA);
                    widgets[i].enable();
                }
                else {
                    widgets[i].disable();
                }
            }
        };

        debug!("show_fullscreen: active - {}", &self.active_widget);
        let mut w = &mut self.widgets;
        if self.active_widget == self.proxy_block {
            self.proxy_area = DEFAULT_FULLSCREEN_AREA;
            show_routine(self.active_widget, w);
        }
        else if self.active_widget == self.response_block {
            self.response_area = DEFAULT_FULLSCREEN_AREA;
            show_routine(self.active_widget, w);
        }
        else if self.active_widget == self.request_block {
            self.request_area = DEFAULT_FULLSCREEN_AREA;
            show_routine(self.active_widget, w);
        }
    }

    pub(super) fn cancel_fullscreen(&mut self) {
        let cancel_routine = |active_widget_index: usize, new_area_index: usize, widgets: &mut Vec<RenderUnit>| {
            for i in 0..widgets.len() {
                // Handling widget and it's clear block
                if i == active_widget_index {
                    widgets[i].set_rect_index(new_area_index);
                }
                widgets[i].enable();
            }
        };

        debug!("cancel_fullscreen: active - {}", &self.active_widget);
        let mut w = &mut self.widgets;
        if self.active_widget == self.proxy_block {
            self.proxy_area = DEFAULT_PROXY_AREA;
            cancel_routine(self.active_widget, self.proxy_area, w);
        }
        else if self.active_widget == self.response_block {
            self.response_area = DEFAULT_RESPONSE_AREA;
            cancel_routine(self.active_widget, self.response_area, w);
        }
        else if self.active_widget == self.request_block {
            self.request_area = DEFAULT_REQUEST_AREA;
            cancel_routine(self.active_widget, self.request_area, w);
        }
    }

    pub(super) fn scroll_request(&mut self, x: Option<i16>, y: Option<i16>) {
        let mut request_block = &mut self.widgets[self.request_block];
        let base_axes = request_block
            .paragraph_get_scroll()
            .unwrap_or((0, 0));

        let new_axes = scrolling_paragraph_axes(base_axes, (x, y));
        debug!("scroll_request: new (x, y) = ({}, {})", new_axes.0, new_axes.1);
        request_block.paragraph_set_scroll(new_axes);
    }

    pub(super) fn scroll_response(&mut self, x: Option<i16>, y: Option<i16>) {
        let mut response_block = self.widgets[self.response_block].borrow_mut();
        let base_axes = response_block
            .paragraph_get_scroll()
            .unwrap_or((0, 0));

        let new_axes = scrolling_paragraph_axes(base_axes, (x, y));
        debug!("scroll_response: new (x, y) = ({}, {})", new_axes.0, new_axes.1);
        response_block.paragraph_set_scroll(new_axes);
    }

    pub(super) fn get_table_sliding_window(&self) -> usize {
        return self.table_window_size.clone();
    }

    pub(super) fn table_scroll_page_down(&mut self, storage: &HTTPStorage) {
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

    pub(super) fn table_scroll_page_up(&mut self, storage: &HTTPStorage) {
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

    pub(super) fn table_scroll_end(&mut self, storage: &HTTPStorage) {
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

    pub(super) fn table_scroll_home(&mut self, storage: &HTTPStorage) {
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

    fn reset_scrolling(&mut self) {
        match self.widgets[self.response_block].paragraph_reset_scroll() {
            Ok(_) => {},
            Err(e) => self.log_error(e)
        }

        match self.widgets[self.request_block].paragraph_reset_scroll() {
            Ok(_) => {},
            Err(e) => self.log_error(e)
        }
    }
}

fn scrolling_paragraph_axes(base_axes: (u16, u16), arguments: (Option<i16>, Option<i16>)) -> (u16, u16) {
    let (base_x, base_y) = base_axes;
    let (x, y) = arguments;

    let new_x = if let Some(x_value) = x {
        debug!("scrolling_paragraph_axes: x = {}", &x_value);
        if x_value < 0 {
            let x_abs = x_value.abs() as u16;
            if base_x < x_abs {
                0u16
            }
            else {
                base_x - x_abs
            }
        }
        else {
            base_x.saturating_add(x_value as u16)
        }
    }
    else {
        debug!("scrolling_paragraph_axes: x = None");
        0u16
    };

    let new_y = if let Some(y_value) = y {
        debug!("scrolling_paragraph_axes: y = {}", &y_value);
        if y_value < 0 {
            base_y.saturating_sub(y_value.abs() as u16)
        }
        else {
            base_y.saturating_add(y_value as u16)
        }
    }
    else {
        debug!("scrolling_paragraph_axes: y = None");
        0u16
    };

    return (new_x, new_y);
}