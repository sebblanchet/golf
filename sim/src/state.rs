use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::option::Option;

use crate::bag;
use crate::ball;

#[derive(Debug, Resource)]
pub struct Inputs {
    pub m: f32,
    pub r: f32,
    pub c_d: f32,
    pub c_m: f32,
    pub rho: f32,
    pub position: Vec3,
    pub velocity: Vec3,
    pub angular: Vec3,
    pub bag: bag::Bag,
    pub club: bag::Club,
}

impl Default for Inputs {
    fn default() -> Self {
        let m = 0.04593; // mass of the ball in kg (e.g., a standard baseball)
        let r = 0.04267 / 2.; // radius of the ball in meters
        let c_d = 0.47; // drag coefficient
        let c_m = 0.2; // Magnus coefficient (this is a rough estimate)
        let rho = 1.225; // air density in kg/m^3
        let position = Vec3::ZERO;
        let velocity = Vec3::new(10., 10., 0.);
        let angular = Vec3::new(100., 0., 0.);
        let bag = bag::Bag::default();
        let club = bag.get("1w".to_string());

        dbg!(&club);
        dbg!(&bag.list());

        Self {
            m,
            r,
            c_d,
            c_m,
            rho,
            position,
            velocity,
            angular,
            bag,
            club,
        }
    }
}

#[derive(Resource)]
pub struct Ouputs {
    pub ball: Option<ball::Ball>,
}

impl Default for Ouputs {
    fn default() -> Self {
        Self { ball: None }
    }
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
}

pub fn trigger_restart(
    input: Res<ButtonInput<KeyCode>>,
    _state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        dbg!("user triggered restart");
        next_state.set(AppState::Restarting);
    }
}

pub fn teardown(
    mut commands: Commands,
    mut outputs: ResMut<Ouputs>,
    query: Query<Entity, (Without<PrimaryWindow>, Without<crate::camera::Camera>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    // wipe ball
    outputs.ball = None;
}
