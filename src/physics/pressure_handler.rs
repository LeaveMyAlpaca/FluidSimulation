use crate::{
    particle_grid::{self, pixel_pos_to_gird_pos},
    particles_spawning::{self, PARTICLES_TO_SPAWN},
};
use bevy::{math::Vec2, prelude::*, tasks::ParallelSlice};
use std::f32::consts::PI;

// can't use SMOOTHING_DISTANCE.powi(4) so just multiply 4 times
const SMOOTHING_KERNEL_DERIVATIVE_SCALE: f32 = 12f32
    / (PI * SMOOTHING_DISTANCE * SMOOTHING_DISTANCE * SMOOTHING_DISTANCE * SMOOTHING_DISTANCE);

fn smoothing_kernel_derivative(distance: f32) -> f32 {
    if distance >= SMOOTHING_DISTANCE {
        return 0f32;
    }
    (distance - SMOOTHING_DISTANCE) * SMOOTHING_KERNEL_DERIVATIVE_SCALE
}

// can't use SMOOTHING_DISTANCE.powi(4) so just multiply 4 times
const SMOOTHING_KERNEL_VOLUME: f32 =
    (PI * SMOOTHING_DISTANCE * SMOOTHING_DISTANCE * SMOOTHING_DISTANCE * SMOOTHING_DISTANCE) / 6f32;
fn smoothing_kernel(distance: f32) -> f32 {
    if distance >= SMOOTHING_DISTANCE {
        return 0f32;
    }

    (SMOOTHING_DISTANCE - distance) * (SMOOTHING_DISTANCE - distance) / SMOOTHING_KERNEL_VOLUME
}

pub fn calculate_density_for_every_particle(
    particles_gird: &[Vec<usize>; particles_spawning::PARTICLES_TO_SPAWN as usize],
    particles_pos: &[Vec2],
    connected_cells: &[Vec<usize>; particles_spawning::PARTICLES_TO_SPAWN as usize],
) -> Vec<f32> {
    let input = vec![0f32; particles_spawning::PARTICLES_TO_SPAWN as usize];
    let data_chuncks = input.par_splat_map(bevy::tasks::ComputeTaskPool::get(), None, |i, data| {
        // `i` is the starting index of the current chunk
        let mut output_chunck = Vec::new();

        for internal_index in 0..data.len() {
            let real_particle_index = internal_index + i;
            output_chunck.push(sample_density(
                &particles_pos[real_particle_index],
                &connected_cells[real_particle_index],
                particles_gird,
                particles_pos,
            ));
        }
        output_chunck
    });
    let mut output = Vec::with_capacity(PARTICLES_TO_SPAWN as usize);
    for chunck in data_chuncks {
        for density in chunck {
            output.push(density);
        }
    }
    output
}
pub fn calculate_pressure_force(
    sample_particel_index: usize,
    sample_connected_cells: &Vec<usize>,
    particles_pos: &[Vec2],
    particle_grid: &[Vec<usize>; particles_spawning::PARTICLES_TO_SPAWN as usize],
    densities: &[f32],
) -> Vec2 {
    let sample_point = particles_pos[sample_particel_index];
    let mut pressure: Vec2 = Vec2::ZERO;
    for cell in sample_connected_cells {
        for particle_index_ref in &particle_grid[cell.to_owned()] {
            let particle_index = particle_index_ref.to_owned();
            let pos = particles_pos[particle_index];
            if particle_index == sample_particel_index || sample_point == pos {
                continue;
            }

            let dist = pos.distance(sample_point);
            let dir = (pos - sample_point) / dist;
            let slope = smoothing_kernel_derivative(dist);
            let shared_pressure = calculate_shared_pressure(
                densities[particle_index],
                densities[sample_particel_index],
            );
            pressure -=
                shared_pressure * dir * slope * INFLUENCE_MODIFIER / densities[particle_index];
        }
    }
    pressure
}
fn calculate_shared_pressure(density_a: f32, density_b: f32) -> f32 {
    let pressure_a = density_to_pressure(density_a);
    let pressure_b = density_to_pressure(density_b);
    (pressure_a + pressure_b) / 2f32
}
const TARGET_DENSITY: f32 = 0.0001f32;
const PRESSURE_MULTIPLIER: f32 = 1000000.0f32;
fn density_to_pressure(density: f32) -> f32 {
    let density_error = density - TARGET_DENSITY;
    density_error * PRESSURE_MULTIPLIER
}

fn get_influence(a: &Vec2, b: &Vec2) -> f32 {
    smoothing_kernel(a.distance(b.xy()))
}

pub const SMOOTHING_DISTANCE: f32 = 100f32;
const INFLUENCE_MODIFIER: f32 = 10f32;
pub fn sample_density(
    sample_particle_pos: &Vec2,
    sample_connected_cells: &Vec<usize>,
    particle_grid: &[Vec<usize>; particles_spawning::PARTICLES_TO_SPAWN as usize],
    particles: &[Vec2],
) -> f32 {
    let mut density: f32 = 0f32;
    for cell in sample_connected_cells {
        for particle_index in &particle_grid[cell.to_owned()] {
            let influence =
                get_influence(sample_particle_pos, &particles[particle_index.to_owned()]);
            density += influence * INFLUENCE_MODIFIER;
        }
    }

    density
}
