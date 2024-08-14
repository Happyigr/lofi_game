use bevy::prelude::*;

#[derive(Component)]
pub struct SpawnBoomAnim(pub Transform);

// pub fn on_boom_animation_pending(
//     trigger: Trigger<OnAdd, SpawnBoomAnim>,
//     query: Query<&Transform, With<SpawnBoomAnim>>,
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
// ) {
//     dbg!("spawned");
//     let layout = TextureAtlasLayout::from_grid(UVec2::splat(512), 8, 8, None, None);
//     let texture = asset_server.load(EXPLOSION_SPRITESHEET);
//     let atlas = texture_atlas_layouts.add(layout);
//
//     let boom_t = query.get(trigger.entity()).unwrap();
//
//     commands.spawn((
//         SpriteBundle {
//             transform: boom_t.clone(),
//             texture: texture.clone(),
//             ..Default::default()
//         },
//         TextureAtlas {
//             layout: atlas,
//             index: 1,
//         },
//         AnimConfig::new(1, 64, 10),
//     ));
// }
