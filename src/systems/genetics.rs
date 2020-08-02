use amethyst::{
    core::{math::Vector3, timing::Time, transform::Transform, SystemDesc},
    derive::{SystemDesc},
    ecs::*,
    shrev::{EventChannel, ReaderId},
};

use super::spawner::{WorkerDistribution, SpawnEvent};
use rand::prelude::*;

#[derive(SystemDesc, Default)]
pub struct GeneticSystem{
    population: u32,
    generations: u32,
    gen_population: bool
}

//TODO: Rename this to SpawnSystem 
impl<'s> System<'s> for GeneticSystem{
    type SystemData = (
        Write<'s, EventChannel<SpawnEvent>>,
        Write<'s, LazyUpdate>,
        Read<'s, Time>
    );

    fn setup(&mut self, world: &mut World){
        Self::SystemData::setup(world);
        self.population = 10;
        self.gen_population = true; 
    }

    fn run(&mut self, (mut spawn_events, lazy_update, time) : Self::SystemData){
        if self.gen_population {
            self.gen_population = false;
            for i in 0..self.population{
                // world.create_entity();
                println!("Spawning element");
                
                let WorkerDistribution {worker_index} : WorkerDistribution = rand::random();

               spawn_events.single_write(
                   SpawnEvent{
                        tile_index: worker_index
                   });
                }
        }
    }
}

