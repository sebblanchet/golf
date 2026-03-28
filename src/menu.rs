use crate::state;
use crate::ui;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContext};

pub fn update(
    mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut state_inputs: ResMut<state::Inputs>,
    mut _state_outputs: ResMut<state::Outputs>,
    mut ui_inputs: ResMut<ui::Ui>,
) {
    let Ok(mut ctx) = egui_ctx.get_single_mut() else {
        return;
    };

    egui::TopBottomPanel::top("top_panel").show(ctx.get_mut(), |ui| {
        ui.horizontal(|ui| {
            if ui.button("Reset").clicked() {
                *state_inputs = state::Inputs::default();
            }
            if ui.button("Restart").clicked() {
                ui_inputs.restart = true;
            }
            if ui.button("Inputs").clicked() {
                ui_inputs.open_inputs = !ui_inputs.open_inputs;
            }
            if ui.button("About").clicked() {
                ui_inputs.open_about = !ui_inputs.open_about;
            }
            if ui.button("Plots").clicked() {
                ui_inputs.open_plots = !ui_inputs.open_plots;
            }
            if ui.button("Stats").clicked() {
                ui_inputs.open_stats = !ui_inputs.open_stats;
            }
        });
    });
}
