use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui::{self, DragValue, Event, Vec2};
use bevy_egui::EguiContext;
use egui_plot::{Legend, Line, PlotPoints};

use crate::state;

pub fn update(
    mut state: ResMut<state::Results>,
    mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
) {
    let Ok(mut ctx) = egui_ctx.get_single_mut() else {
        return;
    };

    let mut p = PlotExample::new();
    p.update(ctx.get_mut());
}

pub struct PlotExample {
    pub lock_x: bool,
    pub lock_y: bool,
    pub ctrl_to_zoom: bool,
    pub shift_to_horizontal: bool,
    pub zoom_speed: f32,
    pub scroll_speed: f32,
}

impl PlotExample {
    pub fn new() -> Self {
        Self {
            lock_x: false,
            lock_y: false,
            ctrl_to_zoom: false,
            shift_to_horizontal: false,
            zoom_speed: 1.0,
            scroll_speed: 1.0,
        }
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("plot")
            .resizable(true)
            .min_height(100.)
            .show(ctx, |ui| {
                egui_plot::Plot::new("plot")
                    .allow_zoom(true)
                    .allow_drag(true)
                    .allow_scroll(true)
                    .legend(Legend::default())
                    .show(ui, |plot_ui| {
                        let sine_points = PlotPoints::from_explicit_callback(|x| x.sin(), .., 5000);
                        plot_ui.line(Line::new(sine_points).name("Sine"));
                    });
            });
    }
}
