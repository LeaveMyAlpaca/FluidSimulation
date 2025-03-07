use std::usize;

use crate::{
    PARTICLE_RESOULTION, PARTICLES_TO_SPAWN,
    collisions::resolve_colisions,
    pressureHandler::{self, calculate_pressure_force},
};
use bevy::{color::palettes::css::CORAL, gizmos, math::*, prelude::*};

// physics settings
const GRAVITY: Vec2 = Vec2::new(0f32, 0f32);
const TIME_SCALE: f32 = 2f32;
const AIR_DENSITY: f32 = 1f32;
const PARTICLE_DRAG_COEFICIENT: f32 = 0.001f32;
const PRESSURE_FORCE_MODIFIER: f32 = 2f32;

const DEBUG_USE_PRESSURE: bool = true;
const DEBUG_RUN_PARTICLE_PHYSICS: bool = true;
const DEBUG_SHOW_PARTICLE_DENSITY: bool = false;
const DEBUG_SHOW_PARTICLE_PRESSURE: bool = true;
const DEBUG_SHOW_DISTANCE_CHECK: bool = false;

pub fn handle_particles_physics(
    mut particles: Query<(&mut Transform, &mut Particle), With<Particle>>,
    time: Res<Time>,
    mut gizmos: Gizmos,
) {
    //
    let mut particle_points = Vec::with_capacity(PARTICLES_TO_SPAWN as usize);
    for (transform, _) in &particles {
        particle_points.push(transform.translation.xy());
    }
    let densities = &pressureHandler::calculate_density_for_every_particle(&particle_points);
    let delta = time.delta().as_secs_f32() * TIME_SCALE;

    let mut debug_particle_index: i32 = -1;
    for (mut transform, mut particle) in &mut particles {
        debug_particle_index += 1;
        if DEBUG_SHOW_PARTICLE_DENSITY {
            println!(
                "density:{} {}",
                densities[debug_particle_index as usize], debug_particle_index
            );
            let scale = crate::PARTICLE_RAY * densities[debug_particle_index as usize] * 200f32;
            transform.scale = vec3(scale, scale, 1f32);
        }
        if DEBUG_SHOW_DISTANCE_CHECK {
            let mut points = 0f32;
            for point in &particle_points {
                let pos = transform.translation.xy();
                if point == &pos {
                    continue;
                }
                points += 10f32 / point.distance_squared(pos);
            }
            println!("points: {}", points);
            let scale = crate::PARTICLE_RAY * points * 5f32;
            transform.scale = vec3(scale, scale, 1f32);
        }

        let pressure_forece: Vec2 = if DEBUG_USE_PRESSURE {
            -calculate_pressure_force(debug_particle_index as usize, &particle_points, densities)
        } else {
            Vec2::ZERO
        };
        if DEBUG_SHOW_PARTICLE_PRESSURE {
            let pos = transform.translation.xy();
            println!("pressure :{}", pressure_forece);
            gizmos.arrow_2d(pos, pos + pressure_forece, CORAL);
        }

        // this is not great because we take velocity for drag calculation from last frame so the more frames we have
        // the more accurate the calculations will be, this is OK for our purpose
        //WARN: because of that our calculations could be UN deterministic ?
        let f = pressure_forece * PRESSURE_FORCE_MODIFIER
            - calc_drag_force(particle.velocity, particle.area);

        let a = GRAVITY + f / particle.mass;
        if !DEBUG_RUN_PARTICLE_PHYSICS {
            continue;
        }
        // s = vt + (at^2)/2
        let s = particle.velocity * delta + (a * delta.squared()) / 2f32;
        transform.translation += vec3(s.x, s.y, 0f32);
        particle.velocity += a * delta;
        resolve_colisions(&mut particle, &mut transform);
    }
}
fn calc_drag_force(velocity: Vec2, area: f32) -> Vec2 {
    // F = .5*d*v^2*C*A https://en.wikipedia.org/wiki/Drag_(physics)
    let speed_squared = velocity.length_squared();
    AIR_DENSITY * speed_squared * PARTICLE_DRAG_COEFICIENT * area / 2f32 * velocity.normalize()
}

#[derive(Component)]
pub(crate) struct Particle {
    pub ray: f32,
    pub mass: f32,
    pub velocity: Vec2,
    pub area: f32,
}
impl Particle {
    pub fn new(mass: f32, velocity: Vec2, ray: f32) -> Particle {
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
