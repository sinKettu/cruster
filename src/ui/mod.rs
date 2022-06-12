mod storage;

use std::{io, time::{Duration, Instant}};
use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{Widget, Block, Borders, Paragraph, Wrap, Table, Row},
    layout::{Rect, Alignment},
//    layout::{Layout, Constraint, Direction, Rect},
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
use crossterm::event::KeyCode::Tab;
use tokio::sync::mpsc::Receiver;
use crate::cruster_handler::request_response::CrusterWrapper;
use storage::RenderUnitUnwrap;

pub(crate) async fn render(mut ui_rx: Receiver<CrusterWrapper>) -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let tick_rate = Duration::from_millis(250);
    let mut terminal = Terminal::new(backend)?;

    run_app(&mut terminal, tick_rate, ui_rx).await?;

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

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    tick_rate: Duration,
    mut ui_rx: Receiver<CrusterWrapper>
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut ui_storage = storage::UI::new();
    loop {
        match ui_rx.try_recv() {
            Ok(wrapper) => {
                // terminal.draw(|f| ui(f, Some(wrapper)))?;
                terminal.draw(|f| new_ui(f, &mut ui_storage))?;
            },
            Err(e) => ()
        }

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn new_ui<B: Backend>(f: &mut Frame<B>, uis: &mut storage::UI<'static>) {
    let window_width = f.size().width;
    let window_height = f.size().height;

    // 0 - Rect for requests log,
    // 1 - Rect for requests
    // 2 - Rect for responses
    let mut rects: [Rect; 3] = [
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
            window_height / 2)
        ,
        Rect::new(
            f.size().x + window_width / 2,
            f.size().y + window_height / 2,
            window_width / 2,
            window_height / 2
        )
    ];

    for ruint in uis.widgets.iter() {
        let area_index = ruint.area();
        f.render_widget(ruint.widget(), rects[area_index]);
    }
}

// fn ui<B: Backend>(f: &mut Frame<B>, thread_message: Option<CrusterWrapper>) {
//     let window_width = f.size().width;
//     let window_height = f.size().height;
//     let lower_left_rect = Rect::new(f.size().x, f.size().y + window_height / 2, window_width / 2, window_height / 2);
//     let lower_right_rect = Rect::new(
//         f.size().x + window_width / 2,
//         f.size().y + window_height / 2,
//         window_width / 2,
//         window_height / 2
//     );
//     let upper_rect = Rect::new(
//         f.size().x,
//         f.size().y,
//         window_width,
//         window_height / 2
//     );
//
//     let request_block = Block::default()
//         .title("Upper Left Block")
//         .borders(Borders::TOP);
//     f.render_widget(request_block, lower_left_rect);
//
//     let response_block = Block::default()
//         .title("Upper Right Block")
//         .borders(Borders::TOP | Borders::LEFT);
//     f.render_widget(response_block, lower_right_rect);
//
//     let proxy_history = Block::default()
//         .title("Proxy History")
//         .borders(Borders::NONE);
//     f.render_widget(proxy_history.clone(), upper_rect);
//
//     if let Some(wrapper) = thread_message {
//         let row = match wrapper {
//             CrusterWrapper::Request(req) => Row::new(vec![req.uri]),
//             CrusterWrapper::Response(rsp) => Row::new(vec![rsp.status])
//         };
//
//         let rows= vec![row];
//         let table = Table::new(rows);
//
//         f.render_widget(table, upper_rect);
//     }
// }
