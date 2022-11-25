use cursive::{
    Cursive,
    views::{
        TextView,
        OnEventView,
        Dialog,
        NamedView,
        ResizedView
    },
    align::HAlign,
    view::{
        Resizable,
        Nameable
    },
    event::Key
};

pub(super) fn draw_error_view(siv: &mut Cursive) {
    if siv.find_name::<TextView>("errors-popup").is_some() { return; }

    let errors = TextView::new("Errors\n")
        .with_name("errors-popup");
    let errors = OnEventView::new(errors)
        .on_event(Key::Esc, |s| { s.pop_layer(); });

    let errors = Dialog::around(errors).title("Errors")
        .title_position(HAlign::Center)
        .full_screen();

    siv.add_fullscreen_layer(errors);
}
