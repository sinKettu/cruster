///
/// Module to store statusbar messages statically
///

use super::FILTER_MAIN_COLOR;

use tui::style::{Modifier, Style};
use tui::style::Color;
use tui::text::{Span, Spans};


pub(crate) fn filter_chown() -> Span<'static> {
    Span::styled(
        "'e' to start editing  |  'Esc' to exit  |  'Enter' to save  ",
        Style::default().fg(FILTER_MAIN_COLOR)
    )
}

pub(crate) fn editing_tips() -> Span<'static> {
    Span::styled(
        "'Esc' to stop  |  'Enter' to save  ",
        Style::default().fg(FILTER_MAIN_COLOR),
    )
}

pub(crate) fn confirmation_tips() -> Spans<'static> {
    Spans::from(vec![
        Span::from("Press "),
        Span::styled(
            "y ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)),
        Span::from("or "),
        Span::styled(
            "n ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)),
        Span::from("  "),
    ])
}