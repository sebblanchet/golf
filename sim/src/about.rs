use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui::special_emojis::GITHUB;
use bevy_egui::{egui, EguiContext};

// example
// https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/about.rs

pub fn update(mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>) {
    let Ok(mut ctx) = egui_ctx.get_single_mut() else {
        return;
    };

    egui::Window::new("About")
        .default_height(150.)
        .show(ctx.get_mut(), |ui| {
            ui.heading("Version");
            ui.hyperlink_to(
                format!("{GITHUB} github.com/sebblanchet/golf"),
                "https://github.com/sebblanchet/golf",
            );
            ui.label("v0.1.0");
        });
}
