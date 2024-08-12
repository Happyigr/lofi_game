use crate::bundles::*;
use crate::components::*;
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

pub fn on_enemy_despawn(
    trigger: Trigger<OnAdd, EnemyToDespawn>,
    mut commands: Commands,
    query: Query<&Transform, (With<Enemy>, With<EnemyToDespawn>)>,
    // mut ev_anim: EventWriter<AnimEvent>,
    mut anim_q: Query<(&mut Transform, &mut AnimConfig), Without<Enemy>>,
) {
    let e_ent = trigger.entity();

    let enemy_t = query.get(e_ent).unwrap();

    // start animation on the place of the enemy
    // ev_anim.send(AnimEvent(enemy_t.translation));

    let (mut anim_t, mut animation) = anim_q.single_mut();

    anim_t.translation = enemy_t.translation;

    // we create a new timer when the animation is triggered
    animation.frame_timer = AnimConfig::timer_from_fps(animation.fps);

    // despawn enemy
    commands.entity(e_ent).despawn_recursive();

    // spawning new enemy
    let (e_new, e_new_text) = EnemyBundle::new_random();
    commands.spawn(e_new).with_children(|parent| {
        parent.spawn(e_new_text);
    });
}
