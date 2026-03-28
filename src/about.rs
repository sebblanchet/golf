use crate::constants::UI_LABEL_SPACE;
use crate::ui;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui::special_emojis::GITHUB;
use bevy_egui::{egui, EguiContext};

pub fn update(
    mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut ui_inputs: ResMut<ui::Ui>,
) {
    let Ok(mut ctx) = egui_ctx.get_single_mut() else {
        return;
    };

    egui::Window::new("About")
        .default_height(150.)
        .open(&mut ui_inputs.open_about)
        .show(ctx.get_mut(), |ui| {
            ui.hyperlink_to(
                format!("{GITHUB} Source Code"),
                "https://github.com/sebblanchet/golf",
            );
            ui.add_space(UI_LABEL_SPACE);
            ui.hyperlink_to(
                "\u{2139} Blog Post",
                "https://sebblanchet.com/blog/2026/03/26/golf-sim/",
            );
            ui.add_space(UI_LABEL_SPACE);
            ui.hyperlink_to("📓 Made with Rust + Bevy", "https://bevy.org/");
            ui.add_space(UI_LABEL_SPACE);
            ui.label("Zoom:     Left Shift+Mouse");
            ui.label("Pan:      Left Ctrl+Mouse");
            ui.label("Orbit:    Left Alt+Mouse");
            ui.label("Restart:  Space bar");
            ui.add_space(UI_LABEL_SPACE);

            ui.label("Seb Blanchet - © 2026 - v0.2.0");
        });
}
