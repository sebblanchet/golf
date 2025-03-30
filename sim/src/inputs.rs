use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContext};

use crate::bag;
use crate::state;
//use crate::shot;

pub fn update(
    bag: ResMut<bag::Bag>,
    mut state: ResMut<state::Inputs>,
    mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
) {
    let Ok(mut ctx) = egui_ctx.get_single_mut() else {
        return;
    };
    egui::SidePanel::right("Inputs")
        .min_width(200.0)
        .resizable(false)
        .show(ctx.get_mut(), |ui| {
            egui::CollapsingHeader::new("Club")
                .default_open(true)
                .show(ui, |ui| {
                    egui::ComboBox::from_id_salt("Club")
                        .selected_text(state.club.name.clone())
                        .show_ui(ui, |ui| {
                            for club in &bag.clubs {
                                ui.selectable_value(
                                    &mut state.club,
                                    club.clone(),
                                    club.clone().name,
                                )
                                .changed()
                                .then(|| {
                                    state.update();
                                });
                            }
                        });

                    //ui.label("hand");
                    //egui::ComboBox::from_id_salt("hand")
                    //    .selected_text(state.hand.to_string())
                    //    .show_ui(ui, |ui| {
                    //        let hands = vec![shot::Hand::Left, shot::Hand::Right];
                    //        for hand in hands {
                    //            ui.selectable_value(&mut state.hand, hand, hand.to_string())
                    //                .changed()
                    //                .then(|| {
                    //                    state.update();
                    //                });
                    //        }
                    //    });

                    //ui.label("shot");
                    //egui::ComboBox::from_id_salt("shot")
                    //    .selected_text(state.shot.to_string())
                    //    .show_ui(ui, |ui| {
                    //        let shots = vec![
                    //            shot::Shot::Push,
                    //            shot::Shot::Slice,
                    //            shot::Shot::Fade,
                    //            shot::Shot::Straight,
                    //            shot::Shot::Draw,
                    //            shot::Shot::Hook,
                    //            shot::Shot::Pull,
                    //        ];
                    //        for shot in shots {
                    //            ui.selectable_value(&mut state.shot, shot, shot.to_string())
                    //                .changed()
                    //                .then(|| {
                    //                    state.update();
                    //                });
                    //        }
                    //    });

                    ui.label("Speed [m/s]");
                    ui.add(egui::Slider::new(&mut state.club.speed, 0. ..=100.))
                        .changed()
                        .then(|| {
                            state.update();
                        });

                    ui.end_row();

                    ui.label("Loft [deg]");
                    ui.add(egui::Slider::new(&mut state.club.loft, 5. ..=60.))
                        .changed()
                        .then(|| {
                            state.update();
                        });

                    ui.end_row();

                    //ui.label("weight [kg]");
                    //ui.add(egui::Slider::new(&mut state.club.weight, 0. ..=1.));
                    //ui.end_row();
                });

            egui::CollapsingHeader::new("Params")
                .default_open(false)
                .show(ui, |ui| {
                    ui.label("Ball Mass [kg]");
                    ui.add(egui::Slider::new(&mut state.m, 0. ..=1.));
                    ui.end_row();

                    ui.label("Ball Radius [m]");
                    ui.add(egui::Slider::new(&mut state.r, 0. ..=1.));
                    ui.end_row();

                    ui.label("Air Density [kg/m^3]");
                    ui.add(egui::Slider::new(&mut state.rho, 0. ..=5.));
                    ui.end_row();

                    ui.label("Air Viscosity [m^2/s]");
                    ui.add(egui::Slider::new(&mut state.mu, 1e-5..=2e-5));
                    ui.end_row();

                    ui.label("Decel Spin [%]");
                    ui.add(egui::Slider::new(&mut state.decel, 0. ..=20.));
                    ui.end_row();
                });

            egui::CollapsingHeader::new("Position")
                .default_open(false)
                .show(ui, |ui| {
                    let pos = 50.0;
                    ui.label("x [m]");
                    ui.add(egui::Slider::new(&mut state.position.x, -pos..=pos));
                    ui.end_row();
                    ui.label("y [m]");
                    ui.add(egui::Slider::new(&mut state.position.y, -0. ..=pos));
                    ui.end_row();
                    ui.label("z [m]");
                    ui.add(egui::Slider::new(&mut state.position.z, -pos..=pos));
                    ui.end_row();
                });

            egui::CollapsingHeader::new("Velocity")
                .default_open(false)
                .show(ui, |ui| {
                    let v = 100.0;
                    ui.label("x [m/s]");
                    ui.add(egui::Slider::new(&mut state.velocity.x, -v..=v));
                    ui.end_row();
                    ui.label("y [m/s]");
                    ui.add(egui::Slider::new(&mut state.velocity.y, 0. ..=v));
                    ui.end_row();
                    ui.label("z [m/s]");
                    ui.add(egui::Slider::new(&mut state.velocity.z, -v..=v));
                    ui.end_row();
                });

            egui::CollapsingHeader::new("Spin")
                .default_open(false)
                .show(ui, |ui| {
                    let w = 1000.0;
                    ui.label("x [rad/s]");
                    ui.add(egui::Slider::new(&mut state.spin.x, -w..=w));
                    ui.end_row();
                    ui.label("y [rad/s]");
                    ui.add(egui::Slider::new(&mut state.spin.y, -w..=w));
                    ui.end_row();
                    ui.label("z [rad/s]");
                    ui.add(egui::Slider::new(&mut state.spin.z, -w..=w));
                    ui.end_row();
                });
        });

    //dbg!(state);
}
