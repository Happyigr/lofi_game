mod constants;

use std::time::Duration;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use constants::*;
use rand::Rng;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    // app.add_event::<AnimEvent>();
    app.add_systems(
        Startup,
        (spawn_camera, spawn_player, spawn_enemies, spawn_animation),
    );
    // app.add_systems(Update, (move_player, check_collison_with_radius));
    // trigger_animation_in_enemy,
    app.add_systems(Update, (check_collison_with_radius, execute_animations));

    app.observe(on_add_cathchable);
    app.observe(on_remove_cathchable);
    app.observe(on_enemy_despawn);

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct AnimConfig {
    first_sprite_i: usize,
    last_sprite_i: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_i: first,
            last_sprite_i: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

#[derive(Component)]
struct BoomAnim;

// #[derive(Event)]
// struct AnimEvent(Vec3);

// fn trigger_animation_in_enemy(
//     mut event_anim: EventReader<AnimEvent>,
//     mut query: Query<(&mut Transform, &mut AnimConfig)>,
// ) {
//     for ev in event_anim.read() {
//         // we expect the Component of type S to be used as a marker Component by only a single entity
//         let (mut transform, mut animation) = query.single_mut();
//
//         transform.translation = ev.0;
//
//         // we create a new timer when the animation is triggered
//         animation.frame_timer = AnimConfig::timer_from_fps(animation.fps);
//     }
// }

// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimConfig, &mut TextureAtlas)>) {
    for (mut config, mut atlas) in &mut query {
        // we track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished() {
            if atlas.index == config.last_sprite_i {
                // ...and it IS the last frame, then we move back to the first frame and stop.
                atlas.index = config.first_sprite_i;
            } else {
                // ...and it is NOT the last frame, then we move to the next frame...
                atlas.index += 1;
                // ...and reset the frame timer to start counting all over again
                config.frame_timer = AnimConfig::timer_from_fps(config.fps);
            }
        }
    }
}

fn spawn_animation(
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

#[derive(Component)]
struct Player {
    up_key: KeyCode,
    down_key: KeyCode,
    right_key: KeyCode,
    left_key: KeyCode,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            up_key: KeyCode::KeyW,
            down_key: KeyCode::KeyS,
            right_key: KeyCode::KeyA,
            left_key: KeyCode::KeyD,
        }
    }
}

fn spawn_player(
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

// fn move_player(
//     mut player_q: Query<(&mut Transform, &Player)>,
//     input: Res<ButtonInput<KeyCode>>,
//     time: Res<Time>,
// ) {
//     for (mut transform, settings) in &mut player_q {
//         if input.pressed(settings.up_key) {
//             transform.translation.y += PLAYER_SPEED * time.delta_seconds();
//         }
//         if input.pressed(settings.down_key) {
//             transform.translation.y -= PLAYER_SPEED * time.delta_seconds();
//         }
//         if input.pressed(settings.left_key) {
//             transform.translation.x += PLAYER_SPEED * time.delta_seconds();
//         }
//         if input.pressed(settings.right_key) {
//             transform.translation.x -= PLAYER_SPEED * time.delta_seconds();
//         }
//     }
// }

fn check_collison_with_radius(
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

#[derive(Component)]
struct Catchable;

fn on_add_cathchable(trigger: Trigger<OnAdd, Catchable>, mut query: Query<(&mut Sprite, &Enemy)>) {
    let (mut sprite, enemy) = query.get_mut(trigger.entity()).unwrap();
    sprite.color = enemy.super_power.get_enemy_color().mix(&Color::WHITE, 0.2);
}

fn on_remove_cathchable(
    trigger: Trigger<OnRemove, Catchable>,
    mut query: Query<(&mut Sprite, &Enemy)>,
) {
    let (mut sprite, enemy) = query.get_mut(trigger.entity()).unwrap();
    sprite.color = enemy.super_power.get_enemy_color();
}

#[derive(Component)]
struct EnemyToDespawn;

fn on_enemy_despawn(
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

#[derive(Clone, Debug, PartialEq, Eq)]
enum SuperPower {
    Boost,
    Jump,
    CatchRad,
    Boom,
}

impl SuperPower {
    fn rand_new() -> SuperPower {
        let num: u8 = rand::thread_rng().gen_range(0..3);

        match num {
            0 => SuperPower::Boost,
            1 => SuperPower::Jump,
            2 => SuperPower::CatchRad,
            3 => SuperPower::Boom,
            _ => unreachable!(),
        }
    }

    fn get_enemy_color(&self) -> Color {
        match self {
            SuperPower::Boost => ENEMY_COLOR_BOOST,
            SuperPower::Jump => ENEMY_COLOR_JUMP,
            SuperPower::CatchRad => ENEMY_COLOR_CATCHRAD,
            SuperPower::Boom => ENEMY_COLOR_BOOM,
        }
    }

    fn get_keycode(&self) -> KeyCode {
        match self {
            SuperPower::Boost => BOOST_ACTIVATOR,
            SuperPower::Jump => JUMP_ACTIVATOR,
            SuperPower::CatchRad => CATCHRAD_ACTIVATOR,
            SuperPower::Boom => BOOM_ACTIVATOR,
        }
    }

    fn get_keycode_str(&self) -> &str {
        match self {
            SuperPower::Boost => "1",
            SuperPower::Jump => "2",
            SuperPower::CatchRad => "3",
            SuperPower::Boom => "4",
        }
    }
}

#[derive(Component)]
struct Enemy {
    super_power: SuperPower,
}

fn spawn_enemies(mut commands: Commands) {
    (0..6).for_each(|_| {
        let (enemy, text) = EnemyBundle::new_random();
        commands.spawn(enemy).with_children(|parent| {
            parent.spawn(text);
        });
    })
}

#[derive(Bundle)]
struct EnemyBundle {
    sprite: SpriteBundle,
    enemy: Enemy,
}

impl EnemyBundle {
    fn new_random() -> (EnemyBundle, Text2dBundle) {
        let super_power = SuperPower::rand_new();
        EnemyBundle::new(super_power)
    }

    fn new(super_power: SuperPower) -> (EnemyBundle, Text2dBundle) {
        (
            EnemyBundle {
                sprite: SpriteBundle {
                    transform: Transform::from_xyz(
                        rand::thread_rng().gen_range(-MAP_W / 2.0..MAP_W / 2.),
                        rand::thread_rng().gen_range(-MAP_H / 2.0..MAP_H / 2.),
                        1.,
                    ),
                    sprite: Sprite {
                        color: super_power.get_enemy_color(),
                        custom_size: Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                enemy: Enemy {
                    super_power: super_power.clone(),
                },
            },
            Text2dBundle {
                transform: Transform::from_xyz(0., 0., TEXT_Z),
                text: Text::from_section(super_power.get_keycode_str(), TextStyle::default()),
                ..Default::default()
            },
        )
    }
}
