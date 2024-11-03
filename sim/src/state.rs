use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::option::Option;

use crate::bag;
use crate::ball;
use crate::shot;

#[derive(Debug, Resource)]
pub struct Inputs {
    pub m: f32,
    pub r: f32,
    pub c_d: f32,
    pub c_m: f32,
    pub rho: f32,
    pub position: Vec3,
    pub velocity: Vec3,
    pub spin: Vec3,
    pub club: bag::Club,
    pub hand: shot::Hand,
    pub shot: shot::Shot,
}

impl Default for Inputs {
    fn default() -> Self {
        let m = 0.04593; // mass of the ball in kg (e.g., a standard baseball)
        let r = 0.04267 / 2.; // radius of the ball in meters
        let c_d = 0.4; // drag coefficient
        let c_m = 0.2; // Magnus coefficient (this is a rough estimate)
        let rho = 1.225; // air density in kg/m^3
        let club = bag::Club::default();

        let position = Vec3::ZERO;
        let velocity = Vec3::new(70., 20., 0.);
        let spin = Vec3::new(0., 0., 250.);
        let hand = shot::Hand::Left;
        let shot = shot::Shot::Straight;

        Self {
            m,
            r,
            c_d,
            c_m,
            rho,
            club,
            position,
            velocity,
            spin,
            hand,
            shot,
        }
    }
}

impl Inputs {
    pub fn update(&mut self) {
        // update velocity and spins
        dbg!("club change");
        dbg!(&self.club);
        let rad = self.club.loft.to_radians();
        self.velocity.x = self.club.speed * rad.cos();
        self.velocity.y = self.club.speed * rad.sin();
        self.spin.z = self.club.spin;
    }
}

#[derive(Resource, Default)]
pub struct Ouputs {
    pub ball: Option<ball::Ball>,
}

//impl Default for Ouputs {
//    fn default() -> Self {
//        Self { ball: None }
//    }
//}

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
    if input.just_pressed(KeyCode::Space) {
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
