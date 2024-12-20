use bevy::color::palettes::basic::{GREEN, WHITE};
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;

use crate::ball;
use crate::state;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_state: ResMut<NextState<state::AppState>>,
    inputs: ResMut<state::Inputs>,
    time: Res<Time>,
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

    // sky
    let m = max * 10.;
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(m, m, m)),
            material: materials.add(StandardMaterial {
                base_color: Srgba::hex("888888").unwrap().into(),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(20.0)),
            ..default()
        },
        NotShadowCaster,
    ));

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(
            Plane3d::default()
                .mesh()
                .size(4. * max, max)
                .subdivisions(10),
        ),
        material: materials.add(Color::from(GREEN)),
        ..default()
    });

    // start
    next_state.set(state::AppState::Running);
}
