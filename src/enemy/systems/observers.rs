use bevy::prelude::*;

use crate::{
    animation::AnimConfig,
    assets::Materials,
    enemy::{Enemy, EnemyBundle},
};

#[derive(Event)]
pub struct EnemyKilled(pub Entity);

#[derive(Component)]
pub struct Catchable;

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
pub fn on_enemy_kill(
    mut ev_enemy_killed: EventReader<EnemyKilled>,
    materials: Res<Materials>,
    en_q: Query<&Transform, With<Enemy>>,
    mut commands: Commands,
) {
    for ev in ev_enemy_killed.read() {
        let e_pos = en_q.get(ev.0).unwrap();

        // spawning animation
        commands.spawn((
            SpriteBundle {
                transform: e_pos.clone(),
                texture: materials.boom_animation_texture.clone(),
                ..Default::default()
            },
            TextureAtlas {
                layout: materials.boom_animation_layout.clone(),
                index: 1,
            },
            AnimConfig::new(1, 63, 40),
        ));

        // despawn enemy
        commands.entity(ev.0).despawn_recursive();

        // spawning new enemy
        let (e_new, e_new_text) = EnemyBundle::new_random();
        commands.spawn(e_new).with_children(|parent| {
            parent.spawn(e_new_text);
        });
    }
}
