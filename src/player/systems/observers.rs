use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    constants::*,
    enemy::SuperPower,
    player::{player::CatchingRadius, Player},
};

#[derive(Event)]
pub struct PlayerUpgrade(pub SuperPower);

pub fn on_player_upgrades(
    mut event_upgrade: EventReader<PlayerUpgrade>,
    mut player_q: Query<&mut Player>,
    mut catch_rad_q: Query<Entity, With<CatchingRadius>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for ev in event_upgrade.read() {
        let mut player = player_q.get_single_mut().unwrap();

        match ev.0 {
            SuperPower::Boost => player.speed_up(),
            SuperPower::CatchRad => {
                let catch_rad = catch_rad_q.get_single_mut().unwrap();
                player.catch_rad_up();
                commands.entity(catch_rad).insert(MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Annulus::new(
                        CATCH_RAD * player.catching_radius_multiplier - 1.,
                        CATCH_RAD * player.catching_radius_multiplier,
                    ))),
                    material: material.add(Color::hsl(1., 92., 79.)),
                    ..Default::default()
                });
            }
            SuperPower::Jump => println!("Jump Buff"),
            SuperPower::Boom => println!("Boom Buff"),
            SuperPower::Poop => println!("Poop Buff"),
            _ => println!("Notes"),
        }
    }
}
