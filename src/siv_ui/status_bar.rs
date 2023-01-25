use cursive::{
    views::{
        TextView,
        LinearLayout, ResizedView, TextContent,
    },
    align::{Align, },
    view::{Resizable, Nameable}, utils::{markup::StyledString}, theme::{Style, Effect, BaseColor, },
};
use std::fmt::Display;

type StatusBar = ResizedView<LinearLayout>;

pub(super) struct StatusBarContent {
    message: TextContent,
    stats: TextContent
}

impl Default for StatusBarContent {
    fn default() -> Self {
        StatusBarContent {
            message: TextContent::new(""),
            stats: TextContent::new("Press '?' to get help")
        }
    }
}

impl StatusBarContent {
    pub(super) fn new(m: TextContent, s: TextContent) -> Self {
        StatusBarContent {
            message: m.clone(),
            stats: s.clone()
        }
    }

    pub(super) fn set_message<T: Display>(&mut self, m: T) {
        self.message.set_content(StyledString::styled(format!(" {}", m), BaseColor::Black.light()));
    }

    pub(super) fn clear_message(&mut self) {
        self.message.set_content(StyledString::styled(" ", BaseColor::Black.light()));
    }

    pub(super) fn set_stats(&mut self, e: usize, r: usize) {
        self.stats.set_content(
            StyledString::styled(
                format!("Errors: {} ▾ Requests: {} ▾ Press '?' to get help ", e, r),
                Style::from(BaseColor::Black.light()).combine(Effect::Underline)
            )
        );
    }
}

pub(super) fn make_status_bar(left_content: TextContent, right_content: TextContent) -> StatusBar {
    let left_status_bar_part = TextView::new_with_content(left_content)
        .with_name("left-sb-part")
        .full_width();

    let right_status_bar_part = TextView::new_with_content(right_content)
        .align(Align::center_right())
        .with_name("right-sb-part")
        .full_width();

    let layout = LinearLayout::horizontal()
        .child(left_status_bar_part)
        .child(right_status_bar_part)
        .fixed_height(1);

    return layout;
}
