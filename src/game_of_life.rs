use amethyst::{
    assets::Loader,
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{Camera},
    ui::{get_default_font, UiTransform, UiText, Anchor, TtfFormat},
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
    pub fn new(worker_dna: Dna) -> Worker {
        Worker {
            width: WORKER_WIDTH,
            height: WORKER_HEIGHT,
            dna: worker_dna
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
        initialise_scoreboard(world);
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

pub struct ScoreText{
    pub gen_text: Entity,
    pub best_dna_text: Entity,
    pub second_best_dna_text: Entity
}

fn initialise_scoreboard(world: &mut World) {
    
    let font = world.read_resource::<Loader>().load(
        "fonts/inconsolata.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );


    let gen_transform = UiTransform::new(
        "Generations".to_string(), Anchor::TopLeft, Anchor::TopLeft,
        8., -8., 1., 512., 18.,
    );
    let best_dna_transform = UiTransform::new(
        "BestDna".to_string(), Anchor::TopLeft, Anchor::TopLeft,
        8., -32., 1., 512., 18.,
    );
    let second_best_dna_transform = UiTransform::new(
        "SecondBestDna".to_string(), Anchor::TopLeft, Anchor::TopLeft,
        8., -48., 1., 512., 18.,
    );
    let gen_text = world
        .create_entity()
        .with(gen_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            12.,
        ))
        .build();

    let best_dna_text = world
        .create_entity()
        .with(best_dna_transform)
        .with(UiText::new(font.clone(), format!("{:?}", vec![0; 16]), [1., 1., 1., 1.], 12.))
        .build();

    let second_best_dna_text = world
        .create_entity()
        .with(second_best_dna_transform)
        .with(UiText::new(font, format!("{:?}", vec![0; 16]), [1., 1., 1., 1.], 12.))
        .build();

    world.insert(ScoreText { gen_text, best_dna_text, second_best_dna_text });
}

