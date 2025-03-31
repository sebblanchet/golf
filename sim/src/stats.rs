use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContext};

use crate::state;

pub fn update(
    inputs: ResMut<state::Inputs>,
    outputs: ResMut<state::Outputs>,
    mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
) {
    let Ok(mut ctx) = egui_ctx.get_single_mut() else {
        return;
    };

    let Some(ball) = &outputs.ball else {
        return;
    };

    egui::Window::new("Stats")
        .default_height(150.)
        .show(ctx.get_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Flight Time:");
                let tstart = ball.time.first().copied().unwrap_or_default();
                let tend = ball.time.last().copied().unwrap_or_default();
                let text = (tend - tstart).to_string();
                ui.label(text);
            });

            ui.horizontal(|ui| {
                ui.label("Carry:");
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
                        max = pos.y;
                    }
                }
                ui.label("Apex:");
                ui.label(max.to_string());
            });

            ui.horizontal(|ui| {
                let spin = ball.spin[0].length();
                ui.label("Spin:");
                ui.label(spin.to_string());
            });

            ui.horizontal(|ui| {
                let smash = ball.velocity[0].length() / inputs.club.speed;
                ui.label("Smash Factor:");
                ui.label(smash.to_string());
            });

            // TODO add shot guess/push pull hook
        });
}
