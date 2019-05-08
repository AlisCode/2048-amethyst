#![warn(missing_docs)]

//! # 2048-amethyst 
//! 
//! 2048-amethyst is a Rust implementation of the famous game [2048](https://en.wikipedia.org/wiki/2048_(video_game)) 
//! using the [Amethyst game engine](https://amethyst.rs)
//!

use amethyst::{
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    utils::application_root_dir,
};

mod components;
mod entities;

struct Example;

impl SimpleState for Example {}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!(
        "{}/resources/display_config.ron",
        application_root_dir()
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([1., 1., 1., 1.], 1.)
            .with_pass(DrawFlat2D::new()),
    );

    let game_data =
        GameDataBuilder::default().with_bundle(RenderBundle::new(pipe, Some(config)))?;
    let mut game = Application::new("./", Example, game_data)?;

    game.run();

    Ok(())
}
