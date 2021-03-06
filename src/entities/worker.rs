use amethyst::{
    assets::{Handle},
    core::{Transform},
    ecs::{Entity, LazyUpdate, Write, Entities},
    renderer::{SpriteSheet, SpriteRender}
};

use std::convert::TryFrom;
use crate::{
    game_of_life::{Worker},
    components::Dna
};

pub fn spawn_worker(entities: &Entities, 
    sprite_sheet_handle: Handle<SpriteSheet>,
    lazy_update: &Write<LazyUpdate>, tile_index: u32, spawn_position: u32, worker_dna: Dna){
        let worker_entity: Entity = entities.create();
        let mut worker_transform = Transform::default();
        

        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: usize::try_from(tile_index).unwrap(),
        };        
        let pos = spawn_position as f32;
        worker_transform.set_translation_xyz(pos * 16.0 + 8.0, 8.0, 0.0);
        lazy_update.insert(worker_entity, Worker::new(worker_dna));
        lazy_update.insert(worker_entity, worker_transform);
        lazy_update.insert(worker_entity, sprite_render);
}
