use std::collections::HashMap;

use amethyst::{
    assets::{AssetStorage, Handle, Loader, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    ecs::prelude::World,
    prelude::WorldExt, 
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteSheetFormat, SpriteSheetHandle},
        SpriteSheet, Texture,
    },
};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum AssetType{
    Worker
}

#[derive(Default)]
pub struct SpriteSheetList {
    sprite_sheets: HashMap<AssetType, Handle<SpriteSheet>>,
}

impl SpriteSheetList {
    pub fn insert(&mut self, asset_type: AssetType, sprite_sheet_handle: SpriteSheetHandle) {
        self.sprite_sheets.insert(asset_type, sprite_sheet_handle);
    }

    pub fn get(&self, asset_type: AssetType) -> Option<&SpriteSheetHandle> {
        self.sprite_sheets.get(&asset_type)
    }
}

pub fn load_assets(world: &mut World, asset_type_list: Vec<AssetType>){
    let mut sprite_sheet_list = SpriteSheetList::default();
    for &asset_type in asset_type_list.iter(){
        let (texture_path, ron_path) = match asset_type {
                AssetType::Worker => ("tiles.png", "tiles.ron")
        };
        match asset_type {
           AssetType::Worker => {
               let sprite_sheet_handle = get_sprite_sheet_handle(world, texture_path, ron_path);
               sprite_sheet_list.insert(asset_type, sprite_sheet_handle);
           }         
        }
    }
    world.insert(sprite_sheet_list);
}

pub fn get_sprite_sheet_handle(
    world: &World,
    texture_path: &str,
    ron_path: &str,
) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `sprite_sheet` is the layout of the sprites on the image
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(texture_path, ImageFormat::default(), (), &texture_storage)
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
