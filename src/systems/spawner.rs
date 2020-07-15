use amethyst::{
    core::{math::Vector3, timing::Time, transform::Transform},
    ecs::*,
    shrev::{EventChannel, ReaderId},
};


use rand::{
    distributions::{Distribution, Standard},
    thread_rng, Rng,
};


#[derive(Debug, Clone)]
pub struct SpawnEvent {
    pub tile_index: u32,
}

struct WorkerDistribution{
    index: u32,
}

impl Distribution<WorkerDistribution> for Standard{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> WorkerDistribution {
        match rng.gen_range(0,3) {
            0 => WorkerDistribution{
                index: 0,
            },
            1 => WorkerDistribution {
                index: 1,
            },
            2 => WorkerDistribution {
                index: 2,
            }
        }
    }
}


pub struct SpawnSystem{
    reader_id: Option<ReaderId<SpawnEvent>>,
}


impl<'s> System<'s> for SpawnSystem{
    type SystemData {
        Read<'s, EventChannel<SpawnEvent>>,
        Write<'s, LazyUpdate>,
    }

    fn setup(&mut self, res: &mut Resources){
        Self::SystemData::setup(res);
        self.reader_id = Some(res.fetch_mut::<EventChannel<SpawnEvent>>().register_reader())
    }

    fn run(&mut self, (spawn_events, lazy_update): Self::SystemData){
        for event in spawn_events.read(self.reader_id.as_mut().unwrap()){
            // lazy_update.create_entity()
            println!("Handling Event: {}!", event.tile_index);
        } 
    }
}

pub struct DebugTriggerSystem{
    spawn_timer: f32,
}


impl<'s> System<'s> for DebugTriggerSystem{
    type SystemData = (
        Read<'s, LazyUpdate>,
        Write<'s, EventChannel<SpawnEvent>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (lazy_update, mut spawn_events, time){
        let delta_seconds = time.delta_seconds();
        if self.spawn_timer <= 0.0 {
           self.spawn_timer = 1.5;
           let WorkerDistribution {worker_index} : WorkerDistribution = rand::random();

           spawn_events.single_write(
               SpawnEvent{
                    tile_index: worker_index
               });
        }
    }
}

