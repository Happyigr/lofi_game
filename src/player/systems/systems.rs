use crate::{
    constants::*,
    enemy::{Catchable, Enemy},
    player::Player,
};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

pub fn check_collision_with_enemy(
    mut not_catched_en_q: Query<(Entity, &Transform), (With<Enemy>, Without<Catchable>)>,
    mut catched_en_q: Query<(Entity, &Transform), (With<Enemy>, With<Catchable>)>,
    player_q: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut commands: Commands,
) {
    let p_pos = player_q.get_single().unwrap();

    // if in the catching range, add catchable component
    for (e_ent, e_pos) in &mut not_catched_en_q {
        if p_pos.translation.distance(e_pos.translation) < CATCH_RAD + ENEMY_SIZE / 2. {
            commands.entity(e_ent).insert(Catchable);
        }
    }

    for (e_ent, e_pos) in &mut catched_en_q {
        // if not in the range, not catchable more.
        if p_pos.translation.distance(e_pos.translation) > CATCH_RAD + ENEMY_SIZE / 2. {
            commands.entity(e_ent).remove::<Catchable>();
        }
    }
}

pub fn try_to_kill_enemy(
    mut catched_en_q: Query<(Entity, &Transform, &Enemy), With<Catchable>>,
    mut player_q: Query<&mut Transform, (With<Player>, Without<Enemy>)>,
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut p_pos = player_q.get_single_mut().unwrap();

    for (e_ent, e_pos, e_catched) in &mut catched_en_q {
        // if catched, teleport to the enemy and kill it
        if input.just_pressed(e_catched.super_power.get_keycode()) {
            p_pos.translation = e_pos.translation;
            println!("Anim To Spawn: {:?}", e_pos.translation);

            commands.entity(e_ent).despawn_recursive();

            // let layout = TextureAtlasLayout::from_grid(UVec2::splat(512), 8, 8, None, None);
            // let texture = asset_server.load(EXPLOSION_SPRITESHEET);
            // let atlas = texture_atlas_layouts.add(layout);
            //
            // commands.spawn((
            //     SpriteBundle {
            //         transform: e_pos.clone(),
            //         texture: texture.clone(),
            //         ..Default::default()
            //     },
            //     TextureAtlas {
            //         layout: atlas,
            //         index: 1,
            //     },
            //     AnimConfig::new(1, 64, 10),
            // ));
        }
    }
}

pub fn move_player(
    mut player_q: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
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
        p_pos.translation.x -= PLAYER_SPEED * time.delta_seconds();
    }
    if input.pressed(p_settings.right_key) {
        p_pos.translation.x += PLAYER_SPEED * time.delta_seconds();
    }
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
