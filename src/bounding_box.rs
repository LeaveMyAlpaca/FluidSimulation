use bevy::{color::palettes::css::GREY, math::*, prelude::*, sprite::Sprite};

pub const BOX_BOUNDS_SIZE_PIXELS: Vec2 = Vec2::new(1700f32, 1000f32);
const BOX_SPRITE_PATH: &str = "sprites/box.png";
pub const BOX_SPRITE_RESOLUTION: Vec2 = Vec2::new(50f32, 50f32);

pub fn spawn_bounding_box(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut sprite = Sprite::from_image(asset_server.load(BOX_SPRITE_PATH));
    sprite.color = Color::Srgba(GREY);
    let scale = BOX_BOUNDS_SIZE_PIXELS / BOX_SPRITE_RESOLUTION;

    // Z works like layer so -1000 to make it stay in the background
    let transform = Transform {
        translation: Vec3::new(0f32, 0f32, -1000f32),
        scale: Vec3::new(scale.x, scale.y, 1f32),
        ..default()
    };

    commands.spawn((transform, sprite));
}
