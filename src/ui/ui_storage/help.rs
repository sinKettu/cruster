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

use super::render_units::*;

pub(super) fn make_help_menu<'help>(rect_index: usize) -> (RenderUnit<'help>, RenderUnit<'help>) {
    // abcdefghijklmnopqrstuvwxyz
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
                "p",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)),
            Span::raw(" - Select (activate) proxy history screen")
        ]),
        Spans::from(vec![
            Span::styled(
                "q",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)),
            Span::raw(" - Quit / Go back")
        ]),
        Spans::from(vec![
            Span::styled(
                "r",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)),
            Span::raw(" - Select (activate) request screen")
        ]),
        Spans::from(vec![
            Span::styled(
                "s",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)),
            Span::raw(" - Select (activate) response screen")
        ]),
    ];

    let help_block = Block::default()
        .title("HELP")
        .title_alignment(Alignment::Center)
        // .style(Style::default().fg(Color::Green))
        .borders(Borders::ALL);

    let help_paragraph = Paragraph::new(help_text)
        .block(help_block);

    let mut clear = RenderUnit::new_clear(rect_index);
    let help = RenderUnit::new_paragraph(help_paragraph, rect_index, false);
    clear.disable();

    return (clear, help);
}