use std::default;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, (spawn_camera, spawn_player));
    app.add_systems(Update, move_player);

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
const PLAYER_CATCH_RAD: f32 = 100.;

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
            parent.spawn(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(
                    meshes.add(Annulus::new(PLAYER_CATCH_RAD - 1., PLAYER_CATCH_RAD)),
                ),
                material: material.add(Color::hsl(51., 92., 79.)),
                ..Default::default()
            });
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
        if input.pressed(settings.right_key) {
            transform.translation.x -= PLAYER_SPEED * time.delta_seconds();
        }
        if input.pressed(settings.left_key) {
            transform.translation.x += PLAYER_SPEED * time.delta_seconds();
        }
    }
}
