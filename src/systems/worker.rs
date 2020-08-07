use amethyst::{
    ecs::{Join, Read, System, SystemData, WriteStorage, Entities},
    derive::{SystemDesc},
    input::{InputHandler, StringBindings},
    core::{Transform}
};

use crate::game_of_life::{Worker, GAME_WIDTH};

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
        Entities<'s>
    );

    fn run(&mut self, (mut transforms, workers, mut entities): Self::SystemData) {
    
        for (worker, transform) in (&workers, &mut transforms).join(){
            transform.append_translation_xyz(MOVE_DIRECTIONS[1].x, MOVE_DIRECTIONS[1].y, 0.);
        }

        for (entity, transform, worker) in (&* entities, &transforms, &workers).join() {
            println!("{:#?}", transform.translation()); 
            if transform.translation().y < 0. || 0. > transform.translation().x || transform.translation().x > GAME_WIDTH {
                let _ = entities.delete(entity);           
            } 
        }
    }
}
