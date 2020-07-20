use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::{math::Vector3, timing::Time, Transform, SystemDesc},
    derive::{SystemDesc},
    ecs::*,
    shrev::{EventChannel, ReaderId},
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::{
    game_of_life::{Worker, Side},
};

pub fn spawn_worker(entities: &Entities, 
    sprite_sheet_handle: Handle<SpriteSheet>,
    lazy_update: &Write<LazyUpdate>){
        let worker_entity: Entity = entities.create();
        let mut worker_transform = Transform::default();
        
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: 0,
        };

        worker_transform.set_translation_xyz(16.0, 50.0, 0.0);
        lazy_update.insert(worker_entity, Worker::new(Side::Right));
        lazy_update.insert(worker_entity, worker_transform);
        lazy_update.insert(worker_entity, sprite_render);
}
