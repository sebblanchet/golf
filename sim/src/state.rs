use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::option::Option;

use crate::bag;
use crate::ball;
use crate::shot;

#[derive(Debug, Resource, Clone)]
pub struct Inputs {
    pub m: f32,
    pub r: f32,
    pub rho: f32,
    pub position: Vec3,
    pub velocity: Vec3,
    pub spin: Vec3,
    pub club: bag::Club,
    pub hand: shot::Hand,
    pub shot: shot::Shot,
    pub decel: f32,
    pub mu: f32,
}

impl Default for Inputs {
    fn default() -> Self {
        let m = 0.04593; // mass of the ball in kg (e.g., a standard baseball)
        let r = 0.04267 / 2.; // radius of the ball in meters
        let rho = 1.225; // air density in kg/m^3
        let decel = 1.;
        let mu = 1.46e-5;

        let club = bag::Club::default();
        let position = Vec3::ZERO;
        let velocity = Vec3::new(70., 20., 0.);
        let spin = Vec3::new(0., 0., 250.);
        let hand = shot::Hand::Left;
        let shot = shot::Shot::Straight;

        Self {
            m,
            r,
            rho,
            club,
            position,
            velocity,
            spin,
            hand,
            shot,
            decel,
            mu,
        }
    }
}

impl Inputs {
    pub fn update(&mut self) {
        // update velocity and spins
        info!("club change");
        self.velocity.x = ball::vx(self.club.speed, self.club.loft, self.club.weight, self.m);
        self.velocity.y = ball::vy(
            self.club.speed,
            self.club.loft,
            self.club.weight,
            self.club.inertia,
            self.m,
            self.r,
        );
        self.spin.z = ball::spin(
            self.club.speed,
            self.club.loft,
            self.club.weight,
            self.club.inertia,
            self.m,
            self.r,
        );
    }
}

#[derive(Resource, Default)]
pub struct Outputs {
    pub ball: Option<ball::Ball>,
}

//impl Default for Outputs {
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
    // wait for restart
    Waiting,
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
    mut outputs: ResMut<Outputs>,
    query: Query<Entity, (Without<PrimaryWindow>, Without<crate::camera::Camera>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    // wipe ball
    outputs.ball = None;
}
