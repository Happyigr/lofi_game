use crate::constants::*;
use bevy::{prelude::*, reflect::List};

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
    pub blue_crystal_img: Handle<Image>,
    pub gold_crystal_img: Handle<Image>,
    pub green_crystal_img: Handle<Image>,
    pub ice_crystal_img: Handle<Image>,
    pub purple_crystal_img: Handle<Image>,
    pub red_crystal_img: Handle<Image>,
    pub hit_male_layout: Handle<TextureAtlasLayout>,
    pub hit_top_male: Handle<Image>,
    pub hit_bottom_male: Handle<Image>,
    pub hit_left_male: Handle<Image>,
    pub hit_right_male: Handle<Image>,
    pub idle_male: Handle<Image>,
    pub notes: Vec<Handle<Image>>,
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
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 4, 1, None, Some(UVec2::splat(1)));
    let hit_male_layout = texture_atlas_layouts.add(layout);
    let hit_top_male = asset_server.load(HIT_TOP_MALE);
    let hit_bottom_male = asset_server.load(HIT_BOTTOM_MALE);
    let hit_left_male = asset_server.load(HIT_LEFT_MALE);
    let hit_right_male = asset_server.load(HIT_RIGHT_MALE);
    let idle_male = asset_server.load(IDLE_MALE);
    let icon = asset_server.load(MENU_ICON);
    let blue_crystal_img = asset_server.load(BLUE_CRYSTAL);
    let gold_crystal_img = asset_server.load(GOLD_CRYSTAL);
    let green_crystal_img = asset_server.load(GREEN_CRYSTAL);
    let ice_crystal_img = asset_server.load(ICE_CRYSTAL);
    let purple_crystal_img = asset_server.load(PURPLE_CRYSTAL);
    let red_crystal_img = asset_server.load(RED_CRYSTAL);
    let blue_note = asset_server.load(BLUE_NOTE);
    let orange_note = asset_server.load(ORANGE_NOTE);
    let red_note = asset_server.load(RED_NOTE);
    let pink_note = asset_server.load(PINK_NOTE);

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
    materials.blue_crystal_img = blue_crystal_img;
    materials.gold_crystal_img = gold_crystal_img;
    materials.green_crystal_img = green_crystal_img;
    materials.ice_crystal_img = ice_crystal_img;
    materials.purple_crystal_img = purple_crystal_img;
    materials.red_crystal_img = red_crystal_img;
    materials.hit_male_layout = hit_male_layout;
    materials.hit_top_male = hit_top_male;
    materials.hit_bottom_male = hit_bottom_male;
    materials.hit_left_male = hit_left_male;
    materials.hit_right_male = hit_right_male;
    materials.idle_male = idle_male;
    materials.notes.push(blue_note);
    materials.notes.push(pink_note);
    materials.notes.push(orange_note);
    materials.notes.push(red_note);
}
