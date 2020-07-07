use amethyst::{
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
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
            let movement = match worker.side {
                Side::Left => input.axis_value("left_worker"),
                Side::Right => input.axis_value("right_worker"),
            };

           if let Some(mv_amount) = movement {
                let scaled_amount = 1.2 * mv_amount as f32;

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

