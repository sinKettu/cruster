use cursive::{Cursive,
    view::Nameable,
    theme::BaseColor,
    utils::markup::StyledString,
    views::{
        TextView,
        Dialog,
        StackView
    },
};

    use super::sivuserdata::SivUserData;

fn save_and_quit(siv: &mut Cursive) {
    super::store_cruster_state(siv);
    siv.quit();
}

fn popup_quit_and_save(siv: &mut Cursive) {
    let styled_text = StyledString::styled(
        "\nSave Cruster state before quit?",
        BaseColor::Yellow.light()
    );
    let txt = TextView::new(styled_text);

    let dialog_with_txt = Dialog::around(txt)
        .title("Quit")
        .button("Cancel", |s: &mut Cursive| { hide_popup(s) })
        .button("Yes", |s: &mut Cursive| { save_and_quit(s); })
        .button("No", |s: &mut Cursive| { s.quit(); })
        .h_align(cursive::align::HAlign::Center)
        .with_name("quit-popup");

    siv.call_on_name("views-stack", |sv: &mut StackView| { sv.add_layer(dialog_with_txt) });
}

fn popup_quit(siv: &mut Cursive) {
    let styled_text = StyledString::styled(
        "\n         Are you sure?         ",
        BaseColor::Yellow.light()
    );
    let txt = TextView::new(styled_text);

    let dialog_with_txt = Dialog::around(txt)
        .title("Quit")
        .button("No", |s: &mut Cursive| { hide_popup(s); })
        .button("Yes", |s: &mut Cursive| { s.quit(); })
        .h_align(cursive::align::HAlign::Center)
        .with_name("quit-popup");

    siv.call_on_name("views-stack", |sv: &mut StackView| { sv.add_layer(dialog_with_txt) });
}

pub(super) fn draw_popup(siv: &mut Cursive) {
    let ud: &mut SivUserData = siv.user_data().unwrap();
    if ud.config.project.is_some() {
        popup_quit_and_save(siv);
    }
    else {
        popup_quit(siv);
    }
}

fn hide_popup(siv: &mut Cursive) {
    siv.call_on_name("views-stack", |sv: &mut StackView| { sv.pop_layer() });
}