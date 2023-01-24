use cursive::{
    Cursive,
    views::{
        TextView,
        OnEventView,
        Dialog,
    },
    align::HAlign,
    view::{
        Resizable,
        Nameable
    },
    event::Key, utils::span::SpannedString, theme::{Style, BaseColor}
};
use super::SivUserData;

pub(crate) fn put_error(siv: &mut Cursive) {
    siv.with_user_data(|ud: &mut SivUserData| {
        let error_message = ud.proxy_err_receiver.try_recv();
        if let Ok(err) = error_message {
            ud.errors.push(err);
        }

        ud.status.set_stats(ud.errors.len(), ud.http_storage.len())
    });
}

pub(super) fn draw_error_view(siv: &mut Cursive) {
    if siv.find_name::<TextView>("errors-popup").is_some() { return; }

    let ud: &mut SivUserData = siv.user_data().unwrap();
    ud.status.clear_message();

    let mut err_msg = SpannedString::new();
    for e in ud.errors.iter() {
        err_msg.append_styled(">>> ", Style::from(BaseColor::Red.light()));
        err_msg.append(e.to_string());
        err_msg.append("\n");
    }

    let errors = TextView::new(err_msg)
        .with_name("errors-popup");
    let errors = OnEventView::new(errors)
        .on_event(Key::Esc, |s| { s.pop_layer(); });

    let errors = Dialog::around(errors).title("Errors")
        .title_position(HAlign::Center)
        .full_screen();

    siv.add_fullscreen_layer(errors);
}
