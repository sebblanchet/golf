mod bag;
mod ball;
mod camera;
mod csv;
mod inputs;
mod logging;
mod plotting;
mod plugins;
mod shot;
mod state;
mod stats;
mod world;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    // shared objs
    app.init_resource::<state::Inputs>()
        .init_resource::<state::Outputs>()
        .init_resource::<bag::Bag>()
        .insert_resource(Time::<Fixed>::from_hz(100.));

    // plugins
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "golf".into(),
                    resolution: WindowResolution::new(1024., 768.),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .add_plugins(EguiPlugin)
    .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)))
    .add_plugins(plugins::Gpu);

    // state
    app.init_state::<state::AppState>();

    // systems
    // TODO add pre Startup
    app.add_systems(Startup, (world::setup, camera::setup))
        .add_systems(PreUpdate, state::trigger_restart)
        .add_systems(
            Update,
            (inputs::update, stats::update, camera::pan, plotting::update),
        )
        .add_systems(FixedUpdate, ball::simulation);

    // state transitions
    app.add_systems(OnEnter(state::AppState::Restarting), world::setup)
        .add_systems(OnExit(state::AppState::Waiting), state::teardown);

    // run
    app.run();
}
