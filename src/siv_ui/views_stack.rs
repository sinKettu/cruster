use cursive::{View, Cursive, views::StackView};

use super::sivuserdata::GetCrusterUserData;


pub(super) fn push_fullscreen_layer<V: View>(siv: &mut Cursive, view: V) {
    siv.get_cruster_userdata().status.clear_message();
    siv.call_on_name("views-stack", |sv: &mut StackView| { sv.add_fullscreen_layer(view); });
}

pub(super) fn push_layer<V: View>(siv: &mut Cursive, view: V) {
    siv.get_cruster_userdata().status.clear_message();
    siv.call_on_name("views-stack", |sv: &mut StackView| { sv.add_layer(view); });
}

pub(super) fn pop_layer(siv: &mut Cursive) {
    siv.get_cruster_userdata().status.clear_message();
    siv.call_on_name("views-stack", |sv: &mut StackView| { sv.pop_layer(); });
}
