use eframe::egui::{self, Color32, Response, Ui, Widget};

use crate::widgets::led;

fn register_ui(ui: &mut Ui, value: u8, color: Color32) -> Response {
    // let desired_size = vec2(ui.available_width(), ui.spacing().interact_size.y);
    // let response = ui.allocate_response(desired_size, Sense::hover());

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
