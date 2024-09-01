use super::SuperPower;
use crate::constants::*;
use bevy::prelude::*;
use rand::Rng;

// Enemy tag wich have some superpower on kill
#[derive(Component)]
pub struct Enemy {
    pub super_power: SuperPower,
}

// Bundle of enemy, to get the Enemy, with it childs component (text)
#[derive(Bundle)]
pub struct EnemyBundle {
    sprite: SpriteBundle,
    enemy: Enemy,
}

impl EnemyBundle {
    // returns a new enemy with random superpower
    pub fn new_random_near_player(catch_rad_multiple: f32) -> (EnemyBundle, Text2dBundle) {
        let super_power = SuperPower::rand_new();
        EnemyBundle::new(super_power, catch_rad_multiple)
    }

    // returns a bundle with SpriteBundle with enemy Component and Text2dBundle as child, to easily
    // control text
    pub fn new(super_power: SuperPower, catch_rad_multiple: f32) -> (EnemyBundle, Text2dBundle) {
        let mut spawnpoint_norm_vec = Vec2::new(
            rand::thread_rng().gen_range(-1.0..1.),
            rand::thread_rng().gen_range(-1.0..1.),
        );

        // be sure, that the enemies will dont spawn in the player
        while spawnpoint_norm_vec == Vec2::new(0.0, 0.0) {
            spawnpoint_norm_vec = Vec2::new(
                rand::thread_rng().gen_range(-1.0..1.),
                rand::thread_rng().gen_range(-1.0..1.),
            );
        }

        let dist_from_player = rand::thread_rng().gen_range(
            PLAYER_SIZE + ENEMY_TO_PLAYER_DISTANCE..CATCH_RAD * catch_rad_multiple - ENEMY_SIZE,
        );

        (
            EnemyBundle {
                sprite: SpriteBundle {
                    transform: Transform::from_xyz(
                        spawnpoint_norm_vec.x * dist_from_player,
                        spawnpoint_norm_vec.y * dist_from_player,
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
