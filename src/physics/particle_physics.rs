use crate::{
    collisions::resolve_collisions,
    particle_grid,
    particles_spawning::{self, PARTICLES_COUNT},
    player_interaction_physics,
    pressure_handler::{self, calculate_pressure_force},
    viscosity_force::calculate_viscosity_force,
};
use bevy::{math::*, prelude::*, window::PrimaryWindow};

// physics settings
const GRAVITY: Vec2 = Vec2::new(0f32, -15f32);
const TIME_SCALE: f32 = 2f32;
const AIR_DENSITY: f32 = 1f32;
const PARTICLE_DRAG_COEFFICIENT: f32 = 0.01f32;
const PRESSURE_FORCE_MODIFIER: f32 = 0.6f32;

const DEBUG_USE_PRESSURE: bool = true;
const RUN_PHYSICS: bool = true;
pub fn handle_particles_physics(
    mut particles: Query<(&mut Transform, &mut Particle)>,
    time: Res<Time>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    if !RUN_PHYSICS {
        return;
    }
    let delta = time.delta().as_secs_f32() * TIME_SCALE;
    particles
        .par_iter_mut()
        .for_each(|(transform, mut particle)| {
            let a = GRAVITY;
            particle.velocity += a * delta;
            particle.predicted_position = transform.translation.xy() + particle.velocity / 120f32;
        });

    let mut particle_predicted_positions =
        Vec::with_capacity(particles_spawning::PARTICLES_COUNT as usize);
    for (_, particle) in &particles {
        particle_predicted_positions.push(particle.predicted_position);
    }

    let connected_cells =
        particle_grid::calculate_connected_cells_for_every_particle(&particle_predicted_positions);

    let grid = particle_grid::split_particles_into_grid(&particle_predicted_positions);

    let densities = &pressure_handler::calculate_density_for_every_particle(
        &grid,
        &particle_predicted_positions,
        &connected_cells,
    );

    // interactions
    let mut use_interaction: bool = true;
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    let cursor_option = window.cursor_position();
    let mouse_position = match cursor_option {
        Some(pos) => camera
            .viewport_to_world(camera_transform, pos)
            .map(|ray| ray.origin.truncate())
            .unwrap(),
        None => {
            use_interaction = false;
            Vec2::ZERO
        }
    };
    let force_sign;
    if mouse_buttons.pressed(MouseButton::Right) {
        force_sign = -1f32;
    } else if mouse_buttons.pressed(MouseButton::Left) {
        // disabled because not working good enough
        use_interaction = false;
        force_sign = 0.5f32;
    } else {
        use_interaction = false;
        force_sign = 0f32;
    };

    particles.par_iter_mut().for_each(|(_, mut particle)| {
        let pressure_force: Vec2 = if DEBUG_USE_PRESSURE {
            -calculate_pressure_force(
                particle.index,
                connected_cells
                    .get(particle.index * 9..(particle.index + 1) * 9)
                    .unwrap(),
                &particle_predicted_positions,
                &grid,
                densities,
            )
        } else {
            Vec2::ZERO
        };

        let interaction_force = match use_interaction {
            true => player_interaction_physics::calculate_interaction_force(
                particle.predicted_position,
                mouse_position,
                force_sign,
                particle.velocity,
            ),
            false => Vec2::ZERO,
        };

        let force = pressure_force * PRESSURE_FORCE_MODIFIER
            - calc_drag_force(particle.velocity, particle.area)
            + interaction_force;
        let acceleration = force / particle.mass;
        particle.velocity += acceleration * delta;
    });
    let mut velocities: Vec<Vec2> = Vec::with_capacity(PARTICLES_COUNT as usize);
    // get velocities for viscosity
    particles.iter().for_each(|(_, particle)| {
        velocities.push(particle.velocity);
    });

    // apply viscosity
    particles.par_iter_mut().for_each(|(_, mut particle)| {
        let viscosity = calculate_viscosity_force(
            particle.predicted_position,
            particle.velocity,
            &particle_predicted_positions,
            // could add that as a separate property to particle so i don't have to get it twice
            // but that's a pretty cheap operation
            connected_cells
                .get(&particle.index * 9..(&particle.index + 1) * 9)
                .unwrap(),
            &grid,
            &velocities,
        );
        particle.velocity += viscosity;
    });

    particles
        .par_iter_mut()
        .for_each(|(mut transform, mut particle)| {
            if particle.velocity.is_nan() {
                particle.velocity = particle.last_velocity;
            }
            particle.last_velocity = particle.velocity;

            let s = particle.velocity * delta;

            transform.translation += vec3(s.x, s.y, 0f32);
            resolve_collisions(&mut particle, &mut transform);
            // just for visualization purposes
            particle.density = densities[particle.index];
        });
}

fn calc_drag_force(velocity: Vec2, area: f32) -> Vec2 {
    // F = .5*d*v^2*C*A https://en.wikipedia.org/wiki/Drag_(physics)
    let speed_squared = velocity.length_squared();
    AIR_DENSITY * speed_squared * PARTICLE_DRAG_COEFFICIENT * area / 2f32 * velocity.normalize()
}

#[derive(Component)]
pub(crate) struct Particle {
    pub ray: f32,
    pub mass: f32,
    pub velocity: Vec2,
    pub last_velocity: Vec2,
    pub area: f32,
    pub index: usize,
    pub predicted_position: Vec2,
    // used for visuals
    pub density: f32,
}
impl Particle {
    pub fn new(mass: f32, velocity: Vec2, ray: f32, index: usize) -> Particle {
        Particle {
            ray,
            mass,
            velocity,
            last_velocity: Vec2::ZERO,
            area: Particle::calc_area(ray),
            index,
            predicted_position: Vec2::ZERO,
            density: 0f32,
        }
    }
    fn calc_area(ray: f32) -> f32 {
        core::f32::consts::PI * ray.squared() * particles_spawning::PARTICLE_RESOLUTION
    }
}
