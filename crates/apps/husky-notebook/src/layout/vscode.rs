use egui::{Layout, Vec2};

use super::*;

impl HuskyNotebookApp {
    pub(super) fn render_vscode_high_level_layout(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
    ) {
        ui.vertical(|ui| {
            self.render_main_menu(ctx, ui);
            ui.separator(0.);
            ui.vertical_reversed(|ui| {
                self.render_status_bar(ctx, ui);
                ui.separator(0.);
                ui.centered_and_justified(|ui| self.render_middle_ground(ctx, ui))
            })
        });
    }

    fn render_middle_ground(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            self.render_activity_bar(ctx, ui);
            ui.separator(0.);
            self.render_activity_view(ctx, ui);
            ui.separator(0.);
            ui.centered_and_justified(|ui| self.render_main_view(ctx, ui))
        });
    }

    fn render_main_view(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.label("todo: main view");
    }
}
