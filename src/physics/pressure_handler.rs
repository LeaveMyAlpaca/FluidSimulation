use crate::particles_spawning;
use bevy::{math::Vec2, prelude::*};
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

pub fn calculate_density_for_every_particle(particles_pos: &Vec<Vec2>) -> Vec<f32> {
    let mut output = Vec::with_capacity(particles_spawning::PARTICLES_TO_SPAWN as usize);

    for i in 0..particles_spawning::PARTICLES_TO_SPAWN as usize {
        output.push(sample_density(i, particles_pos));
    }
    output
}
pub fn calculate_pressure_force(
    sample_particel_index: usize,
    particles_pos: &Vec<Vec2>,
    densities: &Vec<f32>,
) -> Vec2 {
    let sample_point = particles_pos[sample_particel_index];
    let mut pressure: Vec2 = Vec2::ZERO;

    for i in 0..particles_spawning::PARTICLES_TO_SPAWN as usize {
        let pos = particles_pos[i];
        if i == sample_particel_index || sample_point == pos {
            continue;
        }

        let dist = pos.distance(sample_point);
        let dir = (pos - sample_point) / dist;
        let slope = smoothing_kernel_derivative(dist);
        let shared_pressure =
            calculate_shared_pressure(densities[i], densities[sample_particel_index]);
        pressure -= shared_pressure * dir * slope * INFLUENCE_MODIFIER / densities[i];
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

const SMOOTHING_DISTANCE: f32 = 100f32;
const INFLUENCE_MODIFIER: f32 = 10f32;
pub fn sample_density(sample_particle_index: usize, particles_pos: &Vec<Vec2>) -> f32 {
    let mut density: f32 = 0f32;
    let sample_point = particles_pos[sample_particle_index];
    for pos in particles_pos {
        let influence = get_influence(&sample_point, pos);
        density += influence * INFLUENCE_MODIFIER;
    }

    density
}
