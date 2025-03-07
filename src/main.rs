mod bounding_box;
#[path = "physics/collisions.rs"]
mod collisions;
#[path = "physics/particle_physics.rs"]
mod particle_physics;
mod particles_spawning;
#[path = "physics/pressure_handler.rs"]
mod pressure_handler;
mod ui_handler;

use bevy::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, bounding_box::spawn_bounding_box))
        .add_systems(
            Update,
            (
                particle_physics::handle_particles_physics,
                ui_handler::update_ui,
            ),
        )
        .run();
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    ui_handler::setup_ui(&mut commands);
    particles_spawning::handle_spawning_particles(&mut commands, &asset_server);
}
