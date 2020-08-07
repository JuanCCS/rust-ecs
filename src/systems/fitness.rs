use amethyst::{
    core::{transform::Transform},
    derive::{SystemDesc},
    ecs::*,
    shrev::{EventChannel},
};

use crate::{
    resources::GeneticsContext,
    components::Dna,
    game_of_life::Worker,
    systems::NewGenerationEvent,
};

#[derive(SystemDesc, Default)]
pub struct FitnessSystem;


fn calc_fitness(worker: &Worker, transform: &Transform)->f32{
   let y_distance = transform.translation().y;
   let sum_choices = worker.dna.choices.iter().sum::<u32>() as f32;
   y_distance
}

impl<'s> System<'s> for FitnessSystem{
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Worker>,
        Write<'s, GeneticsContext>,
        Write<'s, EventChannel<NewGenerationEvent>>,
    );
    
    fn run(&mut self, (transforms, workers, mut context, mut new_gen_event) : Self::SystemData){
        if context.iteration % (context.iters_per_generation - 1) == 0 {
            let mut max_fitness = 0.;
            let mut best_dna = &Dna::new(&vec![0; 16]);
            let mut second_best_dna = &Dna::new(&vec![0; 16]);
            for (worker, transform) in (&workers, &transforms).join() {
                let worker_fitness = calc_fitness(worker, transform);
                if worker_fitness > max_fitness{
                    max_fitness = worker_fitness;
                    second_best_dna = best_dna;
                    best_dna = &worker.dna;
                }
                 
            } 
            new_gen_event.single_write(
               NewGenerationEvent{
                    best_dna: best_dna.clone(),
                    second_best_dna: second_best_dna.clone()
               });
            context.iteration = 0;
            } 
    } 
}

