use crate::constants::*;
use bevy::prelude::*;

#[derive(States, Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
pub enum GameState {
    #[default]
    Menu,
    Game,
    WinScreen,
}

#[derive(Resource)]
pub struct MySettings {
    pub bg_sound_volume: f64,
    pub audio_on: bool,
    pub boom_anim_fps: u8,
}

impl Default for MySettings {
    fn default() -> Self {
        Self {
            bg_sound_volume: 1.0,
            audio_on: true,
            boom_anim_fps: 60,
        }
    }
}

#[derive(Resource, Default)]
pub struct Game {
    pub score: usize,
}

#[derive(Default)]
pub struct Xylophone {
    pub sweep: Handle<bevy_kira_audio::AudioSource>,
    pub c: Handle<bevy_kira_audio::AudioSource>,
    pub d: Handle<bevy_kira_audio::AudioSource>,
    pub e: Handle<bevy_kira_audio::AudioSource>,
    pub f: Handle<bevy_kira_audio::AudioSource>,
    pub g: Handle<bevy_kira_audio::AudioSource>,
    pub a: Handle<bevy_kira_audio::AudioSource>,
    pub b: Handle<bevy_kira_audio::AudioSource>,
    pub gm: Handle<bevy_kira_audio::AudioSource>,
}

#[derive(Resource, Default)]
pub struct Materials {
    pub boom_animation_texture: Handle<Image>,
    pub boom_animation_layout: Handle<TextureAtlasLayout>,
    pub player_catching_radius_mesh: Handle<Mesh>,
    pub player_catching_radius_color: Handle<ColorMaterial>,
    pub bg_sound: Handle<bevy_kira_audio::AudioSource>,
    pub boom_sound: Handle<bevy_kira_audio::AudioSource>,
    pub heartbeat: Handle<bevy_kira_audio::AudioSource>,
    pub level_up_sound: Handle<bevy_kira_audio::AudioSource>,
    pub xylophone: Xylophone,
    pub font: Handle<Font>,
    pub font_bold: Handle<Font>,
    pub icon: Handle<Image>,
}

pub fn init_materials(
    mut materials: ResMut<Materials>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    // textures
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(512), 8, 8, None, None);
    let boom_animation_layout = texture_atlas_layouts.add(layout);
    let boom_animation_texture = asset_server.load(EXPLOSION_SPRITESHEET);
    let icon = asset_server.load(MENU_ICON);

    // sounds
    let bg_sound = asset_server.load(BG_MUSIC);
    let boom_sound = asset_server.load(BOOM_SOUND);
    let heartbeat = asset_server.load(HEARTBEAT_SOUND);
    let level_up_sound = asset_server.load(PLAYER_UPGRADE);
    let xylophone = Xylophone {
        sweep: asset_server.load(XYLOPHONE_SWEEP),
        c: asset_server.load(XYLOPHONE_C),
        d: asset_server.load(XYLOPHONE_D),
        e: asset_server.load(XYLOPHONE_E),
        f: asset_server.load(XYLOPHONE_F),
        g: asset_server.load(XYLOPHONE_G),
        a: asset_server.load(XYLOPHONE_A),
        b: asset_server.load(XYLOPHONE_B),
        gm: asset_server.load(XYLOPHONE_GM),
    };

    // misc
    let player_catching_radius_mesh = meshes.add(Annulus::new(CATCH_RAD - 1., CATCH_RAD));
    let player_catching_radius_color = material.add(Color::hsl(1., 92., 79.));

    // fonts
    let font = asset_server.load(FONT);
    let font_bold = asset_server.load(FONT_BOLD);

    materials.boom_animation_layout = boom_animation_layout;
    materials.boom_animation_texture = boom_animation_texture;
    materials.player_catching_radius_mesh = player_catching_radius_mesh;
    materials.player_catching_radius_color = player_catching_radius_color;
    materials.bg_sound = bg_sound;
    materials.boom_sound = boom_sound;
    materials.xylophone = xylophone;
    materials.heartbeat = heartbeat;
    materials.font = font;
    materials.font_bold = font_bold;
    materials.icon = icon;
    materials.level_up_sound = level_up_sound;
}
