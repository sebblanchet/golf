use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui;
use bevy_egui::EguiContext;
use egui_plot::{Legend, Line, PlotPoints};

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

    let mut p = Plot::new("position x".to_string());
    let x: Vec<f32> = ball.clone().position.into_iter().map(|p| p.x).collect();
    let y: Vec<f32> = ball.clone().position.into_iter().map(|p| p.y).collect();
    p.update(ctx.get_mut(), x, y);
}

pub struct Plot {
    name: String,
}

impl Plot {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn update(&mut self, ctx: &egui::Context, x: Vec<f32>, y: Vec<f32>) {
        let n = x.len();
        let m = y.len();
        if m != n {
            dbg!("mismatch size");
            return;
        }
        egui::TopBottomPanel::bottom("plot")
            .resizable(true)
            .min_height(100.)
            .show(ctx, |ui| {
                egui_plot::Plot::new("plot")
                    .allow_zoom(true)
                    .allow_drag(true)
                    .allow_scroll(true)
                    .include_y(10.)
                    .include_x(20.)
                    .legend(Legend::default())
                    .show(ui, |plot_ui| {
                        // clean
                        let mut p: Vec<[f64; 2]> = vec![];
                        for i in 0..n {
                            let a = x[i] as f64;
                            let b = y[i] as f64;
                            p.push([a, b]);
                        }

                        let pp = PlotPoints::new(p.clone());
                        plot_ui.line(Line::new(pp).name(self.name.clone()));

                        let pp = PlotPoints::new(p);
                        plot_ui.line(Line::new(pp).name("p2"));
                    });
            });
    }
}
