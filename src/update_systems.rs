use crate::components::*;
use crate::constants::*;
use bevy::prelude::*;

pub fn check_collison_with_radius(
    mut not_catched_en_q: Query<(Entity, &Transform), (With<Enemy>, Without<Catchable>)>,
    mut catched_en_q: Query<(Entity, &Transform, &Enemy), With<Catchable>>,
    mut player_q: Query<(&mut Transform, &Player), Without<Enemy>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    let (mut p_pos, p_settings) = player_q.get_single_mut().unwrap();

    // move player temp
    if input.pressed(p_settings.up_key) {
        p_pos.translation.y += PLAYER_SPEED * time.delta_seconds();
    }
    if input.pressed(p_settings.down_key) {
        p_pos.translation.y -= PLAYER_SPEED * time.delta_seconds();
    }
    if input.pressed(p_settings.left_key) {
        p_pos.translation.x += PLAYER_SPEED * time.delta_seconds();
    }
    if input.pressed(p_settings.right_key) {
        p_pos.translation.x -= PLAYER_SPEED * time.delta_seconds();
    }

    // if in the catching range, add catchable component
    for (e_ent, e_pos) in &mut not_catched_en_q {
        if p_pos.translation.distance(e_pos.translation) < CATCH_RAD + ENEMY_SIZE / 2. {
            commands.entity(e_ent).insert(Catchable);
        }
    }

    for (e_ent, e_pos, e_catched) in &mut catched_en_q {
        // if catched, teleport to the enemy and kill it
        if input.just_pressed(e_catched.super_power.get_keycode()) {
            p_pos.translation = e_pos.translation;
            commands.entity(e_ent).despawn_recursive();
        }
        // if not in the range, not catchable more.
        if p_pos.translation.distance(e_pos.translation) > CATCH_RAD + ENEMY_SIZE / 2. {
            commands.entity(e_ent).remove::<Catchable>();
        }
    }
}
