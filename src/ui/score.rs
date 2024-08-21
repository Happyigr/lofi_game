use bevy::prelude::*;

use crate::resources::Game;

#[derive(Component)]
pub struct Score;

pub fn spawn_score(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(format!("Your score: 0"), TextStyle::default()).with_style(
            Style {
                position_type: PositionType::Relative,
                top: Val::Px(50.),
                left: Val::Vw(-40.),
                justify_self: JustifySelf::Center,
                border: UiRect::all(Val::Px(2.)),
                ..Default::default()
            },
        ),
        Score,
    ));
}

pub fn change_score(game: ResMut<Game>, mut text_query: Query<&mut Text, With<Score>>) {
    let mut text = text_query.single_mut();
    text.sections[0].value = format!("Score: {}", game.score);
}
