use eframe::{
    egui::{self, Color32},
    epi,
};

use crate::widgets;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct EaterEmuApp {
    label: String,
    bit0: bool,
    bit1: bool,
}

impl Default for EaterEmuApp {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            bit0: false,
            bit1: false,
        }
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
        let EaterEmuApp { label, bit0, bit1 } = self;

        // egui examples: https://emilk.github.io/egui

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(label.as_str());

            ui.horizontal(|ui| {
                ui.add(widgets::led(bit0, Color32::RED, "0"));
                ui.add(widgets::led(bit1, Color32::RED, "1"));
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
