use bevy::prelude::*;

const INTERACTION_STRENGTH: f32 = 90000000f32;
const MAX_INTERACTION_DIST_SQRT: f32 = 900000f32;
pub fn calculate_interaction_force(
    pos: Vec2,
    mouse_pos: Vec2,
    force_sign: f32,
    velocity: Vec2,
) -> Vec2 {
    let dist = mouse_pos.distance_squared(pos);
    if dist > MAX_INTERACTION_DIST_SQRT || dist == 0f32 {
        return Vec2::ONE;
    }

    let dir = (mouse_pos - pos) / dist;

    let strength = (INTERACTION_STRENGTH * force_sign - velocity) / dist.max(80f32);

    strength * dir
}
