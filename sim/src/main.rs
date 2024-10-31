mod ball;
mod camera;
mod csv;
mod plotting;
mod state;
mod ui;
mod world;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    // shared objs
    app.init_resource::<state::SharedInputs>()
        .init_resource::<state::SharedUiState>()
        .insert_resource(Time::<Fixed>::from_hz(100.));

    // plugins
    app.add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        );

    // state
    app.init_state::<state::AppState>();

    // systems
    app.add_systems(Startup, (world::setup, camera::setup))
        .add_systems(PreUpdate, state::trigger_restart)
        .add_systems(
            Update,
            (ui::inputs, ui::results, camera::pan, plotting::update),
        )
        .add_systems(FixedUpdate, ball::simulation);

    // state transitions
    app.add_systems(OnEnter(state::AppState::Restarting), world::setup)
        .add_systems(OnExit(state::AppState::Running), state::teardown);

    // run
    app.run();
}
