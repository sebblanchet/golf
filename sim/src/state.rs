use bevy::prelude::*;
use bevy::window::PrimaryWindow;

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

#[derive(Resource)]
pub struct SharedUiState {
    pub time: String,
    pub position: String,
    pub velocity: String,
    pub angular: String,
    pub points: Vec<(f32, f32, f32)>,
}

impl Default for SharedUiState {
    fn default() -> Self {
        Self {
            time: "".to_string(),
            position: "".to_string(),
            velocity: "".to_string(),
            angular: "".to_string(),
            points: vec![(0., 0., 0.)],
        }
    }
}

#[derive(Debug, Resource)]
pub struct SharedInputs {
    pub m: f32,
    pub r: f32,
    pub c_d: f32,
    pub c_m: f32,
    pub rho: f32,
    pub position: Vec3,
    pub velocity: Vec3,
    pub angular: Vec3,
}

impl Default for SharedInputs {
    fn default() -> Self {
        let m = 0.04593; // mass of the ball in kg (e.g., a standard baseball)
        let r = 0.04267 / 2.; // radius of the ball in meters
        let c_d = 0.47; // drag coefficient
        let c_m = 0.2; // Magnus coefficient (this is a rough estimate)
        let rho = 1.225; // air density in kg/m^3
        let position = Vec3::ZERO;
        let velocity = Vec3::new(10., 10., 0.);
        let angular = Vec3::new(100., 0., 0.);

        Self {
            m,
            r,
            c_d,
            c_m,
            rho,
            position,
            velocity,
            angular,
        }
    }
}

pub fn teardown(
    mut commands: Commands,
    mut shared_ui_state: ResMut<SharedUiState>,
    query: Query<Entity, (Without<PrimaryWindow>, Without<crate::camera::Camera>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    shared_ui_state.points = vec![(0., 0., 0.)];
}
