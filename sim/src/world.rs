use bevy::color::palettes::basic::{GREEN, WHITE};
use bevy::prelude::*;

use crate::ball;
use crate::state;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_state: ResMut<NextState<state::AppState>>,
    time: Res<Time>,
    inputs: ResMut<state::Inputs>,
) {
    let sphere = meshes.add(Sphere::new(0.1).mesh());
    let ball = ball::Ball::new(inputs.as_ref(), time.elapsed().as_secs_f32());
    ball.save_params();

    commands
        .spawn(PbrBundle {
            mesh: sphere,
            material: materials.add(Color::from(WHITE)),
            ..default()
        })
        .insert(ball);

    let max = 200.;

    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(-max, max, 0.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(
            Plane3d::default()
                .mesh()
                .size(max, max * 0.5)
                .subdivisions(10),
        ),
        material: materials.add(Color::from(GREEN)),
        ..default()
    });

    // start
    next_state.set(state::AppState::Running);
}
