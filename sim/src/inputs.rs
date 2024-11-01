use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContext};

use crate::bag;
use crate::state;

pub fn update(
    mut bag: ResMut<bag::Bag>,
    mut state: ResMut<state::Inputs>,
    mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
) {
    let Ok(mut ctx) = egui_ctx.get_single_mut() else {
        return;
    };
    egui::SidePanel::right("inputs")
        .min_width(200.0)
        .resizable(false)
        .show(ctx.get_mut(), |ui| {
            egui::CollapsingHeader::new("params")
                .default_open(true)
                .show(ui, |ui| {
                    ui.label("club");
                    egui::ComboBox::from_label("")
                        .selected_text(state.club.clone())
                        .show_ui(ui, |ui| {
                            let keys = bag.list();
                            for k in keys {
                                ui.selectable_value(&mut state.club, k.clone(), k);
                            }
                            // TODO update club
                        });

                    ui.label("mass [kg]");
                    ui.add(egui::Slider::new(&mut state.m, 0. ..=1.));
                    ui.end_row();

                    ui.label("radius [m]");
                    ui.add(egui::Slider::new(&mut state.r, 0. ..=1.));
                    ui.end_row();
                });
            egui::CollapsingHeader::new("position")
                .default_open(false)
                .show(ui, |ui| {
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
                });

            egui::CollapsingHeader::new("velocity")
                .default_open(false)
                .show(ui, |ui| {
                    let v = 100.0;
                    ui.label("x [m]");
                    ui.add(egui::Slider::new(&mut state.velocity.x, -v..=v));
                    ui.end_row();
                    ui.label("y [m]");
                    ui.add(egui::Slider::new(&mut state.velocity.y, 0. ..=v));
                    ui.end_row();
                    ui.label("z [m]");
                    ui.add(egui::Slider::new(&mut state.velocity.z, -v..=v));
                    ui.end_row();
                });

            egui::CollapsingHeader::new("spin")
                .default_open(false)
                .show(ui, |ui| {
                    let w = 1000.0;
                    ui.label("x [m]");
                    ui.add(egui::Slider::new(&mut state.spin.x, -w..=w));
                    ui.end_row();
                    ui.label("y [m]");
                    ui.add(egui::Slider::new(&mut state.spin.y, -w..=w));
                    ui.end_row();
                    ui.label("z [m]");
                    ui.add(egui::Slider::new(&mut state.spin.z, -w..=w));
                    ui.end_row();
                });
        });
}
