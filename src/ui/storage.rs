use std::{io, time::{Duration, Instant}};
use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{Widget, Block, Borders, Paragraph, Wrap, Table, Row, TableState},
    layout::{Rect, Alignment, Constraint},
    style::{Color, Modifier, Style},
    Terminal,
    text,
    Frame,
    self
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tokio::sync::mpsc::Receiver;
use tui::text::Text;

use crate::utils::CrusterError;
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
    pub(crate) fn as_paragraph(&self) -> Result<&Paragraph, CrusterError> {
        match self {
            RenderUnit::TUIParagraph(paragraph) => { Ok(&paragraph.0) },
            _ => Err(CrusterError::RenderUnitCastError(String::from("'as_paragrpah' called on non-paragraph widget")))
        }
    }

    pub(crate) fn as_table(&self) -> Result<Table, CrusterError> {
        match self {
            RenderUnit::TUITable(table) => { Ok(table.0.clone()) },
            _ => Err(CrusterError::RenderUnitCastError(String::from("'as_table' called on non-table widget")))
        }
    }

    pub(crate) fn area(&self) -> usize {
        match self {
            RenderUnit::TUITable((t, a)) => a.to_owned(),
            RenderUnit::TUIBlock((b, a)) => a.to_owned(),
            RenderUnit::TUIParagraph((p, a)) => a.to_owned(),
        }
    }
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
    saved_window_width: u16
    // request_block_index: usize,
    // response_block_history: usize
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
                table_state.select(Some(0));
                table_state
            },
            table_widths: None,
            saved_window_width: 0
        }
    }

    pub(crate) fn add_block(&mut self, block: Block<'static>, area_index: usize) {
        let new_render_unit = RenderUnit::TUIBlock((block, area_index));
        self.widgets.push(new_render_unit);
    }

    pub(crate) fn add_paragraph(&mut self, para: Paragraph<'static>, area_index: usize) {
        let new_render_unit = RenderUnit::TUIParagraph((para, area_index));
        self.widgets.push(new_render_unit);
    }

    // pub(crate) fn modify_proxy_history(&mut self, text: &str) -> Result<(), CrusterError> {
    //     Ok(())
    // }

    pub(crate) fn make_table_widths(&mut self, size: u16) {
        if size == self.saved_window_width { return; }

        self.saved_window_width = size;
        self.table_widths = Some([
            Constraint::Length(16),
            Constraint::Length(16),
            Constraint::Length(16 * 6 + 9),
            Constraint::Length(16),
            Constraint::Length(16 * 2)
        ]);

        let proxy_history_table = Table::new(vec![])
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
    }
}

// ---------------------------------------------------------------------------------------------- //

pub(crate) struct HTTPStorage {
    storage: Vec<(Option<HyperRequestWrapper>, Option<HyperResponseWrapper>)>,
    seek: usize,
    capacity: usize
}

impl Default for HTTPStorage {
    fn default() -> Self {
        Storage {
            storage: Vec::new(),
            seek: 0,
            capacity: 10000
        }
    }
}
