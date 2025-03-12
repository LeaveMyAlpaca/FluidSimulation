use std::usize;

use bevy::math::{Vec2, vec2};

use crate::{
    bounding_box::BOX_BOUNDS_SIZE_PIXELS,
    particles_spawning::{self, PARTICLE_RAY},
    pressure_handler::SMOOTHING_DISTANCE,
};
pub fn split_particles_into_grid(particles: &[Vec2]) -> [Vec<usize>; TOTAL_GRID_SIZE] {
    // maybe change to 1d vec???
    let mut output: [Vec<usize>; TOTAL_GRID_SIZE] = [const { Vec::new() }; TOTAL_GRID_SIZE];

    for i in 0..particles_spawning::PARTICLES_COUNT as usize {
        // println!(" pos {}", particles[i]);
        let grid_index = pos_to_grid_index(&particles[i]);
        if grid_index == usize::MAX {
            continue;
        }
        output[grid_index].push(i);
    }

    output
}
//OK: (890 / 20 , 500 /20)  + (100 /2 ,  50 / 2) -> (95 , 50)
pub fn pixel_pos_to_gird_pos(pixel_pos: &Vec2) -> Vec2 {
    let raw = pixel_pos / SMOOTHING_DISTANCE as f32 + vec2(GRID_SIZE_X / 2f32, GRID_SIZE_Y / 2f32);
    vec2((raw.x as usize) as f32, (raw.y as usize) as f32)
}
pub fn pos_to_grid_index(pixel_pos: &Vec2) -> usize {
    grid_pos_to_index(&pixel_pos_to_gird_pos(pixel_pos))
}
// 2000 / 20 -> 100
pub const GRID_SIZE_X: f32 = (BOX_BOUNDS_SIZE_PIXELS.x as u32).div_ceil(SMOOTHING_DISTANCE) as f32;
// 50
pub const GRID_SIZE_Y: f32 = (BOX_BOUNDS_SIZE_PIXELS.y as u32).div_ceil(SMOOTHING_DISTANCE) as f32;
pub const TOTAL_GRID_SIZE: usize = (GRID_SIZE_X as usize) * (GRID_SIZE_Y as usize + 1) + 1;

pub fn grid_pos_to_index(grid_pos: &Vec2) -> usize {
    if grid_pos.x == -1f32 {
        return usize::MAX;
    }

    ((grid_pos.y) * GRID_SIZE_X + grid_pos.x) as usize
}
pub fn get_connected_cells(sample_grid_pos: &Vec2) -> Vec<Vec2> {
    let mut output = Vec::with_capacity(9);
    if sample_grid_pos.y < GRID_SIZE_Y {
        // i don't have to check if x == 0 because then x == -1 so we are good
        output.push(sample_grid_pos + vec2(-1f32, 1f32));

        output.push(sample_grid_pos + vec2(0f32, 1f32));
        if sample_grid_pos.x < GRID_SIZE_X {
            output.push(sample_grid_pos + vec2(1f32, 1f32));
        } else {
            output.push(vec2(-1f32, -1f32));
        }
    } else {
        output.push(vec2(-1f32, -1f32));
        output.push(vec2(-1f32, -1f32));
        output.push(vec2(-1f32, -1f32));
    }
    output.push(sample_grid_pos + vec2(-1f32, 0f32));
    output.push(sample_grid_pos + vec2(0f32, 0f32));
    if sample_grid_pos.x < GRID_SIZE_X {
        output.push(sample_grid_pos + vec2(1f32, 0f32));
    } else {
        output.push(vec2(-1f32, -1f32));
    }

    if sample_grid_pos.y > 0f32 {
        output.push(sample_grid_pos + vec2(-1f32, -1f32));
        output.push(sample_grid_pos + vec2(0f32, -1f32));
        if sample_grid_pos.x < GRID_SIZE_X {
            output.push(sample_grid_pos + vec2(1f32, -1f32));
        } else {
            output.push(vec2(-1f32, -1f32));
        }
    } else {
        output.push(vec2(-1f32, -1f32));
        output.push(vec2(-1f32, -1f32));
        output.push(vec2(-1f32, -1f32));
    }

    output
}
pub fn get_connected_cells_indexes(sample_grid_pos: &Vec2) -> Vec<usize> {
    let mut output: Vec<usize> = Vec::with_capacity(9);
    for pos in get_connected_cells(sample_grid_pos) {
        let index = grid_pos_to_index(&pos);

        output.push(index);
    }
    output
}
