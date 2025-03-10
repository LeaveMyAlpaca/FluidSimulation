use crate::{
    collisions::resolve_colisions,
    particle_grid,
    particles_spawning::{self, PARTICLES_COUNT},
    pressure_handler::{self, calculate_pressure_force},
};
use bevy::{math::*, prelude::*};

// physics settings
const GRAVITY: Vec2 = Vec2::new(0f32, 0f32);
const TIME_SCALE: f32 = 2f32;
const AIR_DENSITY: f32 = 1f32;
const PARTICLE_DRAG_COEFICIENT: f32 = 0.001f32;
const PRESSURE_FORCE_MODIFIER: f32 = 5f32;

const DEBUG_USE_PRESSURE: bool = true;
const DEBUG_RUN_PARTICLE_PHYSICS: bool = true;
const DEBUG_SHOW_PARTICLE_DENSITY: bool = false;
const DEBUG_SHOW_PARTICLE_PRESSURE: bool = false;
const DEBUG_SHOW_DISTANCE_CHECK: bool = false;

pub fn handle_particles_physics(
    mut particles: Query<(&mut Transform, &mut Particle), With<Particle>>,
    time: Res<Time>,
) {
    // array of vectors for particles that can be indexed by particle index to aces connected cells
    // so i don't have to calculate them multiple times
    let mut connected_cells: Vec<usize> = Vec::with_capacity((PARTICLES_COUNT * 9) as usize);
    // TODO: test if parallel could work
    let mut i = 0;
    particles.iter().for_each(|(transform, _)| {
        let cells = particle_grid::get_connected_cells_indexes(
            &particle_grid::pixel_pos_to_gird_pos(&transform.translation.xy()),
        );
        for cell in cells {
            connected_cells.push(cell);
        }
        i += 1;
    });
    let mut particle_points = Vec::with_capacity(particles_spawning::PARTICLES_COUNT as usize);
    for (transform, _) in &particles {
        particle_points.push(transform.translation.xy());
    }
    let grid = particle_grid::split_particles_into_grid(&particle_points);

    let densities = &pressure_handler::calculate_density_for_every_particle(
        &grid,
        &particle_points,
        &connected_cells,
    );

    let delta = time.delta().as_secs_f32() * TIME_SCALE;
    particles
        .par_iter_mut()
        .for_each(|(mut transform, mut particle)| {
            let pressure_forece: Vec2 = if DEBUG_USE_PRESSURE {
                -calculate_pressure_force(
                    particle.index,
                    connected_cells
                        .get(particle.index * 9..(particle.index + 1) * 9)
                        .unwrap(),
                    &particle_points,
                    &grid,
                    densities,
                )
            } else {
                Vec2::ZERO
            };

            // this is not great because we take velocity for drag calculation from last frame so the more frames we have
            // the more accurate the calculations will be, this is OK for our purpose
            //WARN: because of that our calculations could be UN deterministic ?
            let f = pressure_forece * PRESSURE_FORCE_MODIFIER
                - calc_drag_force(particle.velocity, particle.area);

            let a = GRAVITY + f / particle.mass;

            // s = vt + (at^2)/2
            let s = particle.velocity * delta + (a * delta.squared()) / 2f32;
            transform.translation += vec3(s.x, s.y, 0f32);
            particle.velocity += a * delta;
            resolve_colisions(&mut particle, &mut transform);
        });
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
    pub index: usize,
}
impl Particle {
    pub fn new(mass: f32, velocity: Vec2, ray: f32, index: usize) -> Particle {
        Particle {
            ray,
            mass,
            velocity,
            area: Particle::calc_area(ray),
            index,
        }
    }
    fn calc_area(ray: f32) -> f32 {
        core::f32::consts::PI * ray.squared() * particles_spawning::PARTICLE_RESOULTION
    }
}
