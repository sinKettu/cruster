use std::collections::HashMap;
use std::net::SocketAddr;
use tui::{
    // backend::{CrosstermBackend, Backend},
    widgets::{/*Widget,*/ Block, Borders, Paragraph, /*Wrap,*/ Table, Row, TableState},
    layout::{/*Rect, Alignment,*/ Constraint},
    style::{Color, Modifier, Style},
    // Terminal,
    // text,
    // Frame,
    self
};
use tui::widgets::Wrap;
// use crossterm::{
//     event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };
// use tokio::sync::mpsc::Receiver;
// use tui::text::Text;

// use crate::utils::CrusterError;
use crate::cruster_handler::request_response::{HyperRequestWrapper, HyperResponseWrapper};

#[derive(Clone, Debug)]
pub(crate) enum RenderUnit<'ru_lt> {
    TUIBlock(
        (
            Block<'ru_lt>,
            usize
        )
    ),
    TUIParagraph(
        (
            Paragraph<'ru_lt>,
            usize
        )
    ),
    TUITable(
        (
            Table<'ru_lt>,
            usize
        )
    )
}

impl RenderUnit<'_> {
    // pub(crate) fn as_paragraph(&self) -> Result<&Paragraph, CrusterError> {
    //     match self {
    //         RenderUnit::TUIParagraph(paragraph) => { Ok(&paragraph.0) },
    //         _ => Err(CrusterError::RenderUnitCastError(String::from("'as_paragrpah' called on non-paragraph widget")))
    //     }
    // }
    //
    // pub(crate) fn as_table(&self) -> Result<Table, CrusterError> {
    //     match self {
    //         RenderUnit::TUITable(table) => { Ok(table.0.clone()) },
    //         _ => Err(CrusterError::RenderUnitCastError(String::from("'as_table' called on non-table widget")))
    //     }
    // }

    // pub(crate) fn as_block(&self) -> Result<Block, CrusterError> {
    //     match self {
    //         RenderUnit::TUIBlock(block) => { Ok(block.0.clone()) },
    //         _ => Err(CrusterError::RenderUnitCastError(String::from("'as_table' called on non-table widget")))
    //     }
    // }
    //
    // pub(crate) fn area(&self) -> usize {
    //     match self {
    //         RenderUnit::TUITable((_, a)) => a.to_owned(),
    //         RenderUnit::TUIBlock((_, a)) => a.to_owned(),
    //         RenderUnit::TUIParagraph((_, a)) => a.to_owned(),
    //     }
    // }
}

// ---------------------------------------------------------------------------------------------- //

pub(crate) struct UI<'ui_lt> {
    // 0 - Rect for requests log,
    // 1 - Rect for requests
    // 2 - Rect for responses
    pub(crate) widgets: Vec<RenderUnit<'ui_lt>>,
    // Position of block with proxy history in rectangles array (see ui/mod.rs:new:ui)
    proxy_history_index: usize,
    // State of table with proxy history
    pub(crate) proxy_history_state: TableState,
    // Array of table's cells' widths
    table_widths: Option<[Constraint; 5]>,
    // Last known window width
    saved_window_width: u16,
    request_block_index: usize,
    response_block_index: usize
}

