use cursive::{
    Cursive,
    views::{
        TextView,
        OnEventView,
        Dialog
    },
    align::HAlign,
    view::Resizable, utils::span::SpannedString, theme::{Style, Effect, BaseColor},
};
use std::{rc::Rc, ops::Deref};

pub(super) fn make_help_message() -> SpannedString<Style> {
    let letters_style: Style = BaseColor::Green.light().into();
    let descriptions_style: Style = Effect::Bold.into();
    let txt: Vec<SpannedString<Style>> = vec![
        SpannedString::styled( "? - ",letters_style.clone()),
        SpannedString::styled("Show this help view\n", descriptions_style.clone()),

        SpannedString::styled("e - ", letters_style.clone()),
        SpannedString::styled("Show error logs view\n", descriptions_style.clone()),
    ];

    let mut result = SpannedString::<Style>::default();
    for item in txt {
        result.append(item);
    }

    return result;
}

pub(super) fn draw_help_view(siv: &mut Cursive, content: &Rc<SpannedString<Style>>) {
    let errors = TextView::new(content.deref().clone());
    let errors = OnEventView::new(errors)
        .on_event('q', |s| { s.pop_layer(); })
        .on_event('?', |_| {})
        .on_event('e', |_| {});
    let errors = Dialog::around(errors).title("Errors")
        .title_position(HAlign::Center)
        .full_screen();

    siv.add_fullscreen_layer(errors);
}
