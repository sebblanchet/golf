use bevy::prelude::*;

#[derive(Debug, Resource, Clone)]
pub struct Ui {
    pub restart: bool,
    pub open_inputs: bool,
    pub open_about: bool,
    pub open_plots: bool,
    pub open_stats: bool,
}

impl Default for Ui {
    fn default() -> Self {
        Self {
            restart: false,
            open_inputs: true,
            open_about: true,
            open_plots: true,
            open_stats: true,
        }
    }
}
