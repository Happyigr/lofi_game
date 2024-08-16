use bevy::{color::Color, math::Vec3, prelude::KeyCode};

pub const EXPLOSION_SPRITESHEET: &str = "explosion_4.png";

pub const PLAYER_SIZE: f32 = 32.0;
pub const PLAYER_SPEED: f32 = 200.0;
pub const PLAYER_SPAWN_POS: Vec3 = Vec3::ZERO;
pub const CATCH_RAD: f32 = 100.;
pub const MAP_H: f32 = 400.;
pub const MAP_W: f32 = 700.;

pub const TEXT_Z: f32 = 10.;

pub const PLAYER_UP: KeyCode = KeyCode::ArrowUp;
pub const PLAYER_DOWN: KeyCode = KeyCode::ArrowDown;
pub const PLAYER_LEFT: KeyCode = KeyCode::ArrowLeft;
pub const PLAYER_RIGHT: KeyCode = KeyCode::ArrowRight;

pub const BOOST_MULTIPLIER_DELTA: f32 = 0.5;
pub const CATCHRAD_MULTIPLIER_DELTA: f32 = 0.5;

pub const BOOST_ACTIVATOR: KeyCode = KeyCode::Digit1;
pub const JUMP_ACTIVATOR: KeyCode = KeyCode::Digit2;
pub const CATCHRAD_ACTIVATOR: KeyCode = KeyCode::Digit3;
pub const BOOM_ACTIVATOR: KeyCode = KeyCode::Digit4;
pub const POOP_ACTIVATOR: KeyCode = KeyCode::Digit5;

pub const ENEMY_SIZE: f32 = 22.0;
pub const ENEMY_COLOR_BOOST: Color = Color::linear_rgba(0.5, 0., 0., 1.);
pub const ENEMY_COLOR_JUMP: Color = Color::linear_rgba(0., 0.5, 0., 1.);
pub const ENEMY_COLOR_CATCHRAD: Color = Color::linear_rgba(0., 0., 0.5, 1.);
pub const ENEMY_COLOR_BOOM: Color = Color::linear_rgba(0.2, 0.2, 0.2, 1.);
pub const ENEMY_COLOR_POOP: Color = Color::linear_rgba(0.8, 0.8, 0.4, 1.);
