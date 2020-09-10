use amethyst::{
    
    assets::{AssetStorage, Handle, Loader},
    core::{timing::Time, transform::Transform, math::*},
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture, Transparent},
    ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform},
};


//Public Constants

//Camera Constraints
pub const CAMERA_HEIGHT_MIN: f32 = 375.0;
pub const CAMERA_HEIGHT_MAX: f32 = 750.0;
pub const CAMERA_WIDTH_MIN: f32 = 500.0;
pub const CAMERA_WIDTH_MAX: f32 = 1000.0;

//Arena Constraints
//pub const ARENA_HEIGHT: f32 = 800.0;
//pub const ARENA_WIDTH: f32 = 1600.0;

//Main_State
#[derive(Default)]
pub struct SectorState {
    background_sprite_handle: Option<Handle<SpriteSheet>>,
    spritesheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for SectorState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.background_sprite_handle.replace(load_background_sprite(world));
        self.spritesheet_handle.replace(load_sprite_sheet(world));
        
        let _camera = init_camera(world);
        init_background_sprite(world, self.background_sprite_handle.clone().unwrap());
        init_mining_ship(world, self.spritesheet_handle.clone().unwrap());
    }
}

//Mining Sprite
fn init_mining_ship(world: &mut World, spritesheet_handle: Handle<SpriteSheet>) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(CAMERA_WIDTH_MIN * 0.5, CAMERA_HEIGHT_MIN * 0.5, 0.0);
    let sprite = SpriteRender::new(spritesheet_handle, 0);

    world
        .create_entity()
        .with(transform)
        .with(sprite)
        .named("player_ship")
        .build()
}

//Background Sprites
fn init_background_sprite(world: &mut World, sprite_sheet: Handle<SpriteSheet>) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(CAMERA_WIDTH_MIN * 0.5, CAMERA_HEIGHT_MIN * 0.5, -10.0);
    let scale = Vector3::new(0.5, 0.5, 1.0);
    transform.set_scale(scale);
    let sprite = SpriteRender::new(sprite_sheet, 1);
    world
        .create_entity()
        .with(transform)
        .with(sprite)
        .named("background")
        .with(Transparent)
        .build()
}

fn load_background_sprite(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // 'texture_handle' is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "textures/background_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "textures/background_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

//Camera Initialising
fn init_camera(world: &mut World){
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(CAMERA_WIDTH_MIN * 0.5, CAMERA_HEIGHT_MIN * 0.5, 1.0);

    world
        .create_entity()
        .with(PlayerCamera::new())
        .with(Camera::standard_2d(CAMERA_WIDTH_MIN, CAMERA_HEIGHT_MIN))
        .with(transform)
        .named("camera")
        .build();
}

//Load Sprite Sheet
fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // 'texture_handle' is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "textures/solar_lords_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "textures/solar_lords_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

//Mining Tools
#[derive(PartialEq, Eq)]
pub enum MiningTool{
    Laser,
    MiningLaser,
    ParticleDisruptor,
    MatterDisintegrator,
}
pub enum Weapon{
    Laser,
    PlasmaLauncher,
    VoidBeam,
}

//Player Component
pub struct Player{
    pub PlayerTransform: Transform,
    pub Health: u32,
    pub Sheild: u32,
    pub MiningTool: MiningTool,
    pub Weapon: Weapon,
}

//Camera Zoom componant
pub struct PlayerCamera {
    pub width: f32,
    pub height: f32,
}

impl PlayerCamera {
    pub fn new() -> PlayerCamera {
        PlayerCamera {
            width: CAMERA_WIDTH_MIN,
            height: CAMERA_HEIGHT_MIN,
        }
    }
}

impl Component for PlayerCamera {
    type Storage = DenseVecStorage<Self>;
}