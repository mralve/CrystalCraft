//#![windows_subsystem = "windows"]

use amethyst::{
    animation::{
        get_animation_set, AnimationBundle, AnimationCommand, AnimationControlSet, AnimationSet,
        AnimationSetPrefab, EndControl,
    },
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderSkybox, RenderFlat3D, RenderToWindow},
        sprite::{prefab::SpriteScenePrefab, SpriteRender},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

use amethyst_tiles::RenderTiles2D;

mod splashscreen;
mod crystal_craft;

pub mod miscfunc;

use crate::miscfunc::BaseTile;
use crate::splashscreen::SplashScreen;
use crate::crystal_craft::CrystalCraft;

fn main() -> amethyst::Result<()> {
    // We don't want the logger on release
    //amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            TransformBundle::new()
                .with_dep(&[]),
        )?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                //.with_plugin(RenderFlat2D::default())
                .with_plugin(RenderFlat3D::default())
                .with_plugin(RenderUi::default())
                .with_plugin(RenderSkybox::default())
                //.with_plugin(RenderTiles2D::<BaseTile>::default()),
        )?;
    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, CrystalCraft, game_data)?;
    game.run();

    Ok(())
}
