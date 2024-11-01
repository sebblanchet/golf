use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContext};

use crate::state;

pub fn update(
    outputs: ResMut<state::Ouputs>,
    mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
) {
    let Ok(mut ctx) = egui_ctx.get_single_mut() else {
        return;
    };

    let Some(ball) = &outputs.ball else {
        return;
    };

    egui::Window::new("results")
        .vscroll(true)
        .default_height(100.)
        .show(ctx.get_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("flight time");
                let tstart = ball.time.first().copied().unwrap_or_default();
                let tend = ball.time.last().copied().unwrap_or_default();
                let text = (tend - tstart).to_string();
                ui.label(text);
            });
            ui.horizontal(|ui| {
                ui.label("carry");
                let text = ball
                    .position
                    .last()
                    .copied()
                    .unwrap_or_default()
                    .x
                    .to_string();
                ui.label(text);
            });
            ui.horizontal(|ui| {
                let mut max = f32::MIN;
                for pos in &ball.position {
                    if pos.y > max {
                        dbg!(pos.y);
                        max = pos.y;
                    }
                }
                ui.label("apex");
                ui.label(max.to_string());
            });
        });
}
