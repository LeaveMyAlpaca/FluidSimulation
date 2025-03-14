use bevy::{
    color::palettes::css::{DARK_BLUE, LIGHT_GREEN},
    math::{VectorSpace, vec3},
    prelude::*,
    sprite::Sprite,
};

use crate::{particle_physics::Particle, particles_spawning::PARTICLE_RAY};
const SHOW_PARTICLE_VISUALS: bool = true;
const SPEED_VISUALIZATION_SCALE: f32 = 80f32;

pub fn update_particles_visuals(mut particles: Query<(&mut Transform, &Particle, &mut Sprite)>) {
    if !SHOW_PARTICLE_VISUALS {
        return;
    }

    particles
        .iter_mut()
        .for_each(|(mut transform, particle, mut sprite)| {
            let t = particle.velocity.length() / SPEED_VISUALIZATION_SCALE;
            sprite.color = Color::Srgba(Srgba::lerp(DARK_BLUE, LIGHT_GREEN, t));

            let scale = PARTICLE_RAY
                /* * (pressure_handler::TARGET_DENSITY / particle.density).clamp(0.1f32, 3f32) */;
            transform.scale = vec3(scale, scale, 0f32);
        });
}
