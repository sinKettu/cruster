use std::{io, time::{Duration, Instant}};
use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{Widget, Block, Borders, Paragraph, Wrap},
    layout::{Rect, Alignment},
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

// #[derive(Clone, Debug)]
// pub(crate) struct RenderUnit<W: Widget> {
//     widget: W,
//     area: usize
// }
//
// impl<W: Widget> RenderUnit<W> {
//     pub(crate) fn widget(&self) -> W {
//         return self.widget.clone();
//     }
//
//     pub(crate) fn area(&self) -> usize {
//         return self.area;
//     }
// }

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
    )
}

pub(crate) trait RenderUnitUnwrap<W: Widget> {
    fn widget(&self) -> W;
    fn area(&self) -> usize;
}

impl RenderUnitUnwrap<Block<'_>> for RenderUnit<'static> {
    fn widget(&self) -> Block<'static> {
        return match self {
            RenderUnit::TUIBlock(block) => block.0.clone(),
            RenderUnit::TUIParagraph(paragraph) => unreachable!("Can't do that")
        };
    }

    fn area(&self) -> usize {
        return match self {
            RenderUnit::TUIBlock(block) => block.1,
            _ => unreachable!("Can't do that")
        }
    }
}

// ---------------------------------------------------------------------------------------------- //

pub(crate) struct UI<'ui_lt> {
    // 0 - Rect for requests log,
    // 1 - Rect for requests
    // 2 - Rect for responses
    // rectangles: [Rect; 3],
    pub(crate) widgets: Vec<RenderUnit<'ui_lt>>
}

impl UI<'_> {
    pub(crate) fn new() -> Self {
        let request_block = Block::default()
            .title("Upper Left Block")
            .borders(Borders::TOP);

        let response_block = Block::default()
            .title("Upper Right Block")
            .borders(Borders::TOP | Borders::LEFT);

        let proxy_history = Block::default()
            .title("Proxy History")
            .borders(Borders::NONE);

        UI {
            widgets: vec![
                RenderUnit::TUIBlock((request_block, 1)),
                RenderUnit::TUIBlock((response_block, 2)),
                RenderUnit::TUIBlock((proxy_history, 0))
            ]
        }
    }
}