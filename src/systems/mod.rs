pub use self::worker::WorkerSystem;
pub use self::spawner::SpawnSystem;
pub use self::spawner::DebugTriggerSystem;
pub use self::genetics::GeneticsSpawnSystem;
pub use self::genetics::NewGenerationEvent;
pub use self::fitness::FitnessSystem;

mod genetics;
mod spawner;
mod worker;
mod fitness;
