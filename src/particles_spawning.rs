use crate::{bounding_box::BOX_BOUNDS_SIZE_PIXELS, particle_physics};
use bevy::{math::vec2, prelude::*};
use rand::{Rng, rngs::ThreadRng};

const CIRCLE_SPRITE_PATH: &str = "sprites/circle.png";
pub const PARTICLE_RAY: f32 = 0.03f32;
pub const PARTICLE_RESOLUTION: f32 = 50f32;
pub const STANDARD_PARTICLE_MASS: f32 = 2f32;

pub fn handle_spawning_particles(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let mut rng = rand::rng();

    for i in 0..PARTICLES_COUNT {
        spawn_particle(
            STANDARD_PARTICLE_MASS,
            PARTICLE_RAY,
            get_particle_spawn_position(i as f32, &mut rng),
            i as usize,
            commands,
            asset_server,
        );
    }
}
fn spawn_particle(
    mass: f32,
    ray: f32,
    pos: Vec2,
    index: usize,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let sprite = Sprite::from_image(asset_server.load(CIRCLE_SPRITE_PATH));

    let transform = Transform {
        translation: Vec3::new(pos.x, pos.y, 0f32),
        scale: Vec3::new(ray, ray, ray),
        ..default()
    };
    let particle = particle_physics::Particle::new(mass, vec2(1f32, 0f32), PARTICLE_RAY, index);

    commands.spawn((particle, transform, sprite));
}

pub const PARTICLES_COUNT: u32 = 80000;
const PARTICLES_LAYERS: u32 = 200;
const PARTICLES_SPACING: f32 = 3f32;

const PARTICLES_SIZE_ASPECT: f32 = PARTICLES_COUNT as f32 / PARTICLES_LAYERS as f32;
const OFFSET_VEC: Vec2 = vec2(
    -PARTICLES_SPACING * PARTICLES_SIZE_ASPECT / 2f32,
    -(PARTICLES_LAYERS as f32 / 2f32) * PARTICLES_SPACING,
);

fn get_particle_spawn_position(index: f32, rng: &mut ThreadRng) -> Vec2 {
    get_box_spawn_point(index)
    //get_random_spawn_point(rng)
}
fn get_random_spawn_point(rng: &mut ThreadRng) -> Vec2 {
    let real_box_size = BOX_BOUNDS_SIZE_PIXELS / 2f32;
    let y = rng.random_range(-(real_box_size.y) as i32..(real_box_size.y) as i32);
    let x = rng.random_range(-(real_box_size.x) as i32..(real_box_size.x) as i32);

    vec2(x as f32, y as f32)
}
fn get_box_spawn_point(index: f32) -> Vec2 {
    let y = (index / PARTICLES_SIZE_ASPECT).floor();
    let x = index - y * PARTICLES_SIZE_ASPECT;

    vec2(x, y) * PARTICLES_SPACING + OFFSET_VEC
}
