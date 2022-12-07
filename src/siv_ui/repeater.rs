use cursive::{Cursive, views::{ListView, TextView, LinearLayout, Checkbox, EditView, OnEventView, Dialog}, event, view::{Resizable, Nameable}};
use super::sivuserdata::SivUserData;
use super::views_stack;



pub(super) struct RepeaterParameters {
    redirects: bool,
    https: bool,
    address: String,
}

pub(super) struct RepeaterState {
    name: String,
    request: String,
    response: Option<String>,
    parameters: RepeaterParameters
}

impl RepeaterState {
    pub(super) fn test_new(name: String, request: String) -> Self {
        RepeaterState {
            name,
            request,
            response: None,
            parameters: RepeaterParameters {
                redirects: true,
                https: true,
                address: "test".to_string()
            }
        }
    }
}

pub(super) fn draw_repeater(siv: &mut Cursive) {
    let ud: &SivUserData = siv.user_data().unwrap();

    let mut initial_list = ListView::new();
    for (index, instance) in ud.repeater_state.iter().enumerate() {
        let name = EditView::new().content(instance.name.clone());

        let redirect_cb = if instance.parameters.redirects {
            Checkbox::new().checked()
        }
        else {
            Checkbox::new()
        };

        let https_cb = if instance.parameters.https {
            Checkbox::new().checked()
        }
        else {
            Checkbox::new()
        };

        let addr_txt = EditView::new().content(instance.parameters.address.clone());

        let parameters_list = ListView::new()
            .delimiter()
            .child("Name:", name)
            .child("Follow redirects:", redirect_cb)
            .child("Use HTTPS:", https_cb)
            .child("Host:", addr_txt);

        let params_dialog = Dialog::around(parameters_list)
            .h_align(cursive::align::HAlign::Center)
            .button("Select", |s: &mut Cursive| {});

        initial_list.add_child(&index.to_string(), params_dialog);
        // initial_list.add_delimiter();
    }

    let name_list = initial_list.with_name("repeaters");
    let with_event = OnEventView::new(name_list)
        .on_event(event::Key::Esc, |s: &mut Cursive| { views_stack::pop_layer(s); });

    let dialog = Dialog::around(with_event)
        .title("Select repeater")
        .full_height()
        .min_width(40);

    views_stack::push_layer(siv, dialog);
}

fn get_list_selected(siv: &mut Cursive) -> Option<usize> {
    siv.call_on_name("repeaters", |l: &mut ListView| {
        if ! l.is_empty() {
            Some(l.focus())
        }
        else {
            None
        }
    }).unwrap()
}

fn select_with_button(siv: &mut Cursive) {
    todo!();
}
