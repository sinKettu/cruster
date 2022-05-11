use std::{io, time::{Duration, Instant}};
use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{/*Widget, */Block, Borders, Paragraph, Wrap},
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
use tokio::sync::mpsc::Receiver;
use crate::cruster_handler::request_response::HyperRequestWrapper;

pub(crate) async fn render(mut ui_rx: Receiver<HyperRequestWrapper>) -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let tick_rate = Duration::from_millis(250);
    let mut terminal = Terminal::new(backend)?;

    run_app(&mut terminal, tick_rate).await?;

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
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f))?;

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

fn ui<B: Backend>(f: &mut Frame<B>) {
    let window_width = f.size().width;
    let window_height = f.size().height;
    let lower_left_rect = Rect::new(f.size().x, f.size().y + window_height / 2, window_width / 2, window_height / 2);
    let lower_right_rect = Rect::new(
        f.size().x + window_width / 2,
        f.size().y + window_height / 2,
        window_width / 2,
        window_height / 2
    );
    let upper_rect = Rect::new(
        f.size().x,
        f.size().y,
        window_width,
        window_height / 2
    );

    // let size = f.size();
    let request_block = Block::default()
        .title("Upper Left Block")
        .borders(Borders::TOP);
    f.render_widget(request_block, lower_left_rect);

    let response_block = Block::default()
        .title("Upper Right Block")
        .borders(Borders::TOP | Borders::LEFT);
    f.render_widget(response_block, lower_right_rect);

    let proxy_history = Block::default()
        .title("Proxy History")
        .borders(Borders::NONE);
    f.render_widget(proxy_history.clone(), upper_rect);

    let text = vec![
        text::Spans::from("Hello"),
        text::Spans::from("World"),
        text::Spans::from("!")
    ];

    let para = Paragraph::new(text)
        .block(proxy_history)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true} );

    f.render_widget(para, upper_rect);
}
