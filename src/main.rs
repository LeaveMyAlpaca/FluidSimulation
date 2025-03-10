mod bounding_box;
#[path = "physics/collisions.rs"]
mod collisions;
mod particle_grid;
#[path = "physics/particle_physics.rs"]
mod particle_physics;
mod particles_spawning;
#[path = "physics/pressure_handler.rs"]
mod pressure_handler;
mod ui_handler;

use bevy::{
    color::palettes::css::{BLUE, BROWN, GREEN, RED, YELLOW},
    math::vec2,
    prelude::*,
    text::cosmic_text::Color,
};
use particle_grid::split_particles_into_grid;
use particle_physics::Particle;
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
    gizmos.circle_2d(pointer_isometry, SMOOTHING_DISTANCE, BLUE);

    if DEBUG_CHECKED_PARTICLES {
        let mut particle_points = Vec::with_capacity(particles_spawning::PARTICLES_COUNT as usize);
        for (transform, _) in &particles {
            particle_points.push(transform.translation.xy());
        }
        let grid = particle_grid::split_particles_into_grid(&particle_points);

        let connected_indexes = particle_grid::get_connected_cells_indexes(
            &particle_grid::pixel_pos_to_gird_pos(&pointer.pos),
        );
        for cell_index in connected_indexes {
            for particle_index in &grid[cell_index] {
                let pointer_isometry = Isometry2d::new(
                    particle_points[particle_index.to_owned()],
                    Rot2::degrees(0f32),
                );
                gizmos.circle_2d(pointer_isometry, 5f32, GREEN);
            }
        }
    }
}

#[derive(Component)]
struct DebugPointer {
    pos: Vec2,
}
