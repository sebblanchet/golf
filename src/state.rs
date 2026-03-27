use crate::bag;
use crate::ball;
use crate::constants::{DEFAULT_M, DEFAULT_MU, DEFAULT_R, DEFAULT_RHO};
use crate::shot;
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
        let m = DEFAULT_M;
        let r = DEFAULT_R;
        let rho = DEFAULT_RHO;
        let mu = DEFAULT_MU;
        let decel = 0.;
        let c_d = 0.;
        let c_m = 0.;

        let club = bag::Club::default();
        let position = Vec3::ZERO;
        let velocity = Vec3::new(70., 20., 0.);
        let spin = Vec3::new(0., 0., 250.);
        let _hand = shot::Hand::Left;
        let _shot = shot::Shot::Straight;
        let units = Units::Metric;

        Self {
            m,
            r,
            rho,
            club,
            position,
            velocity,
            spin,
            _hand,
            _shot,
            decel,
            mu,
            units,
            c_d,
            c_m,
        }
    }
}

impl Inputs {
    pub fn update(&mut self) {
        // update velocity and spins
        info!("club change");
        let vy = ball::vy(self.club.speed, self.club.loft, self.club.smash);
        self.velocity.x = ball::vx(self.club.speed, self.club.loft, self.club.smash);
        self.velocity.y = vy;
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
) {
    if input.just_pressed(KeyCode::Space) {
        info!("user triggered restart");
        next_state.set(AppState::Restarting);
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
