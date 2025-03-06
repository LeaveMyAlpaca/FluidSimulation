use std::f32::consts::PI;

use bevy::{
    math::{FloatPow, NormedVectorSpace, Vec2},
    prelude::*,
    scene::ron::value::Float,
};

use crate::particlePhysics::Particle;

fn smoothing_kernel(radious: f32, distanceSquared: f32) -> f32 {
    let value = (radious.squared() - distanceSquared).max(0f32);

    // could replace that with constant modifier witch would be faster but then i would have to adjust it every time i change smoothing radius
    let volume = PI * radious.powi(8) / 4f32;
    value * value * value / volume
}

const SMOOTHING_DISTANCE: f32 = 10f32;
const INFLUENCE_MODIFIER: f32 = 1f32;
pub fn sample_density(point: Vec2, transforms: Query<&Transform, With<Particle>>) -> f32 {
    let mut density: f32 = 0f32;

    for transform in transforms.iter() {
        let influence = smoothing_kernel(
            SMOOTHING_DISTANCE,
            point.distance_squared(transform.translation.xy()),
        );
        density += influence * INFLUENCE_MODIFIER;
    }

    density
}
