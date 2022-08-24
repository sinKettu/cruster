mod ui_storage;
mod ui_events;

use std::{io, time::{Duration, Instant}};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use tokio::sync::mpsc::Receiver;

use crate::cruster_proxy::request_response::CrusterWrapper;
use ui_storage::render_units;
use super::http_storage::*;

use tui::{
    backend::{CrosstermBackend, Backend},
    // widgets::{Widget, Block, Borders, Paragraph, Wrap, Table, Row},
    layout::{Rect/*, Alignment*/},
    // layout::{Layout, Constraint, Direction, Rect},
    Terminal,
    // text,
    Frame,
    self
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::debug;
use tui::widgets::Widget;
use crate::CrusterError;
use crate::ui::ui_events::UIEvents;

// https://docs.rs/tui/latest/tui/widgets/index.html

pub(crate) fn render(
        ui_rx: Receiver<(CrusterWrapper, usize)>,
        err_rx: Receiver<CrusterError>) -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let tick_rate = Duration::from_millis(0);
    let mut terminal = Terminal::new(backend)?;

    return match run_app(&mut terminal, tick_rate, ui_rx, err_rx) {
        Ok(_) => {
            // restore terminal
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            terminal.show_cursor()?;
            Ok(())
        },
        Err(err) => {
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            terminal.show_cursor()?;
            Err(err.into())
        }
    }
}

fn run_app<B: Backend>(
                    terminal: &mut Terminal<B>,
                    tick_rate: Duration,
                    mut ui_rx: Receiver<(CrusterWrapper, usize)>,
                    mut err_rx: Receiver<CrusterError>,
) -> io::Result<()> {
    let mut last_tick = Instant::now();

    let mut ui_storage = ui_storage::UI::new();
    let mut ui_events = UIEvents::default();
    let mut http_storage = HTTPStorage::default();

    loop {
        match ui_rx.try_recv() {
            Ok((wrapper, ctx)) => {
                match wrapper {
                    CrusterWrapper::Request(request) => http_storage.put_request(request, ctx),
                    CrusterWrapper::Response(response) => http_storage.put_response(response, &ctx)
                }
                ui_events.something_changed = true;
                ui_events.table_state_changed = true;
            },
            Err(recv_err) => {
                // something_changed = true;
            }
        }

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if ui_events.process_event(&mut ui_storage, &mut http_storage) {
                return Ok(());
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }

        if ui_events.something_changed {
            ui_storage.draw_statusbar(&http_storage);
            ui_storage.draw_state(&http_storage);

            if ui_events.table_state_changed { ui_storage.make_table(&http_storage); }

            terminal.draw(|f| new_ui(f, &mut ui_storage))?;

            ui_events.something_changed = false;
            ui_events.table_state_changed = false;
        }
    }
}

fn new_ui<B: Backend>(f: &mut Frame<B>, uis: &mut ui_storage::UI<'static>) {
    let window_width = f.size().width;
    let window_height = f.size().height;

    // 0 - Rect for requests log,
    // 1 - Rect for requests
    // 2 - Rect for responses
    // 3 - Rect for statusbar
    // 4 - Rect for help menu
    // 5 - Rect for Proxy FullScreen
    // 6 - Rect for Request FullScreen
    // 7 - Rect for Response FullScreen
    let rects: [Rect; 8] = [
        Rect::new(
            f.size().x,
            f.size().y,
            window_width,
            window_height / 2
        ),
        Rect::new(
            f.size().x,
            f.size().y + window_height / 2,
            window_width / 2,
            window_height / 2 - 2
        ),
        Rect::new(
            f.size().x + window_width / 2,
            f.size().y + window_height / 2,
            window_width / 2 + 1,
            window_height / 2 - 2
        ),
        Rect::new(
            f.size().x,
            f.size().y + window_height - 2,
            window_width,
            2
        ),
        Rect::new(
            f.size().x + 5,
            f.size().y + 5,
            window_width - 10,
            window_height - 10
        ),
        Rect::new(
            f.size().x,
            f.size().y,
            window_width,
            window_height - 2
        ),
        Rect::new(
            f.size().x,
            f.size().y,
            window_width / 2,
            window_height - 2
        ),
        Rect::new(
            f.size().x + window_width / 2,
            f.size().y,
            window_width / 2,
            window_height - 2
        )
    ];

    for ruint in uis.widgets.iter() {
        debug!("Render units handling cycle: {:?}", ruint);
        match ruint {
            render_units::RenderUnit::TUIBlock(block) => {
                if ! block.is_active { continue; }
                f.render_widget(block.clear_widget.clone(), rects[block.rect_index]);
                f.render_widget(block.widget.clone(), rects[block.rect_index]);
                debug!("Render units handling cycle: handled");
            },
            render_units::RenderUnit::TUIParagraph(paragraph) => {
                if ! paragraph.is_active { continue; }
                f.render_widget(paragraph.clear_widget.clone(), rects[paragraph.rect_index]);
                f.render_widget(paragraph.widget.clone(), rects[paragraph.rect_index]);
                debug!("Render units handling cycle: handled");
            }
            render_units::RenderUnit::TUIClear(clear) => {
                if ! clear.is_active { continue; }
                f.render_widget(clear.widget.clone(), rects[clear.rect_index]);
                debug!("Render units handling cycle: handled");
            },
            render_units::RenderUnit::TUITable(table) => {
                if ! table.is_active { continue; }
                f.render_widget(table.clear_widget.clone(), rects[table.rect_index]);
                f.render_stateful_widget(table.widget.clone(), rects[table.rect_index], &mut uis.proxy_history_state);
                debug!("Render units handling cycle: handled");
            },
            _ => {debug!("Render units handling cycle: Skipped")},
        }
    }
}
