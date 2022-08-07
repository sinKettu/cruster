pub(crate) mod render_units;
pub(crate) mod help;

use render_units::*;

use flate2::write::GzDecoder;
use std::io::prelude::*;
use bstr::ByteSlice;

use std::{
    cmp::min,
    collections::HashMap
};
use std::os::macos::raw::stat;

use crate::cruster_handler::request_response::{
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



// ---------------------------------------------------------------------------------------------- //

fn make_help_menu<'help>(rect_index: usize) -> (RenderUnit<'help>, RenderUnit<'help>) {
    let help_text: Vec<Spans> = vec![
        Spans::from(vec![
            Span::styled(
                "?",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)),
            Span::raw(" - Show this screen")
        ]),
        Spans::from(vec![
            Span::styled(
                "↑ ↓ ← →",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)),
            Span::raw(" - Navigation")
        ]),
        Spans::from(vec![
            Span::styled(
                "q",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)),
            Span::raw(" - Quit / Go back")
        ]),
    ];

    let help_block = Block::default()
        .title("HELP")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::Green))
        .borders(Borders::ALL);

    let help_paragraph = Paragraph::new(help_text)
        .block(help_block);

    // let mut clear = UniversalRenderUnit::make_clear(rect_index);
    let mut clear = RenderUnit::new_clear(rect_index);
    // let help = UniversalRenderUnit::make_paragraph(help_paragraph, rect_index, true);
    let help = RenderUnit::new_paragraph(help_paragraph, rect_index, false);
    clear.disable();

    return (clear, help);
}

pub(crate) struct UI<'ui_lt> {
    // 0 - Rect for requests log,
    // 1 - Rect for requests
    // 2 - Rect for responses
    // 3 - Rect for statusbar
    // 4 - Rect for help menu
    pub(crate) widgets: Vec<RenderUnit<'ui_lt>>,

    // Position of block with proxy history in rectangles array (see ui/mod.rs:new:ui)
    proxy_history_index: usize,

    // Position of block with request data
    request_area_index: usize,

    // Position of block with response data
    response_area_index: usize,

    // Statusbar area index,
    statusbar_area_index: usize,

    // State of table with proxy history
    pub(crate) proxy_history_state: TableState,

    // Index of block with request text in vector above
    request_block_index: usize,

    // Index of block with response text in vector above
    response_block_index: usize,

    // Index of block with proxy history in vector above
    proxy_block_index: usize,

    // Index of Statusbar in vector above
    status_index: usize,

    // Index of help menu in vector above
    help_index: usize,

    // Index of request/response in HTTPStorage.storage which is current table's first element
    table_start_index: usize,

    // Index of request/response in HTTPStorage.storage which is current table's last element
    table_end_index: usize,

    // Size in number of elements of table's sliding window
    table_window_size: usize,

    // Step size in number of items to take after cursor reaches current window's border
    table_step: usize
}

