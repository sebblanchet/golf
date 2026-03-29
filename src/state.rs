use crate::bag;
use crate::ball;
use crate::constants::{DEFAULT_M, DEFAULT_MU, DEFAULT_R, DEFAULT_RHO};
use crate::shot;
use crate::ui;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::option::Option;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Units {
    Metric,
    Imperial,
}

#[derive(Debug, Resource, Clone)]
pub struct Inputs {
    pub m: f32,
    pub r: f32,
    pub rho: f32,
    pub position: Vec3,
    pub velocity: Vec3,
    pub spin: Vec3,
    pub club: bag::Club,
    pub _hand: shot::Hand,
    pub _shot: shot::Shot,
    pub decel: f32,
    pub mu: f32,
    pub units: Units,
    pub c_d: f32,
    pub c_m: f32,
}

impl Default for Inputs {
    fn default() -> Self {
        let club = bag::Club::default();
        let vx = ball::vx(club.speed, club.loft, club.smash);
        let vy = ball::vy(club.speed, club.loft, club.smash);
        let velocity = Vec3::new(vx, vy, 0.);
        let spin = Vec3::new(0., 0., club.spin);

        Self {
            m: DEFAULT_M,
            r: DEFAULT_R,
            rho: DEFAULT_RHO,
            club,
            position: Vec3::ZERO,
            velocity,
            spin,
            _hand: shot::Hand::Right,
            _shot: shot::Shot::Straight,
            decel: 0.,
            mu: DEFAULT_MU,
            units: Units::Metric,
            c_d: 0.,
            c_m: 0.,
        }
    }
}

impl Inputs {
    pub fn update(&mut self) {
        // update velocity and spins
        info!("club change");
        self.velocity.x = ball::vx(self.club.speed, self.club.loft, self.club.smash);
        self.velocity.y = ball::vy(self.club.speed, self.club.loft, self.club.smash);
        self.spin.z = self.club.spin;
    }
}

#[derive(Resource, Default)]
pub struct Outputs {
    pub ball: Option<ball::Ball>,
}

// boilerplate for setting up a basic restarting architecture:
/// the two states (re)starting and running
#[derive(States, Default, Debug, Clone, Hash, Eq, PartialEq)]
pub enum AppState {
    /// nothing happens in this state other than moving immediately to the running state
    #[default]
    Restarting,
    // when we enter this state, we run any user-defined setup code
    // when we exit this state we tear down anything that was spawned
    Running,
    // wait for restart
    Waiting,
}

pub fn trigger_restart(
    input: Res<ButtonInput<KeyCode>>,
    _state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut ui: ResMut<ui::Ui>,
) {
    // spacebar
    if input.just_pressed(KeyCode::Space) {
        info!("user triggered restart with spacebar");
        next_state.set(AppState::Restarting);
    }

    // button
    if ui.restart {
        info!("user triggered restart with button");
        next_state.set(AppState::Restarting);
        ui.restart = false;
    }
}

pub fn teardown(
    mut commands: Commands,
    mut outputs: ResMut<Outputs>,
    query: Query<Entity, (Without<PrimaryWindow>, Without<crate::camera::Camera>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    // wipe ball
    outputs.ball = None;
}

// CSV export is now user-triggered from the UI (Stats panel)
