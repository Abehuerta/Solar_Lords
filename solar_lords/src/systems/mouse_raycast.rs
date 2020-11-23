use amethyst::{
    assets::{AssetStorage, Handle, Loader, Progress, ProgressCounter},
    core::{
        geometry::Plane,
        math::{Point2, Vector2, Vector3},
        transform::{Transform, TransformBundle},
        Named, WithNamed,
    },
    derive::SystemDesc,
    ecs::{
        prelude::Entity, Entities, Join, Read, ReadExpect, ReadStorage, System, SystemData,
        WriteStorage, LazyUpdate,
    },
    input::{InputBundle, InputHandler, StringBindings},
    prelude::{Builder, World, WorldExt},
    renderer::{
        camera::{ActiveCamera, Camera},
        plugins::{RenderFlat2D, RenderToWindow},
        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat},
        types::DefaultBackend,
        ImageFormat, RenderingBundle, Texture, Transparent,
    },
    ui::{RenderUi, UiBundle, UiCreator, UiFinder, UiText},
    utils::application_root_dir,
    window::ScreenDimensions,
    Application, GameData, GameDataBuilder, SimpleState, SimpleTrans, StateData, Trans,
};

use crate::solar_lords::{Player, CAMERA_HEIGHT, CAMERA_WIDTH};
use crate::systems::load_sprite_system;

#[derive(SystemDesc, Default)]
pub struct MouseRaycastSystem {
    pub mining_ray: Option<Entity>,
}


impl<'s> System<'s> for MouseRaycastSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Player>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, SpriteRender>,
        ReadStorage<'s, Handle<SpriteSheet>>,
        ReadStorage<'s, Named>,
        Read<'s, AssetStorage<Texture>>,
        Read<'s, AssetStorage<SpriteSheet>>,
        ReadExpect<'s, Loader>,
        ReadExpect<'s, ScreenDimensions>,
        Read<'s, ActiveCamera>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut players,
            ray_transforms,
            cameras,
            sprites,
            spritesheet_handles,
            names,
            texture_storage,
            sheet_storage,
            loader,
            screen_dimensions,
            active_camera,
            input,
            lazy,
        ): Self::SystemData,
    ) {
        // Get the mouse position if its available
        if let Some(mouse_position) = input.mouse_position() {
            // Get the active camera if it is spawned and ready
            let mut camera_join = (&cameras, &ray_transforms).join();
            if let Some((camera, camera_ray_transform)) = active_camera
                .entity
                .and_then(|a| camera_join.get(a, &entities))
                .or_else(|| camera_join.next())
            {
                // Project a ray from the camera to the 0z axis
                let ray = camera.screen_ray(
                    Point2::new(mouse_position.0, mouse_position.1),
                    Vector2::new(screen_dimensions.width(), screen_dimensions.height()),
                    camera_ray_transform,
                );
                let distance = ray.intersect_plane(&Plane::with_z(0.0)).unwrap();
                let mouse_world_position = ray.at_distance(distance);

                // Find any sprites which the mouse is currently inside
                let mut found_name = None;
                for (sprite, ray_transform, name) in (&sprites, &ray_transforms, &names).join() {
                    let sprite_sheet = sheet_storage.get(&sprite.sprite_sheet).unwrap();
                    let sprite = &sprite_sheet.sprites[sprite.sprite_number];
                    let (min_x, max_x, min_y, max_y) = {
                        // Sprites are centered on a coordinate, so we build out a bbox for the sprite coordinate
                        // and dimensions
                        // Notice we ignore z-axis for this example.
                        (
                            ray_transform.translation().x - (sprite.width * 0.5),
                            ray_transform.translation().x + (sprite.width * 0.5),
                            ray_transform.translation().y - (sprite.height * 0.5),
                            ray_transform.translation().y + (sprite.height * 0.5),
                        )
                    };
                    if mouse_world_position.x > min_x
                        && mouse_world_position.x < max_x
                        && mouse_world_position.y > min_y
                        && mouse_world_position.y < max_y
                    {
                        found_name = Some(&name.name);
                    }
                }

                for (player) in (&mut players).join() {
                    player.target = Some(match found_name {
                        None => "".to_string(),
                        Some(x) => x.to_string(),
                    });
                }

                // This draws the mining rays
                
                if input.action_is_down("mine").unwrap() {

                    /*
                    let sprite_sheet = None;
                    for (spritesheet_handle) in (&spritesheet_handles).join() {
                        let sprite_sheet = &spritesheet_handle;
                        let mut ray_ray_transform = ray_Transform::default();
                        ray_ray_transform.set_translation_xyz(
                            CAMERA_WIDTH * 0.5,
                            CAMERA_HEIGHT * 0.5,
                            0.0,
                        );
                        let sprite = SpriteRender::new(sprite_sheet.unwrap(), 7);
                    }*/
                    let ray_sprite = load_sprite_system(
                        &texture_storage,
                        &sheet_storage,
                        &loader,
                        "solar_lords_spritesheet.png",
                        6,
                    );

                    let mut ray_transform = Transform::default();

                    ray_transform.set_translation_xyz(mouse_position.0, mouse_position.1, 4.0);

                    let mining_ray = Some(
                        lazy.create_entity(&entities)
                            .with(ray_sprite)
                            .with(ray_transform)
                            .build(),
                    );
                }
            }
        }
    }
}
