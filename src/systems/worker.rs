use amethyst::{
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    derive::{SystemDesc},
    input::{InputHandler, StringBindings},
    core::{math, Transform, SystemDesc, math::Vector2}
};

use crate::game_of_life::{Worker, Side, GAME_HEIGHT, WORKER_HEIGHT};


#[derive(SystemDesc)]
pub struct WorkerSystem;

struct MoveDirection{
    x: f32,
    y: f32
}


const MOVE_DIRECTIONS: &[&MoveDirection; 8] = &[
    &MoveDirection{x:-1., y: 1.},
    &MoveDirection{x:0., y:1.},
    &MoveDirection{x:1., y:1.},
    &MoveDirection{x:-1., y:0.},
    &MoveDirection{x:1., y:0.},
    &MoveDirection{x:-1., y:-1.},
    &MoveDirection{x:0., y:-1.},
    &MoveDirection{x:1., y:-1.}
];



impl<'s> System<'s> for WorkerSystem { 

    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Worker>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, workers, input): Self::SystemData) {
        for (worker, transform) in (&workers, &mut transforms).join() {
            transform.append_translation_xyz(MOVE_DIRECTIONS[1].x, MOVE_DIRECTIONS[1].y * 16., 0.);
        }
    }
}
