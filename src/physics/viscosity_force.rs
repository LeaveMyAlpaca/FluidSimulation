use bevy::prelude::*;
use ops::FloatPow;

use crate::{particle_grid::TOTAL_GRID_SIZE, pressure_handler::SMOOTHING_DISTANCE};
fn viscosity_smoothing(distance: f32) -> f32 {
    let value: f32 = 0f32.max((SMOOTHING_DISTANCE as f32).squared() - distance.squared());
    value * value * value
}
const VISCOSITY_STRENGTH: f32 = 0.000000001f32;
pub fn calculate_viscosity_force(
    sample_point: Vec2,
    sample_velocity: Vec2,
    particles_pos: &[Vec2],
    connected_cells: &[usize],
    particles_gird: &[Vec<usize>; TOTAL_GRID_SIZE],
    velocities: &[Vec2],
) -> Vec2 {
    let mut viscosity_force = Vec2::ZERO;
    for cell in connected_cells {
        if cell == &usize::MAX {
            continue;
        }
        for index_ref in &particles_gird[cell.to_owned()] {
            let particle_index = index_ref.to_owned();
            let distance = particles_pos[particle_index].distance(sample_point);
            let influence = viscosity_smoothing(distance);

            viscosity_force += (velocities[particle_index] - sample_velocity) * influence;
        }
    }
    // println!("viscosity_force {}", viscosity_force);
    viscosity_force * VISCOSITY_STRENGTH
}
