use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContext};

use crate::state;

pub fn inputs(
    mut state: ResMut<state::Inputs>,
    mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
) {
    let Ok(mut ctx) = egui_ctx.get_single_mut() else {
        return;
    };
    egui::SidePanel::right("inputs")
        .min_width(100.0)
        .resizable(false)
        .show(ctx.get_mut(), |ui| {
            egui::ComboBox::from_label("club")
                .selected_text("test".to_string())
                .show_ui(ui, |ui| {
                    let keys = state.bag.list();
                    for k in keys {
                        let c = state.bag.get(k.clone());
                        ui.selectable_value(&mut state.club, c, k);
                    }
                });
            ui.end_row();

            ui.label("mass [kg]");
            ui.add(egui::Slider::new(&mut state.m, 0. ..=1.));
            ui.end_row();

            ui.label("radius [m]");
            ui.add(egui::Slider::new(&mut state.r, 0. ..=1.));
            ui.end_row();

            let x = 10.0;
            ui.label("x [m]");
            ui.add(egui::Slider::new(&mut state.position.x, -x..=x));
            ui.end_row();
            ui.label("y [m]");
            ui.add(egui::Slider::new(&mut state.position.y, -x..=x));
            ui.end_row();
            ui.label("z [m]");
            ui.add(egui::Slider::new(&mut state.position.z, -x..=x));
            ui.end_row();

            let v = 100.0;
            ui.label("vx [m]");
            ui.add(egui::Slider::new(&mut state.velocity.x, -v..=v));
            ui.end_row();
            ui.label("vy [m]");
            ui.add(egui::Slider::new(&mut state.velocity.y, 0. ..=v));
            ui.end_row();
            ui.label("vz [m]");
            ui.add(egui::Slider::new(&mut state.velocity.z, -v..=v));
            ui.end_row();

            let w = 1000.0;
            ui.label("wx [m]");
            ui.add(egui::Slider::new(&mut state.angular.x, -w..=w));
            ui.end_row();
            ui.label("wy [m]");
            ui.add(egui::Slider::new(&mut state.angular.y, -w..=w));
            ui.end_row();
            ui.label("wz [m]");
            ui.add(egui::Slider::new(&mut state.angular.z, -w..=w));
            ui.end_row();
        });
}

pub fn outputs(
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
                ui.label("time");
                let text = ball.time.last().copied().unwrap_or_default().to_string();
                ui.label(text);
            });
            ui.horizontal(|ui| {
                ui.label("position");
                let text = ball
                    .position
                    .last()
                    .copied()
                    .unwrap_or_default()
                    .to_string();
                ui.label(text);
            });
            ui.horizontal(|ui| {
                ui.label("velocity");
                let text = ball
                    .velocity
                    .last()
                    .copied()
                    .unwrap_or_default()
                    .to_string();

                ui.label(text);
            });
            ui.horizontal(|ui| {
                ui.label("angular");
                let text = ball.angular.last().copied().unwrap_or_default().to_string();
                ui.label(text);
            });
        });
}
