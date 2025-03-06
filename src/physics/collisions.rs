use bevy::{math::Vec2, prelude::Transform};

const COLISION_DAMPING: f32 = 0.5f32;
use crate::{PARTICLE_RAY, PARTICLE_RESOULTION, boundingBox, particlePhysics::Particle};
pub fn resolve_colisions(particle: &mut Particle, transform: &mut Transform) {
    let half_bouds_size =
        boundingBox::BOX_BOUNDS_SIZE / 2f32 - Vec2::ONE * PARTICLE_RAY * PARTICLE_RESOULTION / 2f32;

    if transform.translation.x.abs() > half_bouds_size.x {
        transform.translation.x = half_bouds_size.x * transform.translation.x.signum();
        particle.velocity.x *= -1f32 * COLISION_DAMPING;
    }
    if transform.translation.y.abs() > half_bouds_size.y {
        transform.translation.y = half_bouds_size.y * transform.translation.y.signum();
        particle.velocity.y *= -1f32 * COLISION_DAMPING;
    }
}
