use crate::{
    constants::*,
    enemy::{
        systems::{Catchable, EnemyKilled},
        Enemy,
    },
    player::{player::CatchingRadius, Player},
};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
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
    mut catched_en_q: Query<(Entity, &Transform, &Enemy), With<Catchable>>,
    player_q: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut ev_enemy_kill: EventWriter<EnemyKilled>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let p_pos = player_q.get_single().unwrap();
    let mut enemy_to_kill: HashMap<KeyCode, (f32, Entity)> = HashMap::new();

    for (e_ent, e_pos, e_catched) in &mut catched_en_q {
        let tapped_keycode = e_catched.super_power.get_keycode();
        // if catched, and the same type of the enemy wasnt catched, teleport to the enemy and kill it

        if input.just_pressed(tapped_keycode) {
            if let Some((dist, _)) = enemy_to_kill.get_mut(&tapped_keycode) {
                if *dist > p_pos.translation.distance(e_pos.translation) {
                    enemy_to_kill.insert(
                        tapped_keycode,
                        (p_pos.translation.distance(e_pos.translation), e_ent),
                    );
                }
            } else {
                enemy_to_kill.insert(
                    tapped_keycode,
                    (p_pos.translation.distance(e_pos.translation), e_ent),
                );
            }
        }
    }

    // killing each tapped enemy, sorted by the distance to the player
    enemy_to_kill.into_iter().for_each(|(_, (_, e_ent))| {
        ev_enemy_kill.send(EnemyKilled(e_ent));
    });
}

pub fn move_player(
    mut player_q: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let (mut p_pos, p_settings) = player_q.get_single_mut().unwrap();

    // move player
    if input.pressed(p_settings.up_key) {
        p_pos.translation.y += PLAYER_SPEED * p_settings.speed_multiplier * time.delta_seconds();
    }
    if input.pressed(p_settings.down_key) {
        p_pos.translation.y -= PLAYER_SPEED * p_settings.speed_multiplier * time.delta_seconds();
    }
    if input.pressed(p_settings.left_key) {
        p_pos.translation.x -= PLAYER_SPEED * p_settings.speed_multiplier * time.delta_seconds();
    }
    if input.pressed(p_settings.right_key) {
        p_pos.translation.x += PLAYER_SPEED * p_settings.speed_multiplier * time.delta_seconds();
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
