use std::time::Duration;

use crate::bundles::*;
use crate::components::*;
use crate::constants::EXPLOSION_SPRITESHEET;
use bevy::prelude::*;

pub fn on_add_cathchable(
    trigger: Trigger<OnAdd, Catchable>,
    mut query: Query<(&mut Sprite, &Enemy)>,
) {
    let (mut sprite, enemy) = query.get_mut(trigger.entity()).unwrap();
    sprite.color = enemy.super_power.get_enemy_color().mix(&Color::WHITE, 0.2);
}

pub fn on_remove_cathchable(
    trigger: Trigger<OnRemove, Catchable>,
    mut query: Query<(&mut Sprite, &Enemy)>,
) {
    let (mut sprite, enemy) = query.get_mut(trigger.entity()).unwrap();
    sprite.color = enemy.super_power.get_enemy_color();
}

// dont working
pub fn on_enemy_despawn(
    trigger: Trigger<OnAdd, EnemyToDespawn>,
    mut commands: Commands,
    query: Query<&Transform, (With<Enemy>, With<EnemyToDespawn>)>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    // mut ev_anim: EventWriter<AnimEvent>,
) {
    let e_ent = trigger.entity();

    let enemy_t = query.get(e_ent).unwrap();
    // // spawning a animation \pending entity
    // let anim = commands.spawn_empty().id();
    // commands.entity(anim).insert(SpawnBoomAnim(enemy_t.clone()));

    dbg!("spawned");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(512), 8, 8, None, None);
    let texture = asset_server.load(EXPLOSION_SPRITESHEET);
    let atlas = texture_atlas_layouts.add(layout);

    commands.spawn((
        SpriteBundle {
            transform: enemy_t.clone(),
            texture: texture.clone(),
            ..Default::default()
        },
        TextureAtlas {
            layout: atlas,
            index: 1,
        },
        AnimConfig::new(1, 64, 10),
    ));

    // despawn enemy
    commands.entity(e_ent).despawn_recursive();

    // spawning new enemy
    let (e_new, e_new_text) = EnemyBundle::new_random();
    commands.spawn(e_new).with_children(|parent| {
        parent.spawn(e_new_text);
    });
}

#[derive(Component)]
pub struct AnimConfig {
    pub timer: Timer,
    pub first_i: usize,
    pub last_i: usize,
    pub fps: u8,
}

impl AnimConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_i: first,
            last_i: last,
            fps,
            timer: Self::timer_from_fps(fps),
        }
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

// pub fn on_boom_animation_pending(
//     trigger: Trigger<OnAdd, SpawnBoomAnim>,
//     query: Query<&Transform, With<SpawnBoomAnim>>,
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
// ) {
//     dbg!("spawned");
//     let layout = TextureAtlasLayout::from_grid(UVec2::splat(512), 8, 8, None, None);
//     let texture = asset_server.load(EXPLOSION_SPRITESHEET);
//     let atlas = texture_atlas_layouts.add(layout);
//
//     let boom_t = query.get(trigger.entity()).unwrap();
//
//     commands.spawn((
//         SpriteBundle {
//             transform: boom_t.clone(),
//             texture: texture.clone(),
//             ..Default::default()
//         },
//         TextureAtlas {
//             layout: atlas,
//             index: 1,
//         },
//         AnimConfig::new(1, 64, 10),
//     ));
// }
