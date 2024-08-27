use bevy::{color::Color, math::Vec3, prelude::KeyCode};

pub const EXPLOSION_SPRITESHEET: &str = "animations/explosion_4.png";
pub const BG_MUSIC: &str = "sounds/rain.ogg";
pub const BOOM_SOUND: &str = "sounds/synt_explosion.ogg";
pub const HEARTBEAT_SOUND: &str = "sounds/heartbeat.ogg";
pub const MENU_ICON: &str = "icon.png";

pub const FONT: &str = "fonts/Montserrat-Regular.ttf";
pub const FONT_BOLD: &str = "fonts/Montserrat-Bold.ttf";

pub const XYLOPHONE_SWEEP: &str = "sounds/xylophone/xylophone_sweep.ogg";
pub const XYLOPHONE_A: &str = "sounds/xylophone/xylophone_a.ogg";
pub const XYLOPHONE_B: &str = "sounds/xylophone/xylophone_b.ogg";
pub const XYLOPHONE_C: &str = "sounds/xylophone/xylophone_c.ogg";
pub const XYLOPHONE_D: &str = "sounds/xylophone/xylophone_d.ogg";
pub const XYLOPHONE_E: &str = "sounds/xylophone/xylophone_e.ogg";
pub const XYLOPHONE_F: &str = "sounds/xylophone/xylophone_f.ogg";
pub const XYLOPHONE_G: &str = "sounds/xylophone/xylophone_g.ogg";
pub const XYLOPHONE_GM: &str = "sounds/xylophone/xylophone_gm.ogg";

pub const PLAYER_SIZE: f32 = 32.0;
pub const PLAYER_SPEED: f32 = 100.0;
pub const PLAYER_STEPS_PRO_SEC: usize = 10;
pub const PLAYER_SPAWN_POS: Vec3 = Vec3::ZERO;
pub const ENEMY_TO_PLAYER_DISTANCE: f32 = 10.;
pub const CATCH_RAD: f32 = 100.;
pub const MAP_H: f32 = 400.;
pub const MAP_W: f32 = 700.;

pub const TEXT_Z: f32 = 10.;
pub const PLAYER_Z: f32 = 0.;

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

pub const C_ACTIVATOR: KeyCode = KeyCode::KeyC;
pub const D_ACTIVATOR: KeyCode = KeyCode::KeyD;
pub const E_ACTIVATOR: KeyCode = KeyCode::KeyE;
pub const F_ACTIVATOR: KeyCode = KeyCode::KeyF;
pub const G_ACTIVATOR: KeyCode = KeyCode::KeyG;
pub const GM_ACTIVATOR: KeyCode = KeyCode::KeyM;
pub const A_ACTIVATOR: KeyCode = KeyCode::KeyA;
pub const B_ACTIVATOR: KeyCode = KeyCode::KeyB;

pub const ENEMY_SIZE: f32 = 22.0;
pub const NOTE_COLOR: Color = Color::linear_rgba(0., 0., 0., 1.);
pub const ENEMY_COLOR_BOOST: Color = Color::linear_rgba(0.5, 0., 0., 1.);
pub const ENEMY_COLOR_JUMP: Color = Color::linear_rgba(0., 0.5, 0., 1.);
pub const ENEMY_COLOR_CATCHRAD: Color = Color::linear_rgba(0., 0., 0.5, 1.);
pub const ENEMY_COLOR_BOOM: Color = Color::linear_rgba(0.2, 0.2, 0.2, 1.);
pub const ENEMY_COLOR_POOP: Color = Color::linear_rgba(0.8, 0.8, 0.4, 1.);

pub const NORMAL_BUTTON_COLOR: Color = Color::linear_rgba(0.1, 0.1, 0.1, 1.);
pub const HOVERED_BUTTON_COLOR: Color = Color::linear_rgba(0.1, 0.1, 0.1, 0.8);
pub const PRESSED_BUTTON_COLOR: Color = Color::linear_rgba(0.1, 0.1, 0.1, 0.5);
