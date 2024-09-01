use crate::{
    constants::*,
    enemy::{
        systems::{AwaitKilling, Catchable, EnemyKilled},
        Enemy,
    },
    player::{player::CatchingRadius, Player},
};
use bevy::{
    prelude::*,
    reflect::List,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    ui::debug,
    utils::HashMap,
};

pub fn check_collision_with_enemy(
    mut not_catched_en_q: Query<(Entity, &Transform), (With<Enemy>, Without<Catchable>)>,
    mut catched_en_q: Query<(Entity, &Transform), (With<Enemy>, With<Catchable>)>,
    player_q: Query<(&Transform, &CatchingRadius), Without<Enemy>>,
    mut commands: Commands,
) {
    let (catch_rad_pos, catch_rad) = player_q.get_single().unwrap();

    // if in the catching range, add catchable component
    for (e_ent, e_pos) in &mut not_catched_en_q {
        if catch_rad_pos.translation.distance(e_pos.translation)
            < CATCH_RAD * catch_rad.catching_radius_multiplier + ENEMY_SIZE / 2.
        {
            commands.entity(e_ent).insert(Catchable);
        }
    }

    for (e_ent, e_pos) in &mut catched_en_q {
        // if not in the range, not catchable more.
        if catch_rad_pos.translation.distance(e_pos.translation)
            > CATCH_RAD * catch_rad.catching_radius_multiplier + ENEMY_SIZE / 2.
        {
            commands.entity(e_ent).remove::<Catchable>();
        }
    }
}

pub fn try_to_kill_enemy(
    mut commands: Commands,
    mut catched_en_q: Query<(Entity, &Transform, &Enemy), (With<Catchable>, Without<AwaitKilling>)>,
    mut player_q: Query<(&Transform, &mut Player), Without<Enemy>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let (p_pos, mut p_settings) = player_q.get_single_mut().unwrap();
    let mut enemy_to_kill: HashMap<KeyCode, (f32, Entity, Vec2)> = HashMap::new();

    for (e_ent, e_pos, e_catched) in &mut catched_en_q {
        let tapped_keycode = e_catched.super_power.get_keycode();
        // if catched, and the same type of the enemy wasnt catched, teleport to the enemy and kill it

        if input.just_pressed(tapped_keycode) {
            // sorted insertation if the enemy by the distance from player to the enemy
            if let Some((dist, _, _)) = enemy_to_kill.get_mut(&tapped_keycode) {
                if *dist > p_pos.translation.distance(e_pos.translation) {
                    enemy_to_kill.insert(
                        tapped_keycode,
                        (
                            p_pos.translation.distance(e_pos.translation),
                            e_ent,
                            e_pos.translation.xy(),
                        ),
                    );
                }
            } else {
                enemy_to_kill.insert(
                    tapped_keycode,
                    (
                        p_pos.translation.distance(e_pos.translation),
                        e_ent,
                        e_pos.translation.xy(),
                    ),
                );
            }
        }
    }

    // killing each tapped enemy, sorted by the distance to the player
    enemy_to_kill
        .into_iter()
        .for_each(|(_, (_, e_ent, e_pos))| {
            commands.entity(e_ent).insert(AwaitKilling);
            p_settings.add_killed_enemy(e_pos, e_ent);
        });
}

pub fn move_player(
    mut player_q: Query<(&mut Transform, &mut Player)>,
    mut ev_enemy_kill: EventWriter<EnemyKilled>,
    _time: Res<Time>,
    _input: Res<ButtonInput<KeyCode>>,
) {
    let (mut p_pos, mut p_settings) = player_q.get_single_mut().unwrap();

    // move player keyboard
    // if input.pressed(p_settings.up_key) {
    //     p_pos.translation.y += PLAYER_SPEED * p_settings.speed_multiplier * time.delta_seconds();
    // }
    // if input.pressed(p_settings.down_key) {
    //     p_pos.translation.y -= PLAYER_SPEED * p_settings.speed_multiplier * time.delta_seconds();
    // }
    // if input.pressed(p_settings.left_key) {
    //     p_pos.translation.x -= PLAYER_SPEED * p_settings.speed_multiplier * time.delta_seconds();
    // }
    // if input.pressed(p_settings.right_key) {
    //     p_pos.translation.x += PLAYER_SPEED * p_settings.speed_multiplier * time.delta_seconds();
    // }

    // if there are no enemy to go to and there are some queued
    if p_settings.current_enemy.is_none() {
        if let Some((point, ent)) = p_settings.points_queue.pop() {
            p_settings.current_enemy = Some((point, ent));
            p_settings.steps_to_point = Some(
                (p_settings.start_point.distance(point)
                    / (PLAYER_STEPS_PRO_FIXED_UPDATE as f32 * p_settings.speed_multiplier))
                    as usize,
            );
        }
    }

    // if the step timer is ticked and we have a enemy to go to
    if p_settings.current_enemy.is_some() {
        p_settings.steps_done += 1;

        // calculating the steps, which are left to go to the enemy.
        if let Some(current_step) = p_settings
            .steps_to_point
            .unwrap()
            .checked_sub(p_settings.steps_done)
        {
            // prevent dividing by 0
            let current_step = current_step.max(1);
            // change the position of the player with lerp function from start point of the road to
            // enemy to the enemy
            p_pos.translation = p_settings
                .start_point
                .lerp(
                    p_settings.current_enemy.unwrap().0,
                    1. / current_step as f32,
                )
                .extend(PLAYER_Z);
        }
        // if we came to the enemy, reset all (by substracting with overflow -1 by usize)
        else {
            let (point, ent) = p_settings.current_enemy.unwrap();
            p_settings.steps_to_point = None;
            p_settings.steps_done = 0;
            p_settings.start_point = point;
            p_settings.current_enemy = None;
            ev_enemy_kill.send(EnemyKilled(ent));
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
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
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Annulus::new(CATCH_RAD - 1., CATCH_RAD))),
            material: material.add(Color::hsl(1., 92., 79.)),
            ..Default::default()
        },
        CatchingRadius::default(),
    ));
}