impl UI<'static> {
    pub(crate) fn new() -> Self {
        let request_block = Block::default()
            .title("REQUEST").title_alignment(Alignment::Center)
            .borders(Borders::TOP);

        let response_block = Block::default()
            .title("RESPONSE").title_alignment(Alignment::Center)
            .borders(Borders::TOP | Borders::LEFT);

        let proxy_history_block = Block::default()
            .title("Proxy History")
            .borders(Borders::ALL);

        let statusbar_block = Block::default()
            .borders(Borders::ALL);

        let (help_clear_widget, help_paragraph) = make_help_menu(4);

        UI {
            widgets: vec![
                RenderUnit::new_clear(0),
                RenderUnit::new_block(proxy_history_block, 0, true),
                RenderUnit::new_clear(1),
                RenderUnit::new_block(request_block, 1, true),
                RenderUnit::new_clear(2),
                RenderUnit::new_block(response_block, 2, true),
                RenderUnit::new_clear(3),
                RenderUnit::new_block(statusbar_block, 3, true),
                help_clear_widget,
                help_paragraph
            ],
            proxy_history_index: 0,
            request_area_index: 1,
            response_area_index: 2,
            statusbar_area_index: 3,
            proxy_history_state: {
                let mut table_state = TableState::default();
                table_state.select(None);
                table_state
            },
            request_block_index: 3,
            response_block_index: 5,
            proxy_block_index: 1,
            status_index: 7,
            help_index: 9,
            table_start_index: 0,
            table_end_index: 24,
            table_window_size: 25,
            table_step: 5
        }
    }

    pub(crate) fn draw_state(&mut self, storage: & HTTPStorage) {
        if storage.len() == 0 { return; }

        if let None = self.proxy_history_state.selected() {
            if storage.storage.len() > 0 {
                self.proxy_history_state.select(Some(0));
            }
        }

        let selected_index = match self.proxy_history_state.selected() {
            Some(index) => index + self.table_start_index,
            None => {
                self.widgets[self.request_block_index] = RenderUnit::new_block(
                    Block::default()
                        .title("REQUEST")
                        .borders(Borders::TOP)
                        .title_alignment(Alignment::Center),
                    self.request_area_index,
                    true
                );

                self.widgets[self.response_block_index] = RenderUnit::new_block(
                    Block::default()
                        .title("RESPONSE")
                        .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
                        .title_alignment(Alignment::Center),
                    self.response_area_index,
                    true
                );
                return;
            }
        };

        // TODO: REQUEST as new function
        {
            let request = storage.storage[selected_index].request.as_ref().unwrap();
            let mut request_list: Vec<Spans> = Vec::new();
            let tmp: Vec<Span> = vec![
                Span::from(format!("{} ", request.method)),
                Span::from(format!("{} ", request.uri)),
                Span::from(format!("{}", request.version)),
            ];
            request_list.push(Spans::from(tmp));

            for (k, v) in request.headers.iter() {
                let mut tmp: Vec<Span> = Vec::new();
                tmp.push(Span::from(format!("{}", k)));
                tmp.push(Span::from(": ".to_string()));
                tmp.push(Span::from(format!("{}", v.to_str().unwrap())));
                request_list.push(Spans::from(tmp));
            }

            request_list.push(Spans::from(Span::from("")));

            let tmp: Vec<Span> = request
                .body
                .clone()
                .to_str_lossy()
                .split("\n")
                .map(|s| Span::from(s.to_string()))
                .collect();

            request_list.push(Spans::from(tmp));

            let new_block = Block::default()
                .title("REQUEST").title_alignment(Alignment::Center)
                .borders(Borders::TOP | Borders::BOTTOM);

            let request_paragraph = Paragraph::new(request_list)
                .block(new_block)
                .wrap(Wrap { trim: true });

            self.widgets[self.request_block_index] =  RenderUnit::new_paragraph(request_paragraph, self.request_area_index, true);
        }

        // TODO: RESPONSE as new function
        {
            let response = match storage.storage[selected_index].response.as_ref() {
                Some(rsp) => rsp,
                None => {
                    self.widgets[self.response_block_index] = RenderUnit::new_block(
                        Block::default()
                            .title("RESPONSE")
                            .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
                            .title_alignment(Alignment::Center),
                        self.response_area_index,
                        true
                    );
                    return;
                }
            };

            let response: String = format!(
                "{} {}\n{}\n{}",
                response.status, response.version,
                {
                    let mut headers_string: String = "".to_string();
                    for (k, v) in response.headers.iter() {
                        headers_string.push_str(k.as_str());
                        headers_string.push_str(": ");
                        headers_string.push_str(v.to_str().unwrap());
                        headers_string.push_str("\n");
                    }
                    headers_string
                },
                match response.body_compressed {
                    BodyCompressedWith::NONE => String::from_utf8_lossy(response.body.as_slice()).to_string(),
                    BodyCompressedWith::GZIP => {
                        let writer = Vec::new();
                        let mut decoder = GzDecoder::new(writer);
                        decoder.write_all(response.body.as_slice()).unwrap();
                        decoder.finish().unwrap().to_str_lossy().to_string()
                    }
                    BodyCompressedWith::DEFLATE => { todo!() }
                }
            );

            let new_block = Block::default()
                .title("RESPONSE").title_alignment(Alignment::Center)
                .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM);

            let response_paragraph = Paragraph::new(response)
                .block(new_block)
                .wrap(Wrap { trim: false });

            self.widgets[self.response_block_index] = RenderUnit::new_paragraph(response_paragraph, self.response_area_index, true);
        }
    }

    pub(crate) fn draw_statusbar(&mut self, storage: &HTTPStorage) {
        // -----------------------------------------------------------------------------------------
        // | Errors: N | Requests: K                                             Type "?" for help |
        // -----------------------------------------------------------------------------------------
        let status_block = Block::default()
            .borders(Borders::ALL);
        let status_paragraph = Paragraph::new(vec![
            Spans::from(vec![
                Span::styled("Errors: ", Style::default().add_modifier(Modifier::BOLD)),
                // TODO: make it real later
                Span::from("0".to_string()),
                Span::from(" | "),
                Span::styled("Requests: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::from(storage.len().to_string()),
                Span::from(" | "),
                Span::from("Type '?' for help")
            ])
        ])
            .block(status_block)
            .alignment(Alignment::Right);


        self.widgets[self.status_index] = RenderUnit::new_paragraph(status_paragraph, self.statusbar_area_index, true);
    }

    pub(crate) fn show_help(&mut self) {
        // Make RenderUnit::TUIClear active for help's clear widget
        self.widgets[self.help_index - 1].enable();

        // Make RenderUnit::TUIParagraph active for help's clear widget
        self.widgets[self.help_index].enable();
    }

    pub(crate) fn hide_help(&mut self) {
        // Just like in show_help()
        self.widgets[self.help_index - 1].disable();
        self.widgets[self.help_index].disable();
    }

    fn make_table(&mut self, storage: &HTTPStorage) {
        let mut rows: Vec<Row> = Vec::new();
        for (index, pair) in storage.storage
            .iter()
            .skip(self.table_start_index)
            .take(self.table_window_size + 5)
            .enumerate()
        {
            let request = pair.request.as_ref().unwrap();
            let response = pair.response.as_ref();
            let row = vec![
                (index + self.table_start_index + 1).to_string(),
                request.method.clone(),
                request.uri.clone(),
                if let Some(rsp) = response {rsp.status.clone()} else {"".to_string()},
                "TODO".to_string()
            ];
            rows.push(Row::new(row));
        }

        let proxy_history_table = Table::new(rows)
            .header(Row::new(vec!["№", "Method", "URL", "Response Code", "Address"]))
            .style(Style::default().fg(Color::White))
            .widths(&[
                Constraint::Length(16),
                Constraint::Length(16),
                Constraint::Length(16 * 6 + 9),
                Constraint::Length(16),
                Constraint::Length(16 * 2)
            ])
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD))
            .block(
                Block::default()
                    .title("Proxy History")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL));

        self.widgets[self.proxy_block_index] = RenderUnit::new_table(
            proxy_history_table,
            self.proxy_history_index,
            true

        );
    }

    pub(crate) fn update_table(&mut self, storage: &HTTPStorage) {
        match self.proxy_history_state.selected() {
            Some(i) => {
                if storage.len() < self.table_window_size {
                    self.make_table(storage);
                }
                else if i >= self.table_window_size {
                    let index = self.table_window_size - min(storage.len() - self.table_end_index, self.table_step);
                    self.table_end_index = min(storage.len() - 1, self.table_end_index + self.table_step);
                    self.table_start_index = self.table_end_index.saturating_sub(self.table_window_size - 1);
                    self.proxy_history_state.select(Some(index));
                    self.make_table(storage);
                }
                else if i == 0 {
                    let index = min(self.table_start_index, self.table_step);
                    self.table_start_index = self.table_start_index.saturating_sub(self.table_step);
                    self.table_end_index = self.table_start_index + self.table_window_size - 1;
                    self.proxy_history_state.select(Some(index));
                    self.make_table(storage);
                }
            },
            None => {
                self.make_table(storage);
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------- //

pub(super) struct RequestResponsePair {
    request: Option<HyperRequestWrapper>,
    response: Option<HyperResponseWrapper>
}

pub(crate) struct HTTPStorage {
    pub(super) storage: Vec<RequestResponsePair>,
    context_reference: HashMap<usize, usize>,
    // seek: usize,
    // capacity: usize
}

impl Default for HTTPStorage {
    fn default() -> Self {
        HTTPStorage {
            storage: Vec::with_capacity(1000),
            context_reference: HashMap::new(),
            // seek: 0,
            // capacity: 10000
        }
    }
}

impl HTTPStorage {
    pub(crate) fn put_request(&mut self, request: HyperRequestWrapper, addr: usize) {
        self.storage.push(RequestResponsePair {
                request: Some(request),
                response: None
            }
        );

        self.context_reference.insert(addr, self.storage.len() - 1);
    }

    pub(crate) fn put_response(&mut self, response: HyperResponseWrapper, addr: &usize) {
        if let Some(index) = self.context_reference.get(addr) {
            self.storage[index.to_owned()].response = Some(response);
        }

        self.context_reference.remove(addr);
    }

    pub(crate) fn len(&self) -> usize {
        return self.storage.len().clone();
    }
}
