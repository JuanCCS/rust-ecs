use amethyst::{
    core::{timing::Time},
    derive::{SystemDesc},
    ecs::*,
    shrev::{EventChannel, ReaderId},
};

use crate::{
    entities::spawn_worker,
    resources::{SpriteSheetList, AssetType},
    components::Dna
};

use rand::{
    distributions::{Distribution, Standard}, Rng,
};


#[derive(Debug, Clone)]
pub struct SpawnEvent {
    pub tile_index: u32,
    pub spawn_position: u32,
    pub worker_dna: Dna
}

pub struct WorkerDistribution{
    pub worker_index: u32
}

impl Distribution<WorkerDistribution> for Standard{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> WorkerDistribution {
        match rng.gen_range(0,3) {
            0 => WorkerDistribution{
                worker_index: 0,
            },
            1 => WorkerDistribution {
                worker_index: 1,
            },
            _ => WorkerDistribution {
                worker_index: 2,
            }
        }
    }
}

#[derive(SystemDesc, Default)]
pub struct SpawnSystem{
    reader_id: Option<ReaderId<SpawnEvent>>,
}

// This System reads from the SpawnEvent channel and triggers the Spawning of the different
// entities.
impl<'s> System<'s> for SpawnSystem{
    type SystemData = (
        Entities<'s>,
        Read<'s, EventChannel<SpawnEvent>>,
        Write<'s, LazyUpdate>,
        Read<'s, SpriteSheetList>,
    );

    fn setup(&mut self, world: &mut World){
        Self::SystemData::setup(world);
        self.reader_id = Some(world.fetch_mut::<EventChannel<SpawnEvent>>().register_reader());
    

    }

    fn run(&mut self, (entities, spawn_events, lazy_update, sprite_sheet_list): Self::SystemData){
        for event in spawn_events.read(self.reader_id.as_mut().unwrap()){
            let worker_sprite_sheet =
                    { sprite_sheet_list.get(AssetType::Worker).unwrap().clone() };
            spawn_worker(&entities, worker_sprite_sheet, 
                &lazy_update, event.tile_index, 
                event.spawn_position, event.worker_dna.clone()); 
        }
    }
}

#[derive(SystemDesc, Default)]
pub struct DebugTriggerSystem{
    spawn_timer: f32,
}

// This System is used to Debug the SpawnSystem, after x seconds it will write
// to the SpawnEvent channel
impl<'s> System<'s> for DebugTriggerSystem{
    type SystemData = (
        Read<'s, LazyUpdate>,
        Write<'s, EventChannel<SpawnEvent>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (lazy_update, mut spawn_events, time): Self::SystemData){
        let delta_seconds = time.delta_seconds();
        self.spawn_timer -= delta_seconds;
        if self.spawn_timer <= 0.0 {
           self.spawn_timer = 1.5;
           let WorkerDistribution {worker_index} : WorkerDistribution = rand::random();

//           spawn_events.single_write(
//               SpawnEvent{
//                    tile_index: worker_index,
//                    spawn_position: 0
//               });
//        }
        }
    }
}

