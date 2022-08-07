mod ui_storage;

use std::{io, time::{Duration, Instant}};
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
use tui::widgets::Widget;

// https://docs.rs/tui/latest/tui/widgets/index.html

pub(crate) fn render(ui_rx: Receiver<(CrusterWrapper, usize)>) -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let tick_rate = Duration::from_millis(0);
    let mut terminal = Terminal::new(backend)?;

    run_app(&mut terminal, tick_rate, ui_rx)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(
                    terminal: &mut Terminal<B>,
                    tick_rate: Duration,
                    mut ui_rx: Receiver<(CrusterWrapper, usize)>
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut ui_storage = ui_storage::UI::new();

    // Flags
    let mut something_changed = true;
    let mut table_state_changed = false;
    let mut help_enabled = false;

    let mut http_storage = HTTPStorage::default();

    loop {
        match ui_rx.try_recv() {
            Ok((wrapper, ctx)) => {
                match wrapper {
                    CrusterWrapper::Request(request) => http_storage.put_request(request, ctx),
                    CrusterWrapper::Response(response) => http_storage.put_response(response, &ctx)
                }
                something_changed = true;
            },
            Err(_) => {
                // something_changed = true;
            }
        }

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    if help_enabled {
                        ui_storage.hide_help();
                        something_changed = true;
                        help_enabled = false;
                    }
                    else {
                        return Ok(());
                    }
                }
                else if let KeyCode::Up = key.code {
                    let index = match ui_storage.proxy_history_state.selected() {
                        Some(i) => if i == 0 { 0 } else { i - 1 },
                        None => 0 as usize
                    };

                    ui_storage.proxy_history_state.select(Some(index));
                    table_state_changed = true;
                    something_changed = true
                }
                else if let KeyCode::Down = key.code {
                    let index = match ui_storage.proxy_history_state.selected() {
                        Some(i) => if i >= http_storage.len() - 1 { http_storage.len() - 1 } else { i + 1 },
                        None => 0 as usize
                    };

                    ui_storage.proxy_history_state.select(Some(index));
                    table_state_changed = true;
                    something_changed = true
                }
                else if let KeyCode::Char('?') = key.code {
                    ui_storage.show_help();
                    help_enabled = true;
                    something_changed = true;
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }

        if something_changed {
            ui_storage.draw_statusbar(&http_storage);
            ui_storage.update_table(&http_storage);

            if table_state_changed { ui_storage.draw_state(&http_storage); }

            terminal.draw(|f| new_ui(f, &mut ui_storage))?;

            something_changed = false;
            table_state_changed = false;
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
    let rects: [Rect; 5] = [
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
            window_height / 2 - 3
        ),
        Rect::new(
            f.size().x + window_width / 2,
            f.size().y + window_height / 2,
            window_width / 2,
            window_height / 2 - 3
        ),
        Rect::new(
            f.size().x,
            f.size().y + window_height - 3,
            window_width,
            3
        ),
        Rect::new(
            f.size().x + 5,
            f.size().y + 5,
            window_width - 10,
            window_height - 10
        )
    ];

    for ruint in uis.widgets.iter() {
        match ruint {
            render_units::RenderUnit::TUIBlock(block) => {
                if ! block.is_active { continue; }
                f.render_widget(block.widget.clone(), rects[block.rect_index]);
            },
            render_units::RenderUnit::TUIParagraph(paragraph) => {
                if ! paragraph.is_active { continue; }
                f.render_widget(paragraph.widget.clone(), rects[paragraph.rect_index]);
            }
            render_units::RenderUnit::TUIClear(clear) => {
                if ! clear.is_active { continue; }
                f.render_widget(clear.widget.clone(), rects[clear.rect_index]);
            },
            render_units::RenderUnit::TUITable(table) => {
                if ! table.is_active { continue; }
                f.render_stateful_widget(table.widget.clone(), rects[table.rect_index], &mut uis.proxy_history_state);
            },
            _ => {},
        }
    }
}
