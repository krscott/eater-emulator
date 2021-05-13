use eframe::egui::{Color32, Label, Response, Ui, Widget};

use crate::widgets::led;

fn register_ui(ui: &mut Ui, value: &mut u8, color: Color32, label: String) -> Response {
    // let desired_size = vec2(ui.available_width(), ui.spacing().interact_size.y);
    // let response = ui.allocate_response(desired_size, Sense::hover());

    let mut changed = false;

    let mut response = ui
        .horizontal(|ui| {
            ui.add(Label::new(label).strong());

            for i in (0..8).rev() {
                let mask = 1u8 << i;
                let mut bit = *value & mask == mask;

                let resp = ui.add(led(&mut bit, color, i.to_string()));

                if resp.clicked() {
                    *value ^= mask;
                    changed = true;
                }
            }

            let mut hex_string = format!("{:02X}", value);
            let resp = ui.text_edit_singleline(&mut hex_string);

            if resp.changed() {
                if let Ok(new_value) = u8::from_str_radix(&hex_string, 16) {
                    *value = new_value;
                    changed = true;
                }
            }
        })
        .response;

    if changed {
        response.mark_changed();
    }

    response
}

pub fn register<S: Into<String>>(value: &mut u8, color: Color32, label: S) -> impl Widget + '_ {
    let label: String = label.into();
    move |ui: &mut Ui| register_ui(ui, value, color, label)
}
