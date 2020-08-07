use amethyst::{
    core::{Time},
    derive::{SystemDesc},
    ecs::*,
    shrev::{EventChannel, ReaderId},
};

use super::spawner::{WorkerDistribution, SpawnEvent};

use crate::{
    components::Dna,
    game_of_life::Worker,
};
use rand::prelude::*;

fn mutate(dna: &mut Dna){
    let mut rng = rand::thread_rng();  
    let mut_prob : f32 = rng.gen();
    if mut_prob > 0.98 {
        let mut_index = rng.gen_range(0,8);
        let new_choice = rng.gen_range(0,8);
        dna.movements[mut_index] = match dna.movements[mut_index]{
            0 => 1,
            1 => 0,
            _ => 0
        };
        dna.choices[mut_index] = new_choice;
    }
}

fn reproduce(parent_a: &Dna, parent_b: &Dna) -> Dna{
    let mut child_movement = vec![0; 8];
    let mut child_choices = vec![0; 8];
    for i in 0..8{
        let mut rng = rand::thread_rng();
        let a_prob : f32 = rng.gen();
            if a_prob < 0.5{ 
                child_movement[i] = parent_a.movements[i];
                child_choices[i] = parent_a.choices[i];
            }else{
                child_movement[i] = parent_b.movements[i];
                child_movement[i] = parent_b.choices[i]; 
            }
        }
    let concatenated = [&child_movement[..], &child_choices[..]].concat();
    
    Dna::new(&concatenated)
}

#[derive(SystemDesc, Default)]
pub struct GeneticsSpawnSystem{
    population: u32,
    generations: u32,
    gen_population: bool,
    new_gen_reader_id: Option<ReaderId<NewGenerationEvent>>,
}

#[derive(Debug, Clone)]
pub struct NewGenerationEvent{
    pub best_dna: Dna,
    pub second_best_dna: Dna
}

impl<'s> System<'s> for GeneticsSpawnSystem{
    type SystemData = ( 
        Write<'s, EventChannel<SpawnEvent>>,
        Write<'s, LazyUpdate>,
        Read<'s, Time>,
        Read<'s, EventChannel<NewGenerationEvent>>,
        WriteStorage<'s, Worker>,
        Entities<'s>
    );

    fn setup(&mut self, world: &mut World){
        Self::SystemData::setup(world);
        self.population = 32;
        self.gen_population = true; 
        self.new_gen_reader_id = Some(world.fetch_mut::<EventChannel<NewGenerationEvent>>().register_reader());
    }

    fn run(&mut self, (mut spawn_events, lazy_update, time, new_gen_events, workers, mut entities) : Self::SystemData){
        for event in new_gen_events.read(self.new_gen_reader_id.as_mut().unwrap()){
            for (e, worker) in (&*entities, &workers).join(){
                entities.delete(e);
            } 
            for i in 0..self.population{    
                let WorkerDistribution {worker_index} : WorkerDistribution = rand::random();
                let mut worker_dna = reproduce(&event.best_dna, &event.second_best_dna);
                mutate(&mut worker_dna); 
                spawn_events.single_write(
                   SpawnEvent{
                        tile_index: worker_index,
                        spawn_position: i,
                        worker_dna: worker_dna
                   });
                }
        }
    }
}


