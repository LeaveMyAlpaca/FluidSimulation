use std::borrow::Borrow;

use bevy::{
    color::{
        self,
        palettes::css::{
            BROWN, DARK_BLUE, GREEN, LIGHT_BLUE, LIGHT_CYAN, LIGHT_GREEN, ORANGE, RED, YELLOW,
        },
    },
    gizmos,
    math::{VectorSpace, vec3},
    prelude::*,
    sprite::Sprite,
};

use crate::{
    gradient::{ColorGradient, ColorKey},
    particle_physics::Particle,
    particles_spawning::PARTICLE_RAY,
    pressure_handler,
};
const SHOW_PARTICLE_VISULAS: bool = true;
pub fn update_particles_visuals(
    mut particles: Query<(&mut Transform, &Particle, &mut Sprite)>,
    mut gizmos: Gizmos,
) {
    if !SHOW_PARTICLE_VISULAS {
        return;
    }

    particles
        .iter_mut()
        .for_each(|(mut transform, particle, mut sprite)| {
            let t = particle.velocity.length() / 10f32;
            sprite.color = Color::Srgba(Srgba::lerp(DARK_BLUE, LIGHT_GREEN, t));

            let scale = PARTICLE_RAY
                * (pressure_handler::TARGET_DENSITY / particle.density).clamp(0.1f32, 3f32);
            transform.scale = vec3(scale, scale, 0f32);
        });
}
