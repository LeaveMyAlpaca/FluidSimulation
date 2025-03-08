use std::usize;

use bevy::math::{Vec2, vec2};

use crate::{
    bounding_box::BOX_BOUNDS_SIZE_PIXELS,
    particles_spawning,
    pressure_handler::{self, SMOOTHING_DISTANCE},
};

pub fn split_particles_into_grid(
    particles: &Vec<Vec2>,
) -> [Vec<usize>; particles_spawning::PARTICLES_TO_SPAWN as usize] {
    let mut output: [Vec<usize>; particles_spawning::PARTICLES_TO_SPAWN as usize] =
        [const { Vec::new() }; particles_spawning::PARTICLES_TO_SPAWN as usize];

    for i in 0..particles_spawning::PARTICLES_TO_SPAWN as usize {
        output[pos_to_grid_index(&particles[i])].push(i);
    }

    output
}
pub fn pixel_pos_to_gird_pos(pixel_pos: &Vec2) -> Vec2 {
    let raw = pixel_pos / SMOOTHING_DISTANCE;
    vec2(raw.x.ceil(), raw.y.ceil())
}
pub fn pos_to_grid_index(pixel_pos: &Vec2) -> usize {
    grid_pos_to_index(&pixel_pos_to_gird_pos(pixel_pos))
}

const GRID_SIZE_X: f32 =
    (BOX_BOUNDS_SIZE_PIXELS.x as u32).div_ceil(SMOOTHING_DISTANCE as u32) as f32;
const GRID_SIZE_Y: f32 =
    (BOX_BOUNDS_SIZE_PIXELS.y as u32).div_ceil(SMOOTHING_DISTANCE as u32) as f32;

pub fn grid_pos_to_index(grid_pos: &Vec2) -> usize {
    ((grid_pos.y + GRID_SIZE_Y / 2f32) * GRID_SIZE_X + grid_pos.x + GRID_SIZE_X / 2f32) as usize
}
pub fn get_connected_cells(sample_grid_pos: &Vec2) -> Vec<Vec2> {
    vec![
        //###
        //#o#
        //###
        sample_grid_pos + vec2(-1f32, 1f32),
        sample_grid_pos + vec2(0f32, 1f32),
        sample_grid_pos + vec2(1f32, 1f32),
        sample_grid_pos + vec2(-1f32, 0f32),
        sample_grid_pos.to_owned(),
        sample_grid_pos + vec2(1f32, 0f32),
        sample_grid_pos + vec2(-1f32, -1f32),
        sample_grid_pos + vec2(0f32, -1f32),
        sample_grid_pos + vec2(1f32, -1f32),
    ]
}
pub fn get_connected_cells_indexes(sample_grid_pos: &Vec2) -> Vec<usize> {
    let mut output: Vec<usize> = Vec::with_capacity(9);
    for pos in get_connected_cells(sample_grid_pos) {
        output.push(grid_pos_to_index(&pos));
    }
    output
}
