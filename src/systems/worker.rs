use amethyst::{
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    derive::{SystemDesc},
    input::{InputHandler, StringBindings},
    core::{math, Transform, SystemDesc}
};

use crate::game_of_life::{Worker, Side, GAME_HEIGHT, WORKER_HEIGHT};


#[derive(SystemDesc)]
pub struct WorkerSystem;

impl<'s> System<'s> for WorkerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Worker>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, workers, input): Self::SystemData) {
        for (worker, transform) in (&workers, &mut transforms).join() { 
            println!("{:#?}, {:#?}", worker.dna.movements, worker.dna.choices) 
         }
        }
}
