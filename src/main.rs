use core::f32;

use bevy::{
    ecs::entity,
    math::{FloatPow, vec3},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, handle_input))
        .add_systems(Update, (handle_input, move_particles))
        .run();
}
const CIRCLE_SPRITE_PATH: &str = "sprites/circle.png";
const PARTICLE_RAY: f32 = 0.4f32;
const GRAVITY: Vec3 = Vec3::new(0f32, -10f32, 0f32);
const TIME_SCALE: f32 = 2f32;

const AIR_DENSITY: f32 = 1f32;
const PARTICLE_DRAG_COEFICIENT: f32 = 0.01f32;
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    spawn_particle(1f32, commands, asset_server);
}

fn spawn_particle(mass: f32, mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite = Sprite::from_image(asset_server.load(CIRCLE_SPRITE_PATH));

    let transform = Transform {
        translation: Vec3::new(0f32, 300f32, 0f32),
        scale: Vec3::new(PARTICLE_RAY, PARTICLE_RAY, PARTICLE_RAY),
        ..default()
    };
    let particle = Particle::new(mass, vec3(20f32, 0f32, 0f32), PARTICLE_RAY);

    commands.spawn((particle, transform, sprite));
}

fn move_particles(mut particles: Query<(&mut Transform, &mut Particle)>, time: Res<Time>) {
    let delta = time.delta().as_secs_f32() * TIME_SCALE;
    for (mut transform, mut particle) in &mut particles {
        // this is not great because we take velocity from last frame so the more frames we have
        // the more accurate the calculations will be, this is OK for our purpose
        let a = GRAVITY - calc_drag_force(particle.velocity, particle.area) / particle.mass;

        // s = vt + (at^2)/2
        let s = particle.velocity * delta + (a * delta.squared()) / 2.;
        transform.translation += s;
        particle.velocity += a * delta;
    }
}
fn calc_drag_force(velocity: Vec3, area: f32) -> Vec3 {
    // F = .5*d*v^2*C*A https://en.wikipedia.org/wiki/Drag_(physics)
    return AIR_DENSITY * velocity * velocity * PARTICLE_DRAG_COEFICIENT * area / 2f32;
}

fn handle_input(keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        println!("Space");
    }
}
#[derive(Component)]
struct Particle {
    pub ray: f32,
    pub mass: f32,
    pub velocity: Vec3,
    pub area: f32,
}
impl Particle {
    pub fn new(mass: f32, velocity: Vec3, ray: f32) -> Particle {
        return Particle {
            ray,
            mass,
            velocity,
            area: Particle::calc_area(ray),
        };
    }
    fn calc_area(ray: f32) -> f32 {
        return f32::consts::PI * ray.squared();
    }
}
