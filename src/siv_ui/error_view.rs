use cursive::{
    Cursive,
    views::{
        TextView,
        OnEventView,
        Dialog
    },
    align::HAlign,
    view::Resizable,
};

pub(super) fn draw_error_view(siv: &mut Cursive) {

    let errors = TextView::new("Errors\n");
    let errors = OnEventView::new(errors)
        .on_event('q', |s| { s.pop_layer(); })
        .on_event('?', |_| {})
        .on_event('e', |_| {});
    let errors = Dialog::around(errors).title("Errors")
        .title_position(HAlign::Center)
        .full_screen();

    siv.add_fullscreen_layer(errors);
}
