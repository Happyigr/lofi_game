// use crate::constants::*;
// use bevy::prelude::*;
//
// #[derive(Resource, Default)]
// pub struct Materials {
//     pub boom_animation: Handle<Image>,
//     pub boom_animation_layout: Handle<TextureAtlasLayout>,
// }
//
// pub fn init_materials(
//     mut materials: ResMut<Materials>,
//     asset_server: Res<AssetServer>,
//     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
// ) {
//     let layout = TextureAtlasLayout::from_grid(UVec2::splat(512), 8, 8, None, None);
//     let boom_animation = asset_server.load(EXPLOSION_SPRITESHEET);
//     let boom_animation_layout = texture_atlas_layouts.add(layout);
//
//     let texture = asset_server.load(EXPLOSION_SPRITESHEET);
// }