impl UI<'static> {
    pub(crate) fn new() -> Self {
        let request_block = Block::default()
            .title("Upper Left Block")
            .borders(Borders::TOP);

        let response_block = Block::default()
            .title("Upper Right Block")
            .borders(Borders::TOP | Borders::LEFT);

        // let proxy_history = Paragraph::new("")
        //     .block(
        //         Block::default()
        //             .title("Proxy History")
        //             .borders(Borders::ALL)
        //     );

        UI {
            widgets: vec![
                RenderUnit::TUIBlock((request_block, 1)),
                RenderUnit::TUIBlock((response_block, 2)),
                // RenderUnit::TUITable((proxy_history_table, 0))
            ],
            proxy_history_index: 2,
            proxy_history_state: {
                let mut table_state = TableState::default();
                table_state.select(None);
                table_state
            },
            table_widths: None,
            saved_window_width: 0,
            request_block_index: 1,
            response_block_index: 2
        }
    }

    // pub(crate) fn add_block(&mut self, block: Block<'static>, area_index: usize) {
    //     let new_render_unit = RenderUnit::TUIBlock((block, area_index));
    //     self.widgets.push(new_render_unit);
    // }
    //
    // pub(crate) fn add_paragraph(&mut self, para: Paragraph<'static>, area_index: usize) {
    //     let new_render_unit = RenderUnit::TUIParagraph((para, area_index));
    //     self.widgets.push(new_render_unit);
    // }

    // pub(crate) fn modify_proxy_history(&mut self, text: &str) -> Result<(), CrusterError> {
    //     Ok(())
    // }

    pub(crate) fn make_table_widths(&mut self, size: u16, storage: &HTTPStorage) {
        self.saved_window_width = size;
        self.table_widths = Some([
            Constraint::Length(16),
            Constraint::Length(16),
            Constraint::Length(16 * 6 + 9),
            Constraint::Length(16),
            Constraint::Length(16 * 2)
        ]);
        // self.widgets.clear();

        if let None = self.proxy_history_state.selected() {
            if storage.storage.len() > 0 {
                self.proxy_history_state.select(Some(0));
            }
        }

        let mut rows: Vec<Row> = Vec::new();
        for (index, pair) in storage.storage.iter().enumerate() {
            let request = pair.request.as_ref().unwrap();
            let response = pair.response.as_ref();
            let row = vec![
                (index + 1).to_string(),
                request.method.clone(),
                request.uri.clone(),
                if let Some(rsp) = response {rsp.status.clone()} else {". . .".to_string()},
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
                    .borders(Borders::ALL));

        self.widgets.push(RenderUnit::TUITable((proxy_history_table, 0)));
        self.proxy_history_index = self.widgets.len() - 1;

        // TODO: as new function
        let selected_index = match self.proxy_history_state.selected() {
            Some(index) => index,
            None => return
        };

        let request = storage.storage[selected_index].request.as_ref().unwrap();
        let request: String = format!(
            "{} {} {}\n{}\n{}",
            request.method, request.uri, request.version,
            {
                let mut headers_string: String = "".to_string();
                for (k, v) in request.headers.iter() {
                    headers_string.push_str(k.as_str());
                    headers_string.push_str(": ");
                    headers_string.push_str(v.to_str().unwrap());
                    headers_string.push_str("\n");
                }
                headers_string
            },
            // String::from(&request.body)
            &request.body
        );
        let new_block = Block::default()
            .title("Request")
            .borders(Borders::TOP);

        let request_paragraph = Paragraph::new(request)
            .block(new_block)
            .wrap(Wrap {trim: false});
        // self.widgets.push(RenderUnit::TUIParagraph((request_paragraph, self.request_block_index)));
        self.widgets.insert(self.proxy_history_index + 1, RenderUnit::TUIParagraph((request_paragraph, self.request_block_index)));

        // TODO: as new function
        let response = match storage.storage[selected_index].response.as_ref() {
            Some(rsp) => rsp,
            None => return
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
            &response.body
        );

        let new_block = Block::default()
            .title("Response")
            .borders(Borders::TOP | Borders::LEFT);

        let response_paragraph = Paragraph::new(response)
            .block(new_block)
            .wrap(Wrap {trim: false});

        // self.widgets.push(RenderUnit::TUIParagraph((response_paragraph, self.response_block_index)));
        self.widgets.insert(self.proxy_history_index + 2, RenderUnit::TUIParagraph((response_paragraph, self.response_block_index)));
    }
}

// ---------------------------------------------------------------------------------------------- //

pub(super) struct RequestResponsePair {
    request: Option<HyperRequestWrapper>,
    response: Option<HyperResponseWrapper>
}

pub(crate) struct HTTPStorage {
    pub(super) storage: Vec<RequestResponsePair>,
    context_reference: HashMap<SocketAddr, usize>,
    // seek: usize,
    // capacity: usize
}

impl Default for HTTPStorage {
    fn default() -> Self {
        HTTPStorage {
            storage: Vec::new(),
            context_reference: HashMap::new(),
            // seek: 0,
            // capacity: 10000
        }
    }
}

impl HTTPStorage {
    pub(crate) fn put_request(&mut self, request: HyperRequestWrapper, addr: SocketAddr) {
        self.storage.push(RequestResponsePair {
                request: Some(request),
                response: None
            }
        );

        self.context_reference.insert(addr, self.storage.len() - 1);
    }

    pub(crate) fn put_response(&mut self, response: HyperResponseWrapper, addr: &SocketAddr) {
        if let Some(index) = self.context_reference.get(addr) {
            self.storage[index.to_owned()].response = Some(response);
        }

        self.context_reference.remove(addr);
    }

    pub(crate) fn len(&self) -> usize {
        return self.storage.len().clone();
    }
}
