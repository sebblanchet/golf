use bevy::math::NormedVectorSpace;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContext};

use crate::constants::*;
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
                ui.label("Flight Time [s]:");
                let tstart = ball.time.first().copied().unwrap_or_default();
                let tend = ball.time.last().copied().unwrap_or_default();
                let text = format!("{:.2}", tend - tstart);
                ui.label(text);
            });

            ui.horizontal(|ui| {
                let v0 = ball.velocity[0].length();
                match inputs.units {
                    state::Units::Metric => {
                        ui.label("Ball Speed [m/s]:");
                        ui.label(format!("{:.2}", v0));
                    }
                    state::Units::Imperial => {
                        ui.label("Ball Speed [mph]:");
                        ui.label(format!("{:.2}", v0 * MS_TO_MPH));
                    }
                }
            });

            ui.horizontal(|ui| {
                let carry_m = ball.position.last().copied().unwrap_or_default().x;
                match inputs.units {
                    state::Units::Metric => {
                        ui.label("Carry [m]:");
                        ui.label(format!("{:.2}", carry_m));
                    }
                    state::Units::Imperial => {
                        ui.label("Carry [yd]:");
                        ui.label(format!("{:.2}", carry_m * M_TO_YD));
                    }
                }
            });

            ui.horizontal(|ui| {
                let mut max = f32::MIN;
                for pos in &ball.position {
                    if pos.y > max {
                        max = pos.y;
                    }
                }
                match inputs.units {
                    state::Units::Metric => {
                        ui.label("Apex [m]:");
                        ui.label(format!("{:.2}", max));
                    }
                    state::Units::Imperial => {
                        ui.label("Apex [yd]:");
                        ui.label(format!("{:.2}", max * M_TO_YD));
                    }
                }
            });

            ui.horizontal(|ui| {
                let spin_rs = ball.spin[0].length();
                match inputs.units {
                    state::Units::Metric => {
                        ui.label("Spin [rad/s]:");
                        ui.label(format!("{:.2}", spin_rs));
                    }
                    state::Units::Imperial => {
                        ui.label("Spin [RPM]:");
                        ui.label(format!("{:.0}", spin_rs * RADS_TO_RPM));
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label("Smash Factor [-]:");
                let v0 = ball.inputs.club.speed;
                let v1 = ball.velocity.get(0).copied().unwrap_or_default();
                let sf = v1.norm() / v0;
                let text = format!("{:.2}", sf);
                ui.label(text);
            });

            // TODO add shot guess/push pull hook

            ui.separator();
            if ui.button("Export CSV").clicked() {
                if let Some(ball) = &outputs.ball {
                    ball.save_combined_csv();
                    info!("exported CSV");
                } else {
                    warn!("no ball to export");
                }
            }
        });
}
