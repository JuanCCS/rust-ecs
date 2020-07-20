use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const GAME_HEIGHT: f32 = 100.0;
pub const GAME_WIDTH: f32 = 100.0;
pub const WORKER_WIDTH: f32 = 16.0;
pub const WORKER_HEIGHT: f32 = 16.0;

pub struct GOfLife;

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Worker {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

use crate::{
    resources::{load_assets, AssetType},
};



impl Worker {
    pub fn new(side: Side) -> Worker {
        Worker {
            side,
            width: WORKER_WIDTH,
            height: WORKER_HEIGHT,
        }
    }
}

impl Component for Worker{
    type Storage = DenseVecStorage<Self>;
}

impl SimpleState for GOfLife {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>){     
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);
        initialize_camera(world);
    }
}

fn initialize_camera(world: &mut World){
    // Setup camera in a way that our screen covers whole arena 
    // and (0, 0) is in the bottom left. 
    
    load_assets(world, vec![AssetType::Worker]);


    let mut transform = Transform::default();
    transform.set_translation_xyz(GAME_HEIGHT * 0.5, GAME_WIDTH * 0.5, 1.0);
    world
        .create_entity()
        .with(Camera::standard_2d(GAME_HEIGHT, GAME_WIDTH))
        .with(transform)
        .build();
}

fn initialize_walkers(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>){
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Correctly position the paddles.
    let y = GAME_HEIGHT / 2.0;
    left_transform.set_translation_xyz(WORKER_WIDTH * 0.5, y, 0.0);
    right_transform.set_translation_xyz(GAME_WIDTH - WORKER_WIDTH * 0.5, y, 0.0);
    
    // Assign the sprites for the paddles
    let left_sprite = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 1
    };


    let right_sprite = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 2
    };
        
        

    // Create a left plank entity.
    world
        .create_entity()
        .with(left_sprite)
        .with(Worker::new(Side::Left))
        .with(left_transform)
        .build();

    // Create right plank entity.
    world
        .create_entity()
        .with(right_sprite)
        .with(Worker::new(Side::Right))
        .with(right_transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "tiles.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "tiles.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
    
}



