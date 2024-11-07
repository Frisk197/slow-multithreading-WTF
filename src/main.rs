mod uVec3;
mod components;
mod systems;

use bevy::app::{App, Startup, Update};
use bevy::DefaultPlugins;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use crate::systems::{run_simulation_multithread, run_simulation, setup_tilemap};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,
        LogDiagnosticsPlugin::default(),
        FrameTimeDiagnosticsPlugin,))
        .add_systems(Startup, (setup_tilemap))
        .add_systems(Update, (run_simulation)) // change to run_simulation_multithread for multithreading
        .run();
}
