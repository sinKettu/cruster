use cursive::{View, Cursive, views::StackView};


pub(super) fn push_fullscreen_layer<V: View>(siv: &mut Cursive, view: V) {
    siv.call_on_name("views-stack", |sv: &mut StackView| { sv.add_fullscreen_layer(view); });
}

pub(super) fn push_layer<V: View>(siv: &mut Cursive, view: V) {
    siv.call_on_name("views-stack", |sv: &mut StackView| { sv.add_layer(view); });
}

pub(super) fn pop_layer(siv: &mut Cursive) {
    siv.call_on_name("views-stack", |sv: &mut StackView| { sv.pop_layer(); });
}
