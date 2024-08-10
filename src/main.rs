use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::Rng;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, (spawn_camera, spawn_player, spawn_enemies));
    app.add_systems(Update, (move_player, check_collison_with_radius));

    app.observe(on_add_cathchable);
    app.observe(on_remove_cathchable);

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct PlayerSettings {
    up_key: KeyCode,
    down_key: KeyCode,
    right_key: KeyCode,
    left_key: KeyCode,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            up_key: KeyCode::KeyW,
            down_key: KeyCode::KeyS,
            right_key: KeyCode::KeyA,
            left_key: KeyCode::KeyD,
        }
    }
}

const PLAYER_SIZE: f32 = 32.0;
const PLAYER_SPEED: f32 = 100.0;
const PLAYER_SPAWN_POS: Vec3 = Vec3::ZERO;
const CATCH_RAD: f32 = 100.;
const MAP_H: f32 = 400.;
const MAP_W: f32 = 700.;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            PlayerSettings::default(),
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

fn move_player(
    mut player_q: Query<(&mut Transform, &PlayerSettings)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, settings) in &mut player_q {
        if input.pressed(settings.up_key) {
            transform.translation.y += PLAYER_SPEED * time.delta_seconds();
        }
        if input.pressed(settings.down_key) {
            transform.translation.y -= PLAYER_SPEED * time.delta_seconds();
        }
        if input.pressed(settings.left_key) {
            transform.translation.x += PLAYER_SPEED * time.delta_seconds();
        }
        if input.pressed(settings.right_key) {
            transform.translation.x -= PLAYER_SPEED * time.delta_seconds();
        }
    }
}

fn check_collison_with_radius(
    catch_radius_q: Query<&Transform, With<PlayerSettings>>,
    mut not_catched_en_q: Query<(Entity, &Transform), (With<Enemy>, Without<Catchable>)>,
    mut catched_en_q: Query<(Entity, &Transform), (With<Enemy>, With<Catchable>)>,
    mut commands: Commands,
) {
    let p_pos = catch_radius_q.single();

    for (e_ent, e_pos) in &mut not_catched_en_q {
        if p_pos.translation.distance(e_pos.translation) < CATCH_RAD + ENEMY_SIZE / 2. {
            commands.entity(e_ent).insert(Catchable);
        }
    }

    for (e_ent, e_pos) in &mut catched_en_q {
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

fn catching(
    mut query: Query<(Entity, &Transform, &Enemy), With<Catchable>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (ent, mut transform, enemy) in &mut query {
        if input.just_pressed(enemy.super_power.get_keycode()) {
            dbg!();
        }
    }
}

fn on_remove_cathchable(
    trigger: Trigger<OnRemove, Catchable>,
    mut query: Query<(&mut Sprite, &Enemy)>,
) {
    let (mut sprite, enemy) = query.get_mut(trigger.entity()).unwrap();
    sprite.color = enemy.super_power.get_enemy_color();
}

enum SuperPower {
    Boost,
    Jump,
    CatchRad,
    Boom,
}

const BOOST_ACTIVATOR: KeyCode = KeyCode::Digit1;
const JUMP_ACTIVATOR: KeyCode = KeyCode::Digit2;
const CATCHRAD_ACTIVATOR: KeyCode = KeyCode::Digit3;
const BOOM_ACTIVATOR: KeyCode = KeyCode::Digit4;

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
}

#[derive(Component)]
struct Enemy {
    super_power: SuperPower,
}

const ENEMY_SIZE: f32 = 22.0;
const ENEMY_COLOR_BOOST: Color = Color::linear_rgba(0.5, 0., 0., 1.);
const ENEMY_COLOR_JUMP: Color = Color::linear_rgba(0., 0.5, 0., 1.);
const ENEMY_COLOR_CATCHRAD: Color = Color::linear_rgba(0., 0., 0.5, 1.);
const ENEMY_COLOR_BOOM: Color = Color::linear_rgba(0.2, 0.2, 0.2, 1.);

fn spawn_enemies(mut commands: Commands) {
    (0..6).for_each(|_| {
        let super_power = SuperPower::rand_new();
        commands.spawn((
            SpriteBundle {
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
            Enemy { super_power },
        ));
    })
}
