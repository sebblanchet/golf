use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui;
use bevy_egui::EguiContext;
use egui_plot::{Legend, Line, PlotPoints};
use std::option::Option;

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

    let mut p = Plot::new();
    let x: Vec<f32> = ball.clone().position.into_iter().map(|p| p.x).collect();
    let y: Vec<f32> = ball.clone().position.into_iter().map(|p| p.y).collect();
    p.update(ctx.get_mut(), "position x/y".to_string(), x, y);

    let x: Vec<f32> = ball.clone().position.into_iter().map(|p| p.x).collect();
    let y: Vec<f32> = ball.clone().position.into_iter().map(|p| p.z).collect();
    p.update(ctx.get_mut(), "position x/z".to_string(), x, y);

    //let x: Vec<f32> = ball.clone().time;
    //let y: Vec<f32> = ball.clone().velocity.into_iter().map(|p| p.x).collect();
    //p.update(ctx.get_mut(), "velocity x".to_string(), x, y);
    //let x: Vec<f32> = ball.clone().time;
    //let y: Vec<f32> = ball.clone().velocity.into_iter().map(|p| p.y).collect();
    //p.update(ctx.get_mut(), "velocity y".to_string(), x, y);
}

pub struct Plot {}

impl Plot {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, ctx: &egui::Context, name: String, x: Vec<f32>, y: Vec<f32>) {
        let n = x.len();
        let m = y.len();
        if m != n {
            dbg!("mismatch size");
            return;
        }
        egui::Window::new(name)
            .resizable(true)
            .min_height(250.)
            .min_width(250.)
            .show(ctx, |ui| {
                egui_plot::Plot::new("plot")
                    .allow_zoom(true)
                    .allow_drag(true)
                    .allow_scroll(true)
                    .legend(Legend::default())
                    .show(ui, |plot_ui| {
                        // clean
                        let mut v: Vec<[f64; 2]> = vec![];
                        for i in 0..n {
                            let a = x[i] as f64;
                            let b = y[i] as f64;
                            v.push([a, b]);
                        }

                        let p = PlotPoints::new(v.clone());
                        plot_ui.line(Line::new(p));
                    });
            });
    }
}
