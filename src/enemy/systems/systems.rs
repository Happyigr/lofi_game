use bevy::prelude::*;

use crate::{enemy::EnemyBundle, resources::Materials};

pub fn spawn_enemies(mut commands: Commands, materials: Res<Materials>) {
    let (enemy, text) = EnemyBundle::new(crate::enemy::SuperPower::CatchRad, 1., &materials);
    commands.spawn(enemy).with_children(|parent| {
        parent.spawn(text);
    });
    (0..6).for_each(|_| {
        // at the start is the catchin radius multiplier 1.
        let (enemy, text) = EnemyBundle::new_random_near_player(1., &materials);
        commands.spawn(enemy).with_children(|parent| {
            parent.spawn(text);
        });
    })
}
