use bevy::prelude::*;

use crate::enemy::EnemyBundle;

pub fn spawn_enemies(mut commands: Commands) {
    let (enemy, text) = EnemyBundle::new(crate::enemy::SuperPower::CatchRad, 1.);
    commands.spawn(enemy).with_children(|parent| {
        parent.spawn(text);
    });
    (0..6).for_each(|_| {
        // at the start is the catchin radius multiplier 1.
        let (enemy, text) = EnemyBundle::new_random_near_player(1.);
        commands.spawn(enemy).with_children(|parent| {
            parent.spawn(text);
        });
    })
}
