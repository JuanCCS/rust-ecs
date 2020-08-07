use amethyst::{
    ecs::{Join, Read, System, SystemData, WriteStorage, Entities, Write},
    derive::{SystemDesc},
    core::{Transform}
};

use crate::{
    game_of_life::{Worker, GAME_WIDTH},
    resources::GeneticsContext,
};

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
        Entities<'s>,
        Write<'s, GeneticsContext>
    );

    fn run(&mut self, (mut transforms, workers, mut entities, mut context): Self::SystemData) {  
        for (entity, transform, worker) in (&* entities, &mut transforms, &workers).join() {
            if worker.dna.movements[context.iteration] == 1 {
                let mv_index = worker.dna.choices[context.iteration] as usize;
                transform.append_translation_xyz(MOVE_DIRECTIONS[mv_index].x * 16.0, MOVE_DIRECTIONS[mv_index].y * 16.0, 0.);
                if transform.translation().y < 0. || 0. > transform.translation().x || transform.translation().x > GAME_WIDTH {
                    let _ = entities.delete(entity);           
            } 
        }
        }
        context.iteration += 1;
    }
}
