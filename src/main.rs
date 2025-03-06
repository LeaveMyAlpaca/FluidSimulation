mod boundingBox;
#[path = "physics/collisions.rs"]
mod collisions;
#[path = "physics/particlePhysics.rs"]
mod particlePhysics;
use core::f32;

use bevy::{
    math::{FloatPow, vec3},
    prelude::*,
};
use boundingBox::spawn_bounding_box;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_bounding_box))
        .add_systems(Update, (particlePhysics::handle_particles_physics))
        .run();
}
const CIRCLE_SPRITE_PATH: &str = "sprites/circle.png";
const PARTICLE_RAY: f32 = 0.4f32;
const PARTICLE_RESOULTION: f32 = 50f32;
const STANDARD_PARTICLE_MASS: f32 = 20f32;
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    spawn_particle(STANDARD_PARTICLE_MASS, PARTICLE_RAY, commands, asset_server);
}

fn spawn_particle(mass: f32, ray: f32, mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite = Sprite::from_image(asset_server.load(CIRCLE_SPRITE_PATH));

    let transform = Transform {
        translation: Vec3::new(0f32, 300f32, 0f32),
        scale: Vec3::new(ray, ray, ray),
        ..default()
    };
    let particle = particlePhysics::Particle::new(mass, vec3(20f32, 0f32, 0f32), PARTICLE_RAY);

    commands.spawn((particle, transform, sprite));
}
