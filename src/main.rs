use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, (spawn_camera, spawn_player, spawn_enemy));
    app.add_systems(Update, (move_player, check_collison_with_radius));

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
    mut enemy_q: Query<(&mut Enemy, &Transform, &mut Sprite), With<Enemy>>,
) {
    let p_pos = catch_radius_q.single();

    for (mut enemy, e_pos, mut sprite) in &mut enemy_q {
        if p_pos.translation.distance(e_pos.translation) < CATCH_RAD {
            enemy.catchable = true;
            sprite.color = ENEMY_COLOR_CATCHABLE;
        } else {
            enemy.catchable = false;
            sprite.color = ENEMY_COLOR;
        }
    }
}

#[derive(Component)]
struct Enemy {
    catchable: bool,
}

const ENEMY_SIZE: f32 = 22.0;
const ENEMY_COLOR: Color = Color::hsl(40., 97., 79.);
const ENEMY_COLOR_CATCHABLE: Color = Color::hsl(100., 97., 79.);

fn spawn_enemy(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(20., 20., 1.),
            sprite: Sprite {
                color: ENEMY_COLOR,
                custom_size: Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
                ..Default::default()
            },
            ..Default::default()
        },
        Enemy { catchable: false },
    ));
}
