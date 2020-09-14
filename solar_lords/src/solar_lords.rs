use amethyst::{
    
    assets::{AssetStorage, Handle, Loader},
    core::{timing::Time, transform::Transform, math::*, Parent},
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
        
        let _player = init_mining_ship(world, self.spritesheet_handle.clone().unwrap());
        let _camera = init_camera(world, _player);
        init_background_sprites(world, self.background_sprite_handle.clone().unwrap());
    }
}

//Mining Sprite
fn init_mining_ship(world: &mut World, spritesheet_handle: Handle<SpriteSheet>) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(CAMERA_WIDTH_MIN * 0.5, CAMERA_HEIGHT_MIN * 0.5, 0.0);
    let sprite = SpriteRender::new(spritesheet_handle, 0);

    world
        .create_entity()
        .with(Player::new())
        .with(transform)
        .with(sprite)
        .named("player_ship")
        .build()
}

//Background Sprites
fn init_background_sprites(world: &mut World, sprite_sheet: Handle<SpriteSheet>){
    let mut b1_transform = Transform::default();
    b1_transform.set_translation_xyz(CAMERA_WIDTH_MIN * 0.5, CAMERA_HEIGHT_MIN * 0.5, -10.0);
    let b1_scale = Vector3::new(0.5, 0.5, 1.0);
    b1_transform.set_scale(b1_scale);
    let b1_sprite = SpriteRender::new(sprite_sheet.clone(), 0);
    world
        .create_entity()
        .with(Background::new(2.0))
        .with(b1_transform)
        .with(b1_sprite)
        .named("background_01")
        .with(Transparent)
        .build();

    let mut b2_transform = Transform::default();
    b2_transform.set_translation_xyz(CAMERA_WIDTH_MIN * 0.5, CAMERA_HEIGHT_MIN * 0.5, -9.0);
    let b2_scale = Vector3::new(1.0, 1.0, 1.0);
    b2_transform.set_scale(b2_scale);
    let b2_sprite = SpriteRender::new(sprite_sheet.clone(), 1);
    world
        .create_entity()
        .with(Background::new(1.8))
        .with(b2_transform)
        .with(b2_sprite)
        .named("background_02")
        .with(Transparent)
        .build();
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
fn init_camera(world: &mut World, parent: Entity){
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(transform)
        .with(PlayerCamera::new())
        .with(Parent { entity: parent })
        .with(Camera::standard_2d(CAMERA_WIDTH_MIN, CAMERA_HEIGHT_MIN))
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
#[derive(PartialEq, Eq)]
pub enum Weapon{
    Laser,
    PlasmaLauncher,
    VoidBeam,
}

//Background Component
pub struct Background{
    pub movement_speed: f32,
}

impl Background{
    pub fn new(speed: f32) -> Background {
        Background{
            movement_speed: speed,
        }
    }
}

impl Component for Background{
    type Storage = DenseVecStorage<Self>;
}

//Player Component
pub struct Player{
    pub hull: u32,
    pub sheild: u32,
    pub mining_tool: MiningTool,
    pub weapon: Weapon,
}

impl Player {
    pub fn new() -> Player {
        Player {
            hull: 100,
            sheild: 100,
            mining_tool: MiningTool::Laser,
            weapon: Weapon::Laser,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
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

//Astroid components
#[derive(PartialEq, Eq)]
enum Ore{
    Ice,
    Copper,
    Iron,
    Gold,
    Diamond,
    Plasma,
}


struct Astroid {
    pub ore: Ore,
    pub resources: u32,
    pub sprite_counter: u32,
}

impl Astroid {
    pub fn new() -> Astroid {
        Astroid{
            ore: Ore::Ice,
            resources: 180,
            sprite_counter: 0,
        }
    }
}

impl Component for Astroid {
    type Storage = DenseVecStorage<Self>;
}