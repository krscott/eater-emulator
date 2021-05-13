use eframe::egui::{self, Color32, Response, Ui, Widget};

use crate::widgets::{led, toggle_switch};

fn register_ui(ui: &mut Ui, value: u8, color: Color32) -> Response {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(4.0, 20.0);

        for i in (0..8).rev() {
            let mask = 1u8 << i;
            let bit = value & mask == mask;

            ui.add(led(bit, color, i.to_string()));
        }

        ui.separator();
        ui.monospace(format!("${:02X}", value));
        ui.separator();
        ui.monospace(format!("{}", value));
    })
    .response
}

pub fn register(value: u8, color: Color32) -> impl Widget {
    move |ui: &mut Ui| register_ui(ui, value, color)
}

fn register_input_ui(ui: &mut Ui, value: &mut u8) -> Response {
    let mut mark_changed = false;

    let mut response = ui
        .horizontal(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(4.0, 20.0);

            for i in (0..8).rev() {
                let mask = 1u8 << i;
                let mut bit = *value & mask == mask;

                let resp = ui.add(toggle_switch(&mut bit));

                if resp.changed() {
                    *value ^= mask;
                    mark_changed = true;
                }
            }

            // ui.separator();
            // ui.monospace(format!("${:02X}", value));
            // ui.separator();
            // ui.monospace(format!("{}", value));
        })
        .response;

    if mark_changed {
        response.mark_changed();
    }

    response
}

pub fn register_input(value: &mut u8) -> impl Widget + '_ {
    move |ui: &mut Ui| register_input_ui(ui, value)
}
