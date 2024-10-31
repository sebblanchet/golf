//! Simple spiral example using the pre-defined xytime chart type

use crate::plotting::charts::xytime::XyTimeData;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::f32::consts::PI;

use bevy_egui::egui::{CentralPanel, Slider, TopBottomPanel, Visuals, Window};
use bevy_egui::EguiContext;
use plotters::style::{
    full_palette::{GREY_400, GREY_700, GREY_900, ORANGE_50},
    ShapeStyle, WHITE,
};

const SPIRAL_LEN: usize = 10;
const SPIRAL_SUB: usize = 100;

pub fn init() -> XyTimeData {
    let mut points: Vec<(f32, f32, f32)> = Vec::with_capacity(SPIRAL_LEN * SPIRAL_SUB);

    let mut scale = 1.0 / SPIRAL_SUB as f32;
    let mut rev = PI / SPIRAL_SUB as f32;

    for i in 0..SPIRAL_LEN * SPIRAL_SUB {
        points.push((
            rev.sin() * scale,
            rev.cos() * scale,
            i as f32 / SPIRAL_SUB as f32,
        ));

        scale += 1.0 / SPIRAL_SUB as f32;
        rev += PI / SPIRAL_SUB as f32;
    }

    XyTimeData::new(&points, "", "", "")
        .line_style(ShapeStyle {
            color: WHITE.into(),
            filled: false,
            stroke_width: 1,
        })
        .grid_style(ShapeStyle {
            color: GREY_700.into(),
            filled: false,
            stroke_width: 2,
        })
        .subgrid_style(ShapeStyle {
            color: GREY_900.into(),
            filled: false,
            stroke_width: 1,
        })
        .axes_style(ShapeStyle {
            color: GREY_400.into(),
            filled: false,
            stroke_width: 2,
        })
        .text_color(ORANGE_50)
}

pub fn init2(points: Vec<(f32, f32, f32)>) -> XyTimeData {
    XyTimeData::new(&points, "", "", "")
        .line_style(ShapeStyle {
            color: WHITE.into(),
            filled: false,
            stroke_width: 1,
        })
        .grid_style(ShapeStyle {
            color: GREY_700.into(),
            filled: false,
            stroke_width: 2,
        })
        .subgrid_style(ShapeStyle {
            color: GREY_900.into(),
            filled: false,
            stroke_width: 1,
        })
        .axes_style(ShapeStyle {
            color: GREY_400.into(),
            filled: false,
            stroke_width: 2,
        })
        .text_color(ORANGE_50)
}

pub fn update(
    shared_ui_state: ResMut<crate::state::SharedUiState>,
    mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
) {
    let Ok(mut ctx) = egui_ctx.get_single_mut() else {
        return;
    };

    let mut spiralchart = init2(shared_ui_state.points.clone());

    ctx.get_mut().tessellation_options_mut(|tess_options| {
        tess_options.feathering = false;
    });
    ctx.get_mut().set_visuals(Visuals::dark());

    Window::new("plot").show(ctx.get_mut(), |ui| {
        CentralPanel::default().show_inside(ui, |ui| {
            spiralchart.draw(ui);
        });
    });

    ctx.get_mut().request_repaint();
}
