mod boundingBox;
#[path = "physics/collisions.rs"]
mod collisions;
#[path = "physics/particlePhysics.rs"]
mod particlePhysics;
use core::f32;

use bevy::{
    math::{FloatPow, vec2, vec3},
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

    for i in 0..PARTICLES_TO_SPAWN {
        spawn_particle(
            STANDARD_PARTICLE_MASS,
            PARTICLE_RAY,
            get_particle_spawn_position(i as f32),
            &mut commands,
            &asset_server,
        );
    }
}

fn spawn_particle(
    mass: f32,
    ray: f32,
    pos: Vec2,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let sprite = Sprite::from_image(asset_server.load(CIRCLE_SPRITE_PATH));

    let transform = Transform {
        translation: Vec3::new(pos.x, pos.y, 0f32),
        scale: Vec3::new(ray, ray, ray),
        ..default()
    };
    let particle = particlePhysics::Particle::new(mass, vec3(1f32, 0f32, 0f32), PARTICLE_RAY);

    commands.spawn((particle, transform, sprite));
}
//To Adjust
const PARTICLES_TO_SPAWN: u32 = 10000;
const PARTICLES_LAYERS: u32 = 50;
const PARTICLES_SPACING: f32 = 20f32;

const PARTICLES_SIZE_ASPECT: f32 = PARTICLES_TO_SPAWN as f32 / PARTICLES_LAYERS as f32;
const OFFSET_VEC: Vec2 = vec2(
    -PARTICLES_SPACING * PARTICLES_SIZE_ASPECT / 2f32,
    -(PARTICLES_LAYERS as f32 / 2f32) * PARTICLES_SPACING,
);
fn get_particle_spawn_position(index: f32) -> Vec2 {
    let y = (index / PARTICLES_SIZE_ASPECT).floor();
    let x = index - y * PARTICLES_SIZE_ASPECT;

    vec2(x, y) * PARTICLES_SPACING + OFFSET_VEC
}
