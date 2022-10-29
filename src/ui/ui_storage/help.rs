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

pub(super) fn make_help_menu<'help>(rect_index: usize) -> RenderUnit<'help> {
    // abcdefghijklmnopqrstuvwxyz
    let help_text: Vec<Spans> = vec![
        Spans::from(vec![
            Span::styled(
                "[Enter]",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)),
            Span::raw(" - Save edited text")
        ]),
        Spans::from(vec![
            Span::styled(
                "[Esc]",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)),
            Span::raw(" - Stop editing text")
        ]),
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
                "e",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)),
            Span::raw(" - Show error log (on main screen) / Start editing text")
        ]),
        Spans::from(vec![
            Span::styled(
                "f",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)),
            Span::raw(" - Enter/Quit fullscreen mode for proxy, request, response view")
        ]),
        Spans::from(vec![
            Span::styled(
                "F",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)),
            Span::raw(" - Set filter for proxy data")
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
        Spans::from(vec![
            Span::styled(
                "u",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)),
            Span::raw(" - Reveal hidden body")
        ]),
    ];

    let help_block = Block::default()
        .title(Span::styled(
            " HELP ",
            Style::default()
                .bg(Color::LightGreen)
                .fg(Color::Black)))
        .title_alignment(Alignment::Center)
        // .style(Style::default().fg(Color::Green))
        .borders(Borders::ALL);

    let help_paragraph = Paragraph::new(help_text)
        .block(help_block);

    let help = RenderUnit::new_paragraph(
        help_paragraph,
        rect_index,
        true,
        (0, 0)
    );

    return help;
}