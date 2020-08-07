use amethyst::{
    core::{Time},
    derive::{SystemDesc},
    ecs::*,
    shrev::{EventChannel},
};

use super::spawner::{WorkerDistribution, SpawnEvent};

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
        self.population = 32;
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
                        tile_index: worker_index,
                        spawn_position: i
                   });
                }
        }
    }
}

