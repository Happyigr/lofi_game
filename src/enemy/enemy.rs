use super::SuperPower;
use crate::constants::*;
use bevy::{math::NormedVectorSpace, prelude::*};
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
    pub fn new_random() -> (EnemyBundle, Text2dBundle) {
        let super_power = SuperPower::rand_new();
        EnemyBundle::new(super_power, 1.)
    }

    // returns a bundle with SpriteBundle with enemy Component and Text2dBundle as child, to easily
    // control text
    pub fn new(super_power: SuperPower, catch_rad_multiple: f32) -> (EnemyBundle, Text2dBundle) {
        let mut spawnpoint = get_spawnpoint(catch_rad_multiple);

        // be sure, that the enemies will dont spawn near the player
        while spawnpoint.distance(Vec2::new(0., 0.)) < PLAYER_SIZE + ENEMY_TO_PLAYER_DISTANCE {
            spawnpoint = get_spawnpoint(catch_rad_multiple);
        }

        (
            EnemyBundle {
                sprite: SpriteBundle {
                    transform: Transform::from_xyz(spawnpoint.x, spawnpoint.y, 1.),
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

fn get_spawnpoint(catch_rad_multiple: f32) -> Vec2 {
    // -10 will make the enemies to spawn not outside the catching radius
    let max_dist_from_player = CATCH_RAD + CATCHRAD_MULTIPLIER_DELTA * catch_rad_multiple - 10.;

    Vec2::new(
        rand::thread_rng().gen_range(-max_dist_from_player..max_dist_from_player),
        rand::thread_rng().gen_range(-max_dist_from_player..max_dist_from_player),
    )
}
