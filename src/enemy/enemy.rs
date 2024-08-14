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
    pub fn new_random() -> (EnemyBundle, Text2dBundle) {
        let super_power = SuperPower::rand_new();
        EnemyBundle::new(super_power)
    }

    // returns a bundle with SpriteBundle with enemy Component and Text2dBundle as child, to easily
    // control text
    pub fn new(super_power: SuperPower) -> (EnemyBundle, Text2dBundle) {
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
