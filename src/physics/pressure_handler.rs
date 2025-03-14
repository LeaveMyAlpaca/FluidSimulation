use crate::{
    particle_grid::TOTAL_GRID_SIZE,
    particles_spawning::{self, PARTICLES_COUNT},
};
use bevy::{math::Vec2, prelude::*, tasks::ParallelSlice};
use std::f32::consts::PI;

// can't use SMOOTHING_DISTANCE.powi(4) so just multiply 4 times
const SMOOTHING_KERNEL_DERIVATIVE_SCALE: f32 = 12f32
    / (PI
        * (SMOOTHING_DISTANCE * SMOOTHING_DISTANCE * SMOOTHING_DISTANCE * SMOOTHING_DISTANCE)
            as f32);

fn smoothing_kernel_derivative(distance: f32) -> f32 {
    if distance >= SMOOTHING_DISTANCE as f32 {
        return 0f32;
    }
    (distance - SMOOTHING_DISTANCE as f32) * SMOOTHING_KERNEL_DERIVATIVE_SCALE
}

// can't use SMOOTHING_DISTANCE.powi(4) so just multiply 4 times
const SMOOTHING_KERNEL_VOLUME: f32 = (PI
    * (SMOOTHING_DISTANCE * SMOOTHING_DISTANCE * SMOOTHING_DISTANCE * SMOOTHING_DISTANCE) as f32)
    / 6f32;
fn smoothing_kernel(distance: f32) -> f32 {
    if distance >= SMOOTHING_DISTANCE as f32 {
        return 0f32;
    }

    (SMOOTHING_DISTANCE as f32 - distance) * (SMOOTHING_DISTANCE as f32 - distance)
        / SMOOTHING_KERNEL_VOLUME
}

pub fn calculate_density_for_every_particle(
    particles_gird: &[Vec<usize>; TOTAL_GRID_SIZE],
    particles_pos: &[Vec2],
    connected_cells: &[usize],
) -> Vec<f32> {
    let input = vec![0f32; particles_spawning::PARTICLES_COUNT as usize];
    let data_chunks = input.par_splat_map(bevy::tasks::ComputeTaskPool::get(), None, |i, data| {
        // `i` is the starting index of the current chunk
        let mut output_chunk = Vec::new();

        for internal_index in 0..data.len() {
            let real_particle_index = internal_index + i;
            output_chunk.push(sample_density(
                &particles_pos[real_particle_index],
                connected_cells
                    .get(real_particle_index * 9..(real_particle_index + 1) * 9)
                    .unwrap(),
                particles_gird,
                particles_pos,
            ));
        }
        output_chunk
    });
    let mut output = Vec::with_capacity(PARTICLES_COUNT as usize);
    for chunk in data_chunks {
        for density in chunk {
            output.push(density);
        }
    }
    output
}
pub fn calculate_pressure_force(
    sample_particle_index: usize,
    sample_connected_cells: &[usize],
    particles_pos: &[Vec2],
    particle_grid: &[Vec<usize>; TOTAL_GRID_SIZE],
    densities: &[f32],
) -> Vec2 {
    let sample_point = particles_pos[sample_particle_index];
    let mut pressure: Vec2 = Vec2::ZERO;
    for cell in sample_connected_cells {
        if cell == &usize::MAX {
            continue;
        }
        for particle_index_ref in &particle_grid[cell.to_owned()] {
            let particle_index = particle_index_ref.to_owned();
            let pos = particles_pos[particle_index];
            if particle_index == sample_particle_index || sample_point == pos {
                continue;
            }

            let dist = pos.distance(sample_point);
            let dir = (pos - sample_point) / dist;
            let slope = smoothing_kernel_derivative(dist);
            let shared_pressure = calculate_shared_pressure(
                densities[particle_index],
                densities[sample_particle_index],
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
pub const TARGET_DENSITY: f32 = 0.15f32;
const PRESSURE_MULTIPLIER: f32 = 100000.0f32;
fn density_to_pressure(density: f32) -> f32 {
    let density_error = density - TARGET_DENSITY;
    density_error * PRESSURE_MULTIPLIER
}

fn get_influence(a: &Vec2, b: &Vec2) -> f32 {
    smoothing_kernel(a.distance(b.xy()))
}

pub const SMOOTHING_DISTANCE: u32 = 10;
const INFLUENCE_MODIFIER: f32 = 10f32;
pub fn sample_density(
    sample_particle_pos: &Vec2,
    sample_connected_cells: &[usize],
    particle_grid: &[Vec<usize>; TOTAL_GRID_SIZE],
    particles: &[Vec2],
) -> f32 {
    let mut density: f32 = 0f32;
    for cell in sample_connected_cells {
        if cell == &usize::MAX {
            continue;
        }
        for particle_index in &particle_grid[cell.to_owned()] {
            let influence =
                get_influence(sample_particle_pos, &particles[particle_index.to_owned()]);
            density += influence * INFLUENCE_MODIFIER;
        }
    }

    density
}
