use amethyst::{
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    core::math
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

            if(!worker.movement_enabled){
                continue
            }

            let movement = match worker.side {
                Side::Left => input.axis_value("left_worker"),
                Side::Right => input.axis_value("right_worker"),
            };


            if let Some(mv_amount) = movement {
                let scaled_amount = (WORKER_HEIGHT * mv_amount).round() as f32;
                let worker_y = transform.translation().y;
                transform.set_translation_y(
                (worker_y + scaled_amount)
                    .min(GAME_HEIGHT - WORKER_HEIGHT * 0.5)
                    .max(WORKER_HEIGHT * 0.5),
                );
        }
    }
}
}


#[derive(SystemDesc)]
pub struct MoveEnableSystem;

impl<'s> System<'s> for MoveEnableSystem {
    type SystemData = (
        WriteStorage<'s, Worker>,
        Read<'s, InputHandler<StringBindings>>
    );

    fn run(&mut self, (mut workers, input): Self::SystemData) {
        for worker in (&mut workers).join() {    
            let enable = match worker.side {
                Side::Left => input.action_is_down("enable_movement_left"),
                Side::Right => input.action_is_down("enable_movement_right"),
            };

            if let Some(en) = enable {
                worker.movement_enabled = en
            } 
        }
    }
}

