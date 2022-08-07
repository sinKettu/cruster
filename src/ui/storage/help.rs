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
                "q",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)),
            Span::raw(" - Quit / Go back")
        ]),
    ];

    let help_block = Block::default()
        .title("HELP")
        .title_alignment(Alignment::Center)
        // .style(Style::default().fg(Color::Green))
        .borders(Borders::ALL);

    let help_paragraph = Paragraph::new(help_text)
        .block(help_block);

    // let mut clear = UniversalRenderUnit::make_clear(rect_index);
    let mut clear = RenderUnit::new_clear(rect_index);
    // let help = UniversalRenderUnit::make_paragraph(help_paragraph, rect_index, true);
    let help = RenderUnit::new_paragraph(help_paragraph, rect_index, false);
    clear.disable();

    return (clear, help);
}