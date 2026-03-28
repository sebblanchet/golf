use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContext};

use crate::bag;
use crate::constants::*;
use crate::state;
use crate::ui;

pub fn update(
    bag: ResMut<bag::Bag>,
    mut state: ResMut<state::Inputs>,
    ui_inputs: ResMut<ui::Ui>,
    mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
) {
    let Ok(mut ctx) = egui_ctx.get_single_mut() else {
        return;
    };

    egui::SidePanel::right("Inputs")
        .min_width(200.0)
        .resizable(false)
        .show_animated(ctx.get_mut(), ui_inputs.open_inputs, |ui| {
            // Units toggle
            egui::CollapsingHeader::new("Units")
                .default_open(true)
                .show(ui, |ui| {
                    egui::ComboBox::from_id_salt("Units")
                        .selected_text(match state.units {
                            state::Units::Metric => "Metric (m, m/s, rad/s)",
                            state::Units::Imperial => "Imperial (yd, mph, RPM)",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut state.units,
                                state::Units::Metric,
                                "Metric (m, m/s)",
                            );
                            ui.selectable_value(
                                &mut state.units,
                                state::Units::Imperial,
                                "Imperial (yd, mph, RPM)",
                            );
                        });
                });

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
                    // Speed slider (unit-aware: m/s or mph)
                    match state.units {
                        state::Units::Metric => {
                            ui.label("Clubhead Speed [m/s]");
                            ui.add(egui::Slider::new(&mut state.club.speed, 0. ..=100.))
                                .changed()
                                .then(|| {
                                    state.update();
                                });
                        }
                        state::Units::Imperial => {
                            ui.label("Clubhead Speed [mph]");
                            let mut speed_mph = state.club.speed * MS_TO_MPH;
                            if ui
                                .add(egui::Slider::new(&mut speed_mph, 0. ..=(100. * MS_TO_MPH)))
                                .changed()
                            {
                                state.club.speed = speed_mph / MS_TO_MPH;
                                state.update();
                            }
                        }
                    }
                    ui.end_row();

                    ui.label("Smash Factor [-]");
                    ui.add(egui::Slider::new(&mut state.club.smash, 0.5..=2.))
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
                });

            egui::CollapsingHeader::new("Params")
                .default_open(false)
                .show(ui, |ui| {
                    ui.label("Ball Mass [kg]");
                    ui.add(egui::Slider::new(&mut state.m, 0.01..=0.5));
                    ui.end_row();

                    ui.label("Ball Radius [m]");
                    ui.add(egui::Slider::new(&mut state.r, 0.05..=0.5));
                    ui.end_row();

                    ui.label("Air Density [kg/m^3]");
                    ui.add(egui::Slider::new(&mut state.rho, 0. ..=2.));
                    ui.end_row();

                    ui.label("Air Viscosity [m^2/s]");
                    ui.add(egui::Slider::new(&mut state.mu, 1e-5..=2e-5));
                    ui.end_row();

                    ui.label("Decel Spin [%]");
                    ui.add(egui::Slider::new(&mut state.decel, 0. ..=20.));
                    ui.end_row();

                    ui.label("Drag Coefficient [-]");
                    ui.add(egui::Slider::new(&mut state.c_d, 0. ..=2.));
                    ui.end_row();

                    ui.label("Magnus Coefficient [-]");
                    ui.add(egui::Slider::new(&mut state.c_m, 0. ..=2.));
                    ui.end_row();
                });

            egui::CollapsingHeader::new("Position")
                .default_open(false)
                .show(ui, |ui| {
                    let pos_m = 50.0;
                    match state.units {
                        state::Units::Metric => {
                            ui.label("x [m]");
                            ui.add(egui::Slider::new(&mut state.position.x, -pos_m..=pos_m));
                            ui.end_row();
                            ui.label("y [m]");
                            ui.add(egui::Slider::new(&mut state.position.y, -0. ..=pos_m));
                            ui.end_row();
                            ui.label("z [m]");
                            ui.add(egui::Slider::new(&mut state.position.z, -pos_m..=pos_m));
                            ui.end_row();
                        }
                        state::Units::Imperial => {
                            let mut x_yd = state.position.x * M_TO_YD;
                            let mut y_yd = state.position.y * M_TO_YD;
                            let mut z_yd = state.position.z * M_TO_YD;
                            let range_yd = pos_m * M_TO_YD;
                            ui.label("x [yd]");
                            if ui
                                .add(egui::Slider::new(&mut x_yd, -range_yd..=range_yd))
                                .changed()
                            {
                                state.position.x = x_yd / M_TO_YD;
                            }
                            ui.end_row();
                            ui.label("y [yd]");
                            if ui
                                .add(egui::Slider::new(&mut y_yd, -0. ..=range_yd))
                                .changed()
                            {
                                state.position.y = y_yd / M_TO_YD;
                            }
                            ui.end_row();
                            ui.label("z [yd]");
                            if ui
                                .add(egui::Slider::new(&mut z_yd, -range_yd..=range_yd))
                                .changed()
                            {
                                state.position.z = z_yd / M_TO_YD;
                            }
                            ui.end_row();
                        }
                    }
                });

            egui::CollapsingHeader::new("Velocity")
                .default_open(false)
                .show(ui, |ui| {
                    let v_ms = 100.0;
                    match state.units {
                        state::Units::Metric => {
                            ui.label("x [m/s]");
                            ui.add(egui::Slider::new(&mut state.velocity.x, -v_ms..=v_ms));
                            ui.end_row();
                            ui.label("y [m/s]");
                            ui.add(egui::Slider::new(&mut state.velocity.y, 0. ..=v_ms));
                            ui.end_row();
                            ui.label("z [m/s]");
                            ui.add(egui::Slider::new(&mut state.velocity.z, -v_ms..=v_ms));
                            ui.end_row();
                        }
                        state::Units::Imperial => {
                            let mut vx_mph = state.velocity.x * MS_TO_MPH;
                            let mut vy_mph = state.velocity.y * MS_TO_MPH;
                            let mut vz_mph = state.velocity.z * MS_TO_MPH;
                            let range_mph = v_ms * MS_TO_MPH;
                            ui.label("x [mph]");
                            if ui
                                .add(egui::Slider::new(&mut vx_mph, -range_mph..=range_mph))
                                .changed()
                            {
                                state.velocity.x = vx_mph / MS_TO_MPH;
                            }
                            ui.end_row();
                            ui.label("y [mph]");
                            if ui
                                .add(egui::Slider::new(&mut vy_mph, 0. ..=range_mph))
                                .changed()
                            {
                                state.velocity.y = vy_mph / MS_TO_MPH;
                            }
                            ui.end_row();
                            ui.label("z [mph]");
                            if ui
                                .add(egui::Slider::new(&mut vz_mph, -range_mph..=range_mph))
                                .changed()
                            {
                                state.velocity.z = vz_mph / MS_TO_MPH;
                            }
                            ui.end_row();
                        }
                    }
                });

            egui::CollapsingHeader::new("Spin")
                .default_open(false)
                .show(ui, |ui| {
                    let w = 1000.0; // rad/s range for Metric
                    match state.units {
                        state::Units::Metric => {
                            ui.label("x [rad/s]");
                            ui.add(egui::Slider::new(&mut state.spin.x, -w..=w));
                            ui.end_row();
                            ui.label("y [rad/s]");
                            ui.add(egui::Slider::new(&mut state.spin.y, -w..=w));
                            ui.end_row();
                            ui.label("z [rad/s]");
                            ui.add(egui::Slider::new(&mut state.spin.z, -w..=w));
                            ui.end_row();
                        }
                        state::Units::Imperial => {
                            let w_rpm = w * RADS_TO_RPM; // convert range to RPM
                            let mut sx_rpm = state.spin.x * RADS_TO_RPM;
                            let mut sy_rpm = state.spin.y * RADS_TO_RPM;
                            let mut sz_rpm = state.spin.z * RADS_TO_RPM;

                            ui.label("x [RPM]");
                            if ui
                                .add(egui::Slider::new(&mut sx_rpm, -w_rpm..=w_rpm))
                                .changed()
                            {
                                state.spin.x = sx_rpm / RADS_TO_RPM;
                            }
                            ui.end_row();
                            ui.label("y [RPM]");
                            if ui
                                .add(egui::Slider::new(&mut sy_rpm, -w_rpm..=w_rpm))
                                .changed()
                            {
                                state.spin.y = sy_rpm / RADS_TO_RPM;
                            }
                            ui.end_row();
                            ui.label("z [RPM]");
                            if ui
                                .add(egui::Slider::new(&mut sz_rpm, -w_rpm..=w_rpm))
                                .changed()
                            {
                                state.spin.z = sz_rpm / RADS_TO_RPM;
                            }
                            ui.end_row();
                        }
                    }
                });
        });
}
