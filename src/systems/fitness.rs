use amethyst::{
    core::{math::Vector3, timing::Time, transform::Transform, SystemDesc},
    derive::{SystemDesc},
    ecs::*,
    shrev::{EventChannel, ReaderId},
};

pub struct FitnessSystem{
    iteration: u32,
    iter_per_generation: u32
}

fn calc_fitness(worker: &Worker, transform: &Transform)->f32{
   let y_distance = transform.translation.y();
   let sum_choices = worker.dna.choices.sum();
   y_distance / sum_choices
}

impl<'s> System<'s> for GeneticSystem{
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Worker>
    );

    fn setup(&mut self, world: &mut World){
       Self::SystemData::setup(world);
       self.iteration = 0;
       self.iter_per_generation = 8;
    }

    fn run(&mut self, (transforms, workers) : Self::SystemData){
        if self.iteration == self.iter_per_generation {
            self.iteration = 0;
        }
        self.iteration += 1; 
    }
}

