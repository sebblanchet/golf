use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui;
use bevy_egui::egui::Pos2;
use bevy_egui::EguiContext;
use egui_plot::{Legend, Line, PlotPoints, PlotUi};

use crate::constants::*;
use crate::state;

pub fn update(
    inputs: Res<state::Inputs>,
    outputs: ResMut<state::Outputs>,
    mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
) {
    let Ok(mut ctx) = egui_ctx.get_single_mut() else {
        return;
    };

    let Some(ball) = &outputs.ball else {
        return;
    };

    let mut p = Plot::new();

    // unit-aware conversion for position
    let to_yd = matches!(inputs.units, state::Units::Imperial);
    let (label_unit, k) = if to_yd {
        ("[yd]", M_TO_YD)
    } else {
        ("[m]", 1.0)
    };

    // Prepare shared x, and y/z series; draw both on one plot
    let x: Vec<f32> = ball.clone().position.into_iter().map(|p| p.x * k).collect();
    let y: Vec<f32> = ball.clone().position.into_iter().map(|p| p.y * k).collect();
    let z: Vec<f32> = ball.clone().position.into_iter().map(|p| p.z * k).collect();
    p.update_multi(
        ctx.get_mut(),
        format!("position vs x {}", label_unit),
        x,
        vec![
            (format!("x/y {}", label_unit), y),
            (format!("x/z {}", label_unit), z),
        ],
    );
}

pub struct Plot {}

impl Plot {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update_multi(
        &mut self,
        ctx: &egui::Context,
        name: String,
        x: Vec<f32>,
        named_ys: Vec<(String, Vec<f32>)>,
    ) {
        let n = x.len();
        if named_ys.iter().any(|(_, y)| y.len() != n) {
            dbg!("mismatch size");
            return;
        }

        egui::Window::new("Plots")
            .resizable(true)
            .default_pos(Pos2::new(100., 400.))
            .default_width(400.)
            .default_height(250.)
            .show(ctx, |ui| {
                egui_plot::Plot::new(name.clone())
                    .allow_zoom(true)
                    .allow_drag(true)
                    .allow_scroll(true)
                    .legend(Legend::default())
                    .show(ui, |plot_ui: &mut PlotUi| {
                        for (label, y) in named_ys {
                            let mut v: Vec<[f64; 2]> = Vec::with_capacity(n);
                            for i in 0..n {
                                v.push([x[i] as f64, y[i] as f64]);
                            }
                            let p = PlotPoints::new(v);
                            plot_ui.line(Line::new(p).name(label));
                        }
                    });
            });
    }
}
