use tui::layout::Alignment;
use tui::style::Style;
use tui::widgets::{Block, Borders, Paragraph};
use crate::CrusterError;
use crate::http_storage::{HTTPStorage, RequestResponsePair};
use super::UI;
use super::render_units::RenderUnit;

impl UI<'static> {
    // pub(crate) request_to_spans

    fn make_repeater_request_content(&mut self, http_storage: &HTTPStorage) -> Option<String> {
        let selected = self.proxy_history_state.selected();

        return match selected {
            Some(index) => {
                let get_pair_result = http_storage.get_pair_from_cache(index);

                match get_pair_result {
                    Ok(pair) => {
                        match pair.request.as_ref() {
                            Some(request) => {
                                Some(request.to_string())
                            },
                            None => {
                                self.log_error(CrusterError::EmptyRequest(format!("Could not process request in repeater - it's empty (id - {})", index)));
                                None
                            }
                        }
                    },
                    Err(e) => {
                        self.log_error(e);
                        None
                    }
                }
            },
            None => None
        };
    }

    pub(crate) fn show_repeater(&mut self, http_storage: &HTTPStorage) {
        let request_content = self.make_repeater_request_content(http_storage);
        if request_content.is_none() {
            return;
        }

        let request_paragrpah = Paragraph::new(request_content.unwrap())
            .block(
                Block::default()
                    .title(" Request ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
            );

        self.widgets[self.repeater_req_block] = RenderUnit::new_paragraph(
            request_paragrpah,
            self.repeater_req_area,
            true,
            (0, 0)
        );

        self.widgets[self.repeater_res_block] = RenderUnit::new_paragraph(
            Paragraph::new("").block(
                Block::default()
                    .title(" Response ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
            ),
            self.repeater_res_area,
            true,
            (0, 0)
        );
    }

    pub(crate) fn hide_repeater(&mut self) {
        self.widgets[self.repeater_req_block] = RenderUnit::PLACEHOLDER;
        self.widgets[self.repeater_res_block] = RenderUnit::PLACEHOLDER;
    }
}