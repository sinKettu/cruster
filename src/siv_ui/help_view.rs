use cursive::{
    Cursive,
    views::{
        TextView,
        OnEventView,
        Dialog,
    },
    align::HAlign,
    view::{Resizable, Nameable}, utils::span::SpannedString, theme::{Style, Effect, BaseColor},
};
use std::{rc::Rc, ops::Deref};

pub(super) fn make_help_message() -> SpannedString<Style> {
    let letters_style: Style = BaseColor::Green.light().into();
    let descriptions_style: Style = Effect::Bold.into();
    let txt: Vec<SpannedString<Style>> = vec![
        SpannedString::styled( "? - ",letters_style.clone()),
        SpannedString::styled("Show this help view\n", descriptions_style.clone()),

        SpannedString::styled("q - ", letters_style.clone()),
        SpannedString::styled("Quit\n", descriptions_style.clone()),

        SpannedString::styled("e - ", letters_style.clone()),
        SpannedString::styled("Show error logs view\n", descriptions_style.clone()),

        SpannedString::styled("t - ", letters_style.clone()),
        SpannedString::styled("Show fullscreen HTTP proxy table\n", descriptions_style.clone()),

        SpannedString::styled("<Enter> - ", letters_style.clone()),
        SpannedString::styled("\n    <On Proxy Table> - ", BaseColor::Yellow.dark()),
        SpannedString::styled("Show interactive fullscreen view for selected request and response contents\n", descriptions_style.clone()),

        SpannedString::styled("<Esc> - ", letters_style.clone()),
        SpannedString::styled("Close secondary view (i.e. help, errors, etc.)\n", descriptions_style.clone()),

    ];

    let mut result = SpannedString::<Style>::default();
    for item in txt {
        result.append(item);
    }

    return result;
}

pub(super) fn draw_help_view(siv: &mut Cursive, content: &Rc<SpannedString<Style>>) {
    if siv.find_name::<TextView>("help-popup").is_some() { return; }

    let help = TextView::new(content.deref().clone())
        .with_name("help-popup");
    let help = OnEventView::new(help)
        .on_event(cursive::event::Key::Esc, |s| { s.pop_layer(); });

    let help = Dialog::around(help)
        .title("Help")
        .title_position(HAlign::Center)
        .full_screen();

    siv.add_fullscreen_layer(help);
}
