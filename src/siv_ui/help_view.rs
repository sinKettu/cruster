use cursive::{
    Cursive,
    views::{TextView, OnEventView, Dialog},
    align::HAlign,
    view::{Resizable, Nameable},
    utils::span::SpannedString,
    theme::{Style, Effect, BaseColor},
};
use std::{rc::Rc, ops::Deref};
use super::sivuserdata::SivUserData;

pub(super) fn make_help_message() -> SpannedString<Style> {
    let letters_style: Style = BaseColor::Green.light().into();
    let descriptions_style: Style = Effect::Bold.into();
    let txt: Vec<SpannedString<Style>> = vec![
        SpannedString::styled( "? - ",letters_style.clone()),
        SpannedString::styled("Show this help view\n", descriptions_style.clone()),

        SpannedString::styled("<Enter> - ", letters_style.clone()),
        SpannedString::styled("\n    <On Proxy Table> - ", BaseColor::Yellow.dark()),
        SpannedString::styled("Show interactive fullscreen view for selected request and response contents", descriptions_style.clone()),
        SpannedString::styled("\n    <On Filter View> - ", BaseColor::Yellow.dark()),
        SpannedString::styled("Apply written filter", descriptions_style.clone()),
        SpannedString::styled("\n    <On Repeater View> - ", BaseColor::Yellow.dark()),
        SpannedString::styled("Apply edited request / Send\n", descriptions_style.clone()),

        SpannedString::styled("<Esc> - ", letters_style.clone()),
        SpannedString::styled("Close secondary view (i.e. help, errors, etc.)\n", descriptions_style.clone()),

        SpannedString::styled("<Shift> + r - ", letters_style.clone()),
        SpannedString::styled("Repeat request selected on table\n", descriptions_style.clone()),

        SpannedString::styled("<Shift> + s - ", letters_style.clone()),
        SpannedString::styled("Store proxy data on drive, file path is configured on start\n", descriptions_style.clone()),

        SpannedString::styled("<Shift> + f - ", letters_style.clone()),
        SpannedString::styled("Set filter for table\n", descriptions_style.clone()),

        SpannedString::styled("e - ", letters_style.clone()),
        SpannedString::styled("Show error logs view\n", descriptions_style.clone()),

        SpannedString::styled("i - ", letters_style.clone()),
        SpannedString::styled("\n    <On Repeater View> - ", BaseColor::Yellow.dark()),
        SpannedString::styled("Edit request\n", descriptions_style.clone()),

        SpannedString::styled("p - ", letters_style.clone()),
        SpannedString::styled("\n    <On Repeater View> - ", BaseColor::Yellow.dark()),
        SpannedString::styled("Show parameters\n", descriptions_style.clone()),

        SpannedString::styled("r - ", letters_style.clone()),
        SpannedString::styled("Show active repeaters\n", descriptions_style.clone()),

        SpannedString::styled("t - ", letters_style.clone()),
        SpannedString::styled("Show fullscreen HTTP proxy table\n", descriptions_style.clone()),

        SpannedString::styled("q - ", letters_style.clone()),
        SpannedString::styled("Quit\n", descriptions_style.clone()),
    ];

    let mut result = SpannedString::<Style>::default();
    for item in txt {
        result.append(item);
    }

    return result;
}

pub(super) fn draw_help_view(siv: &mut Cursive, content: &Rc<SpannedString<Style>>) {
    if siv.find_name::<TextView>("help-popup").is_some() { return; }

    siv.with_user_data(|ud: &mut SivUserData| { ud.status.clear_message(); });

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
