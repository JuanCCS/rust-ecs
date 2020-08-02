use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::{math::Vector3, timing::Time, Transform, SystemDesc},
    derive::{SystemDesc},
    ecs::*,
    shrev::{EventChannel, ReaderId},
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use rand::prelude::*;
use std::convert::TryFrom;
use crate::{
    game_of_life::{Worker, Side},
};

fn get_position()->f32{
    let mut rng = rand::thread_rng();
    let rand_result = rng.gen::<f32>(); 
    let rand_val = (rand_result * 8.0).floor() * 16.0 + 8.0;
    rand_val
}


pub fn spawn_worker(entities: &Entities, 
    sprite_sheet_handle: Handle<SpriteSheet>,
    lazy_update: &Write<LazyUpdate>, tile_index: u32){
        let worker_entity: Entity = entities.create();
        let mut worker_transform = Transform::default();
        

        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: usize::try_from(tile_index).unwrap(),
        };        
        
        let new_y: f32 = get_position();
        let new_x: f32 = get_position();
        println!("x: {}, y: {}", new_x, new_y);
        worker_transform.set_translation_xyz(new_x, new_y, 0.0);
        lazy_update.insert(worker_entity, Worker::new(Side::Right));
        lazy_update.insert(worker_entity, worker_transform);
        lazy_update.insert(worker_entity, sprite_render);
}
