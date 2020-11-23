
use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    input::{InputBundle, StringBindings},
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod solar_lords;
mod systems;

use crate::solar_lords::SectorState;




//Main Application
fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let binding_path = app_root.join("config").join("input.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;


    let game_data = GameDataBuilder::default()
                    .with_bundle(TransformBundle::new())?
                    /*
                    .with_bundle(
                        InputBundle::<StringBindings>::new().with_bindings_from_file(input_bundle)?,
                    )?*/
                    .with_bundle(input_bundle)?
                    .with(systems::BackgroundMovementSystem, "background_movement", &["input_system"])
                    .with(systems::ZoomSystem, "camera_zoom_system", &["input_system"])
                    .with(systems::PlayerControlSystem, "player_control", &["input_system"])
                    .with(systems::MiningSystem, "mining", &["input_system"])
                    .with(systems::MouseRaycastSystem::default(), "mouse_raycast", &["input_system"])
                    .with_bundle(
                        RenderingBundle::<DefaultBackend>::new()
                            // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                            .with_plugin(
                                RenderToWindow::from_config_path(display_config_path)?
                                    .with_clear([0.0, 0.0, 0.0, 1.0]),
                            )
                            // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                            .with_plugin(RenderFlat2D::default()),
                    )?;

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, SectorState::default(), game_data)?;
    game.run();

    Ok(())
}


