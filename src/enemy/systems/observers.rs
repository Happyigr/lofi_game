use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{
    animation::AnimConfig,
    enemy::{Enemy, EnemyBundle},
    player::{systems::PlayerUpgrade, CatchingRadius, Player},
    resources::{Game, Materials, MySettings},
};

#[derive(Event)]
pub struct EnemyKilled(pub Entity);

#[derive(Component)]
pub struct Catchable;

#[derive(Component)]
pub struct AwaitKilling;

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

pub fn on_await_killing(
    trigger: Trigger<OnAdd, AwaitKilling>,
    mut query: Query<(&mut Sprite, &Enemy)>,
) {
    let (mut sprite, enemy) = query.get_mut(trigger.entity()).unwrap();
    sprite.color = enemy.super_power.get_enemy_color().mix(&Color::BLACK, 0.5);
}

pub fn on_enemy_kill(
    mut ev_enemy_killed: EventReader<EnemyKilled>,
    mut ev_player_upgrade: EventWriter<PlayerUpgrade>,
    materials: Res<Materials>,
    en_q: Query<(&Transform, &Enemy), Without<Player>>,
    mut catch_rad_q: Query<&CatchingRadius, Without<Player>>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    audio: Res<Audio>,
    settings: Res<MySettings>,
) {
    for ev in ev_enemy_killed.read() {
        let (e_pos, enemy) = en_q.get(ev.0).unwrap();
        let catch_rad = catch_rad_q.get_single_mut().unwrap();

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
            AnimConfig::new(1, 63, settings.boom_anim_fps),
        ));

        // playing sound
        audio.play(enemy.super_power.get_sound(&materials));

        // activating superpower
        ev_player_upgrade.send(PlayerUpgrade(enemy.super_power.clone()));

        // incriasing score
        game.score += 1;

        // despawn enemy
        commands.entity(ev.0).despawn_recursive();

        // spawning new enemy
        let (e_new, e_new_text) =
            EnemyBundle::new_random_near_player(catch_rad.catching_radius_multiplier);
        commands.spawn(e_new).with_children(|parent| {
            parent.spawn(e_new_text);
        });
    }
}
