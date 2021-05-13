use eframe::{
    egui::{self, Color32, Grid},
    epi,
};

use crate::{cpu::EaterCpu, widgets};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct EaterEmuApp {
    cpu: EaterCpu,
}

impl Default for EaterEmuApp {
    fn default() -> Self {
        Self {
            cpu: Default::default(),
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
        let EaterEmuApp { cpu } = self;

        ctx.set_pixels_per_point(1.5);

        // let mut style: egui::Style = (*ctx.style()).clone();
        // style.spacing.item_spacing = egui::vec2(5.0, 20.0);
        // ctx.set_style(style);

        // egui examples: https://emilk.github.io/egui

        egui::CentralPanel::default().show(ctx, |ui| {
            Grid::new("panel_grid").show(ui, |ui| {
                ui.heading("Output");
                ui.add(widgets::register(cpu.out, 8, Color32::RED));
                ui.end_row();

                ui.heading("Reg A");
                ui.add(widgets::register(cpu.a, 8, Color32::RED));
                ui.end_row();

                ui.heading("Reg B");
                ui.add(widgets::register(cpu.b, 8, Color32::RED));
                ui.end_row();

                ui.heading("IR");
                ui.add(widgets::register(cpu.ir, 4, Color32::BLUE));
                ui.end_row();

                ui.heading("MAR");
                ui.add(widgets::register(cpu.mar, 4, Color32::YELLOW));
                ui.end_row();

                ui.heading("PC");
                ui.add(widgets::register(cpu.pc, 4, Color32::RED));
                ui.end_row();

                ui.heading("T");
                ui.add(widgets::register(cpu.t, 4, Color32::GREEN));
                ui.end_row();

                ui.heading("Input");
                ui.add(widgets::register_input(&mut cpu.a));
                ui.end_row();

                ui.separator();
                ui.separator();
                ui.end_row();

                ui.heading("Clock");
                ui.add(widgets::toggle_switch(&mut cpu.clk));
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
