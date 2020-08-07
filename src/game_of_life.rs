use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera},
};

const WORKER_SIZE: f32 = 16.0;
const BOARD_SIZE: f32 = 32.0;

pub const GAME_HEIGHT: f32 = WORKER_SIZE * BOARD_SIZE;
pub const GAME_WIDTH: f32 =  GAME_HEIGHT;
pub const WORKER_WIDTH: f32 = WORKER_SIZE;
pub const WORKER_HEIGHT: f32 = WORKER_SIZE;

pub struct GOfLife;

use crate::{
    resources::{load_assets, AssetType, GeneticsContext},
    components::Dna
};

pub struct Worker {
    pub width: f32,
    pub height: f32,
    pub dna: Dna,
}


impl Worker {
    pub fn new(worker_dna: Vec<u32>) -> Worker {
        Worker {
            width: WORKER_WIDTH,
            height: WORKER_HEIGHT,
            dna: Dna::new(&worker_dna)
        }
    }
}

impl Component for Worker{
    type Storage = DenseVecStorage<Self>;
}

impl SimpleState for GOfLife {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>){     
        let world = data.world;
        world.insert(GeneticsContext::new());
        initialize_camera(world);
    }
}

fn initialize_camera(world: &mut World){
    
    load_assets(world, vec![AssetType::Worker]);


    let mut transform = Transform::default();
    transform.set_translation_xyz(GAME_HEIGHT * 0.5, GAME_WIDTH * 0.5, 1.0);
    world
        .create_entity()
        .with(Camera::standard_2d(GAME_HEIGHT, GAME_WIDTH))
        .with(transform)
        .build();
}

