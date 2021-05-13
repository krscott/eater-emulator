use eframe::{
    egui::{self, Color32, Grid},
    epi,
};

use crate::widgets;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct EaterEmuApp {
    bus: u8,
}

impl Default for EaterEmuApp {
    fn default() -> Self {
        Self { bus: 0x55 }
    }
}

impl epi::App for EaterEmuApp {
    fn name(&self) -> &str {
        "Eater-like Computer Emulator"
    }

    /// Called by the framework to load old app state (if any).
    #[cfg(feature = "persistence")]
    fn load(&mut self, storage: &dyn epi::Storage) {
        *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
    }

    /// Called by the framework to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        let EaterEmuApp { bus } = self;

        ctx.set_pixels_per_point(1.5);

        // let mut style: egui::Style = (*ctx.style()).clone();
        // style.spacing.item_spacing = egui::vec2(5.0, 20.0);
        // ctx.set_style(style);

        // egui examples: https://emilk.github.io/egui

        egui::CentralPanel::default().show(ctx, |ui| {
            Grid::new("panel_grid").show(ui, |ui| {
                ui.heading("Bus");
                ui.add(widgets::register(*bus, Color32::RED));
                ui.end_row();

                ui.separator();
                ui.add(widgets::register_input(bus));
                ui.end_row();
            });

            ui.separator();

            ui.heading("eater-emulator");
            ui.hyperlink("https://github.com/krscott/eater-emulator");
            ui.add(egui::github_link_file_line!(
                "https://github.com/krscott/eater-emulator/blob/master/",
                "Direct link to source code."
            ));
            egui::warn_if_debug_build(ui);
        });
    }
}
