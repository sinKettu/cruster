pub(super) mod ui_storage;
mod ui_events;
mod ui_layout;

use std::{io, time::{Duration, Instant}};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use tokio::sync::mpsc::Receiver;

use crate::cruster_proxy::request_response::CrusterWrapper;
use ui_storage::render_units;
use super::http_storage::*;

use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{Paragraph, Widget, Wrap},
    layout::{Rect/*, Alignment*/},
    Terminal,
    text::{Span, Spans},
    style::{Style, Color},
    Frame,
    self
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use log::debug;
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
    let mut terminal = Terminal::new(backend)?;

    return match run_app(&mut terminal, ui_rx, err_rx) {
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
                    mut ui_rx: Receiver<(CrusterWrapper, usize)>,
                    mut err_rx: Receiver<CrusterError>,
) -> io::Result<()> {
    let mut ui_storage = ui_storage::UI::new();
    let mut ui_events = UIEvents::default();
    let mut http_storage = HTTPStorage::default();

    loop {
        match err_rx.try_recv() {
            Ok(error) => {
                ui_storage.log_error(error);
            }
            Err(_) => {}
        }

        match ui_rx.try_recv() {
            Ok((wrapper, ctx)) => {
                match wrapper {
                    CrusterWrapper::Request(request) => {
                        http_storage.put_request(request, ctx)
                    },
                    CrusterWrapper::Response(response) => {
                        http_storage.put_response(response, &ctx)
                    }
                }
                // ui_events.something_changed = true;
                ui_events.table_state_changed = true;
            },
            Err(_) => {
                // something_changed = true;
            }
        }

        if crossterm::event::poll(Duration::from_nanos(0))? {
            if ui_events.process_event(&mut ui_storage, &mut http_storage) {
                return Ok(());
            }
        }

        if ui_events.something_changed {
            ui_storage.draw_state(&http_storage);
            ui_events.something_changed = false;
        }

        if ui_events.table_state_changed {
            ui_storage.make_table(&mut http_storage, terminal.get_frame().size());
            ui_events.table_state_changed = false;
        }

        terminal.draw(|f| new_ui(f, &mut ui_storage, ui_events.input_mode))?;
    }
}

fn new_ui<B: Backend>(f: &mut Frame<B>, uis: &mut ui_storage::UI<'static>, input_mode: bool) {
    // Show only warning if terminal size too small
    if f.size().width < 60u16 || f.size().height < 20u16 {
        let width = f.size().width;
        let height = f.size().height;

        let rect = Rect::new(
            0u16,
            0u16,
            width - 1,
            height - 1
        );

        let too_small_message = Paragraph::new(Spans::from(vec![
            Span::from("Terminal size must be not less then "),
            Span::styled("60 x 20", Style::default().fg(Color::LightGreen)),
            Span::from(", the current is "),
            Span::styled(format!("{} x {}", width, height), Style::default().fg(Color::LightRed)),
        ]))
            .wrap(Wrap { trim: true });

        f.render_widget(too_small_message, rect);
        return;
    }

    // 0 - Rect for requests log,
    // 1 - Rect for requests
    // 2 - Rect for responses
    // 3 - Rect for statusbar
    // 4 - Rect for help menu
    // 5 - Rect for Proxy FullScreen
    let rects = ui_layout::CrusterLayout::new(f.size().borrow());

    // Show cursor when user edit some text
    if input_mode {
        if let Some(editable_area) = uis.get_currently_edited_area() {
            let (x_offset, y_offset) = uis.get_cursor_relative_position();
            f.set_cursor(
                rects[editable_area].x + x_offset as u16,
                rects[editable_area].y + y_offset as u16
            );
        }
    }

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
            _ => {
                debug!("Render units handling cycle: Skipped")
            },
        }
    }
}
