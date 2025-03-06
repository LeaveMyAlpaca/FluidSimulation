use bevy::{math::*, prelude::*};

use crate::{PARTICLE_RESOULTION, collisions::resolve_colisions};

// physics statics
const GRAVITY: Vec3 = Vec3::new(0f32, -10f32, 0f32);
const TIME_SCALE: f32 = 2f32;
const AIR_DENSITY: f32 = 1f32;
const PARTICLE_DRAG_COEFICIENT: f32 = 0.001f32;

pub fn handle_particles_physics(
    mut particles: Query<(&mut Transform, &mut Particle)>,
    time: Res<Time>,
) {
    let delta = time.delta().as_secs_f32() * TIME_SCALE;
    for (mut transform, mut particle) in &mut particles {
        // this is not great because we take velocity from last frame so the more frames we have
        // the more accurate the calculations will be, this is OK for our purpose
        let a = GRAVITY - calc_drag_force(particle.velocity, particle.area) / particle.mass;

        // s = vt + (at^2)/2
        let s = particle.velocity * delta + (a * delta.squared()) / 2.;
        transform.translation += s;
        particle.velocity += a * delta;
        resolve_colisions(&mut particle, &mut transform);
    }
}
fn calc_drag_force(velocity: Vec3, area: f32) -> Vec3 {
    // F = .5*d*v^2*C*A https://en.wikipedia.org/wiki/Drag_(physics)
    let speed_squared = velocity.length_squared();
    AIR_DENSITY * speed_squared * PARTICLE_DRAG_COEFICIENT * area / 2f32 * velocity.normalize()
}

#[derive(Component)]
pub(crate) struct Particle {
    pub ray: f32,
    pub mass: f32,
    pub velocity: Vec3,
    pub area: f32,
}
impl Particle {
    pub fn new(mass: f32, velocity: Vec3, ray: f32) -> Particle {
        Particle {
            ray,
            mass,
            velocity,
            area: Particle::calc_area(ray),
        }
    }
    fn calc_area(ray: f32) -> f32 {
        core::f32::consts::PI * ray.squared() * PARTICLE_RESOULTION
    }
}
