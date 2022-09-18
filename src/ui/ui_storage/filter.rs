use super::{
    UI,
    FILTER_MAIN_COLOR,
    render_units::RenderUnit
};

use tui::{
    text::{Span, Spans},
    style::{Style, Color, Modifier},
    widgets::{Paragraph, Block, Borders, BorderType},
    layout::Alignment,
};

pub(super) const DEFAULT_MESSAGE_LENGTH: usize = 4_usize;

impl<'ui_lt> UI<'ui_lt> {
    pub(crate) fn show_filter(&mut self) {
        let filter_text: Vec<Spans> = vec![
            Spans::from(""),
            Spans::from(
                vec![
                    Span::styled("  ~ ", Style::default().fg(FILTER_MAIN_COLOR).add_modifier(Modifier::BOLD)),
                    Span::from(self.input_buffer.clone())
                ]
            )
        ];

        let filter_paragraph = Paragraph::new(
            filter_text
        ).block(
            Block::default()
                .title(" Filter ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(FILTER_MAIN_COLOR))
                .border_type(BorderType::Double)
        );

        let filter = RenderUnit::new_paragraph(
            filter_paragraph,
            self.filter_area,
            true,
            (0, 0)
        );

        self.widgets[self.filter_block] = filter;
        self.editable_area = Some(self.filter_area);
    }

    pub(crate) fn hide_filter(&mut self) {
        self.widgets[self.filter_block] = RenderUnit::PLACEHOLDER;
        self.editable_area = None;
    }

    pub(crate) fn save_filter(&mut self) {
        if self.input_buffer.len() == 0 {
            self.filter = None;
        }
        else {
            self.filter = Some(self.input_buffer.clone());
        }
    }
}