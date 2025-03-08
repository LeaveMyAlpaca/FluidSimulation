use bevy::{math::Vec2, prelude::Transform};

const COLISION_DAMPING: f32 = 0.5f32;
use crate::{bounding_box, particle_physics::Particle, particles_spawning};
pub fn resolve_colisions(particle: &mut Particle, transform: &mut Transform) {
    let half_bouds_size = bounding_box::BOX_BOUNDS_SIZE_PIXELS / 2f32
        - Vec2::ONE * particles_spawning::PARTICLE_RAY * particles_spawning::PARTICLE_RESOULTION
            / 2f32;

    if transform.translation.x.abs() > half_bouds_size.x {
        transform.translation.x = half_bouds_size.x * transform.translation.x.signum();
        particle.velocity.x *= -1f32 * COLISION_DAMPING;
    }
    if transform.translation.y.abs() > half_bouds_size.y {
        transform.translation.y = half_bouds_size.y * transform.translation.y.signum();
        particle.velocity.y *= -1f32 * COLISION_DAMPING;
    }
}
