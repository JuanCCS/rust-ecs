use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir
};

mod game_of_life;
mod systems;
mod entities;
mod resources;
mod components;

use crate::game_of_life::GOfLife;


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::WorkerSystem, "worker_system", &["input_system"])
        .with(systems::SpawnSystem::default(), "spawn_system", &[])
//        .with(systems::DebugTriggerSystem::default(), "debug_trigger", &[])
        .with(systems::GeneticSystem::default(), "genetic_system", &[]);

    let mut game = Application::new(assets_dir, GOfLife, game_data)?;
    game.run();

    Ok(())
}
