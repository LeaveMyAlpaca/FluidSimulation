mod bounding_box;
#[path = "physics/collisions.rs"]
mod collisions;
mod particle_grid;
#[path = "physics/particle_physics.rs"]
mod particle_physics;
mod particles_spawning;
mod particles_visuals;
#[path = "physics/player_interation_physics.rs"]
mod player_interation_physics;
#[path = "physics/pressure_handler.rs"]
mod pressure_handler;
mod ui_handler;
#[path = "physics/viscosity_force.rs"]
mod viscosity_force;

use bevy::{
    color::palettes::css::{BLUE, GREEN, RED},
    math::vec2,
    prelude::*,
};
use bounding_box::BOX_BOUNDS_SIZE_PIXELS;
use particle_grid::{GRID_SIZE_X, GRID_SIZE_Y, TOTAL_GRID_SIZE, pos_to_grid_index};
use particle_physics::Particle;
use particles_spawning::PARTICLE_RAY;
use pressure_handler::SMOOTHING_DISTANCE;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, bounding_box::spawn_bounding_box))
        .add_systems(
            Update,
            (
                particle_physics::handle_particles_physics,
                ui_handler::update_ui,
                debug_input_update,
                particles_visuals::update_particles_visuals,
            ),
        )
        .run();
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(DebugPointer {
        pos: Vec2::new(1f32, 1f32),
    });

    ui_handler::setup_ui(&mut commands);
    particles_spawning::handle_spawning_particles(&mut commands, &asset_server);
}
const USE_DEBUG_POINTER: bool = false;
const DEBUG_POINTER_MOVEMENT_SPEED: f32 = 30f32;
const DEBUG_CHECKED_PARTICLES: bool = true;

fn debug_input_update(
    mut pointer_query: Query<&mut DebugPointer>,
    particles: Query<(&mut Transform, &mut Particle), With<Particle>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut gizmos: Gizmos,
) {
    if !USE_DEBUG_POINTER {
        return;
    }

    let mut pointer = pointer_query.single_mut();
    let mut movement: Vec2 = Vec2::ZERO;
    if keys.just_pressed(KeyCode::KeyJ) {
        movement += vec2(0f32, -1f32);
    }
    if keys.just_pressed(KeyCode::KeyK) {
        movement += vec2(0f32, 1f32);
    }
    if keys.just_pressed(KeyCode::KeyH) {
        movement += vec2(-1f32, 0f32);
    }
    if keys.just_pressed(KeyCode::KeyL) {
        movement += vec2(1f32, 0f32);
    }
    pointer.pos += movement * DEBUG_POINTER_MOVEMENT_SPEED;

    let pointer_isometry = Isometry2d::new(pointer.pos, Rot2::degrees(0f32));
    gizmos.circle_2d(pointer_isometry, 10f32, RED);
    gizmos.circle_2d(pointer_isometry, SMOOTHING_DISTANCE as f32, BLUE);

    if DEBUG_CHECKED_PARTICLES {
        let mut particle_predicted_positions =
            Vec::with_capacity(particles_spawning::PARTICLES_COUNT as usize);
        for (_, particle) in &particles {
            particle_predicted_positions.push(particle.predicted_position);
        }
        let grid = particle_grid::split_particles_into_grid(&particle_predicted_positions);
        let connected_pos =
            particle_grid::get_connected_cells(&particle_grid::pixel_pos_to_gird_pos(&pointer.pos));

        for cell_pos in connected_pos {
            let pixel_pos = (cell_pos - vec2(GRID_SIZE_X / 2f32, GRID_SIZE_Y / 2f32))
                * SMOOTHING_DISTANCE as f32;
            let iso = Isometry2d::new(pixel_pos, Rot2::degrees(0f32));
            gizmos.rect_2d(
                iso,
                vec2(SMOOTHING_DISTANCE as f32, SMOOTHING_DISTANCE as f32),
                GREEN,
            );
        }
        let connected_indexes = particle_grid::get_connected_cells_indexes(
            &particle_grid::pixel_pos_to_gird_pos(&pointer.pos),
        );
        // println!(
        //     "&particle_grid::pixel_pos_to_gird_pos(&pointer.pos) {}",
        //     &particle_grid::pixel_pos_to_gird_pos(&pointer.pos)
        // );
        for cell_index in connected_indexes {
            if cell_index == usize::MAX {
                continue;
            }

            for particle_index in &grid[cell_index] {
                let pointer_isometry = Isometry2d::new(
                    particle_predicted_positions[particle_index.to_owned()],
                    Rot2::degrees(0f32),
                );
                gizmos.circle_2d(pointer_isometry, 1f32, RED);
            }
        }
    }
}

#[derive(Component)]
struct DebugPointer {
    pos: Vec2,
}
