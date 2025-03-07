mod boundingBox;
#[path = "physics/collisions.rs"]
mod collisions;
#[path = "physics/particlePhysics.rs"]
mod particlePhysics;
mod particlesSpawning;
#[path = "physics/pressureHandler.rs"]
mod pressureHandler;

use bevy::{math::vec2, prelude::*};
use boundingBox::{BOX_BOUNDS_SIZE, spawn_bounding_box};
use rand::{Rng, rngs::ThreadRng};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_bounding_box))
        .add_systems(Update, (particlePhysics::handle_particles_physics))
        .run();
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Text::new("fps ->"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.),
            left: Val::Px(12.),
            ..default()
        },
    ));

    particlesSpawning::handle_spawning_particles(&mut commands, &asset_server);
}
