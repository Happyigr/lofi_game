use crate::constants::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct CatchingRadius {
    pub catching_radius_multiplier: f32,
}

impl Default for CatchingRadius {
    fn default() -> Self {
        Self {
            catching_radius_multiplier: 1.,
        }
    }
}

impl CatchingRadius {
    pub fn catch_rad_up(&mut self) {
        self.catching_radius_multiplier += CATCHRAD_MULTIPLIER_DELTA;
    }
}

#[derive(Component)]
pub struct Player {
    pub up_key: KeyCode,
    pub down_key: KeyCode,
    pub right_key: KeyCode,
    pub left_key: KeyCode,
    pub speed_multiplier: f32,
}

impl Player {
    pub fn speed_up(&mut self) {
        self.speed_multiplier += BOOST_MULTIPLIER_DELTA;
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            up_key: PLAYER_UP,
            down_key: PLAYER_DOWN,
            right_key: PLAYER_RIGHT,
            left_key: PLAYER_LEFT,
            speed_multiplier: 1.0,
        }
    }
}

// todo
// #[derive(Bundle)]
// pub struct PlayerBundle(SpriteBundle, Player);
//
// impl PlayerBundle {
//     pub fn new(materials: Res<Materials>) -> (PlayerBundle, MaterialMesh2dBundle) {
//         (
//             PlayerBundle(
//                 SpriteBundle {
//                     transform: Transform::from_translation(PLAYER_SPAWN_POS),
//                     sprite: Sprite {
//                         color: Color::WHITE,
//                         custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
//                         ..Default::default()
//                     },
//                     ..Default::default()
//                 },
//                 Player::default(),
//             ),
//             MaterialMesh2dBundle {
//                 mesh: Mesh2dHandle(materials.player_catching_radius_mesh),
//                 material: materials.player_catching_radius_color,
//                 ..Default::default()
//             },
//         )
//     }
// }
