use bevy::prelude::*;
use bevy::reflect::List;
use bevy::window::PrimaryWindow;
use bevy_egui::egui::Pos2;
use bevy_egui::egui::{Context, Window};
use bevy_egui::EguiContext;
use egui_plot::{Legend, Line, PlotPoints, PlotUi};

use crate::state;

pub fn update(
    outputs: ResMut<state::Outputs>,
    mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
) {
    let Ok(mut ctx) = egui_ctx.get_single_mut() else {
        return;
    };

    let Some(ball) = &outputs.ball else {
        return;
    };

    let mut plots = vec![];

    let x: Vec<f32> = ball.clone().position.into_iter().map(|p| p.x).collect();
    let y: Vec<f32> = ball.clone().position.into_iter().map(|p| p.z).collect();
    plots.push(Plot {
        name: "Position x/z".to_string(),
        x,
        y,
    });

    let x: Vec<f32> = ball.clone().position.into_iter().map(|p| p.x).collect();
    let y: Vec<f32> = ball.clone().position.into_iter().map(|p| p.y).collect();
    plots.push(Plot {
        name: "Position x/y".to_string(),
        x,
        y,
    });

    for (i, plot) in plots.iter().enumerate() {
        Window::new(plot.name.clone())
            .resizable(true)
            .default_pos(Pos2::new(100. + (i as f32 * 250.), 400.))
            .default_width(400.)
            .default_height(250.)
            .show(ctx.get_mut(), |ui| {
                egui_plot::Plot::new("plot")
                    .allow_zoom(true)
                    .allow_drag(true)
                    .allow_scroll(true)
                    .legend(Legend::default())
                    .show(ui, |plot_ui: &mut PlotUi| {
                        let n = plot.x.len();
                        let mut v: Vec<[f64; 2]> = vec![];
                        for i in 0..n {
                            let a = plot.x[i] as f64;
                            let b = plot.y[i] as f64;
                            v.push([a, b]);
                        }

                        let p = PlotPoints::new(v.clone());
                        plot_ui.line(Line::new(p).name(plot.name.clone()));
                    });
            });
    }
}

pub struct Plot {
    name: String,
    x: Vec<f32>,
    y: Vec<f32>,
}
