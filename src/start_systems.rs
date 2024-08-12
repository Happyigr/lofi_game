use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::components::*;
use crate::constants::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_animation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(512), 8, 8, None, None);

    let anim_config = AnimConfig::new(1, 63, 10);

    commands.spawn((
        SpriteBundle {
            // transform: Transform::from_translation(ANIM_POS),
            transform: Transform::from_scale(Vec3::splat(6.0))
                .with_translation(Vec3::new(-50.0, 0.0, 0.0)),
            texture: asset_server.load("explosion_4.png"),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layouts.add(layout),
            index: anim_config.first_sprite_i,
        },
        BoomAnim,
        anim_config,
    ));
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            Player::default(),
            SpriteBundle {
                transform: Transform::from_translation(PLAYER_SPAWN_POS),
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Annulus::new(CATCH_RAD - 1., CATCH_RAD))),
                material: material.add(Color::hsl(1., 92., 79.)),
                ..Default::default()
            },));
        });
}

pub fn spawn_enemies(mut commands: Commands) {
    (0..6).for_each(|_| {
        let (enemy, text) = EnemyBundle::new_random();
        commands.spawn(enemy).with_children(|parent| {
            parent.spawn(text);
        });
    })
}
