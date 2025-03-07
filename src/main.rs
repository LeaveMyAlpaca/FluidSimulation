mod boundingBox;
#[path = "physics/collisions.rs"]
mod collisions;
#[path = "physics/particlePhysics.rs"]
mod particlePhysics;
mod particlesSpawning;
#[path = "physics/pressureHandler.rs"]
mod pressureHandler;
mod uiHandler;

use bevy::{math::vec2, prelude::*};
use boundingBox::BOX_BOUNDS_SIZE;
use rand::{Rng, rngs::ThreadRng};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, boundingBox::spawn_bounding_box))
        .add_systems(
            Update,
            (
                particlePhysics::handle_particles_physics,
                uiHandler::update_ui,
            ),
        )
        .run();
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    uiHandler::setup_ui(&mut commands);
    particlesSpawning::handle_spawning_particles(&mut commands, &asset_server);
}
