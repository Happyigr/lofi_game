use crate::constants::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct MySettings {
    pub heartbeat_speed: f64,
    pub audio_on: bool,
    pub boom_anim_fps: u8,
}

impl Default for MySettings {
    fn default() -> Self {
        Self {
            heartbeat_speed: 1.0,
            audio_on: true,
            boom_anim_fps: 60,
        }
    }
}

#[derive(Resource, Default)]
pub struct Game {
    pub score: usize,
}

#[derive(Resource, Default)]
pub struct Materials {
    pub boom_animation_texture: Handle<Image>,
    pub boom_animation_layout: Handle<TextureAtlasLayout>,
    pub player_catching_radius_mesh: Handle<Mesh>,
    pub player_catching_radius_color: Handle<ColorMaterial>,
    pub heartbeat_bg_sound: Handle<bevy_kira_audio::AudioSource>,
}

pub fn init_materials(
    mut materials: ResMut<Materials>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(512), 8, 8, None, None);
    let boom_animation_layout = texture_atlas_layouts.add(layout);
    let boom_animation_texture = asset_server.load(EXPLOSION_SPRITESHEET);

    let heartbeat_bg_sound = asset_server.load(HEARTBEAT_SOUND);
    let player_catching_radius_mesh = meshes.add(Annulus::new(CATCH_RAD - 1., CATCH_RAD));
    let player_catching_radius_color = material.add(Color::hsl(1., 92., 79.));

    materials.boom_animation_layout = boom_animation_layout;
    materials.boom_animation_texture = boom_animation_texture;
    materials.player_catching_radius_mesh = player_catching_radius_mesh;
    materials.player_catching_radius_color = player_catching_radius_color;
    materials.heartbeat_bg_sound = heartbeat_bg_sound;
}
