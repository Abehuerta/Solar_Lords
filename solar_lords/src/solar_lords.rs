use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::*, timing::Time, transform::Transform, Named, Parent, WithNamed},
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{
        Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture, Transparent,
    },
    ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform},
    window::ScreenDimensions,
};
use rand::Rng;

//Public Constants

//Camera Constraints
pub const CAMERA_WIDTH: f32 = 1000.0;
pub const CAMERA_HEIGHT: f32 = 750.0;

//Arena Constraints
//pub const ARENA_HEIGHT: f32 = 800.0;
//pub const ARENA_WIDTH: f32 = 1600.0;

pub struct CameraDimensions{
    width: f32,
    height: f32,
}

//Main_State
#[derive(Default)]
pub struct SectorState {
    pub background_sprite_handle: Option<Handle<SpriteSheet>>,
    pub spritesheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for SectorState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.background_sprite_handle
            .replace(load_background_sprite(world));
        self.spritesheet_handle.replace(load_sprite_sheet(world));

        world.register::<Astroid>();

        let _player = init_mining_ship(world, self.spritesheet_handle.clone().unwrap());
        let _camera = init_camera(world, _player);
        init_background_sprites(
            world,
            self.background_sprite_handle.clone().unwrap(),
            _camera,
        );
        for i in 0..4 {
            init_astroids(world, self.spritesheet_handle.clone().unwrap(), i);
        }
    }

    fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        _camera = init_camera(world, _player);
        Trans::None
    }
}

//Mining Sprite
fn init_mining_ship(world: &mut World, spritesheet_handle: Handle<SpriteSheet>) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(CAMERA_WIDTH * 0.5, CAMERA_HEIGHT * 0.5, 0.1);
    let sprite = SpriteRender::new(spritesheet_handle, 0);

    world
        .create_entity()
        .with(Player::new())
        .with(transform)
        .with(sprite)
        .named("player_ship")
        .build()
}

//Astroid Initialization
fn init_astroids(world: &mut World, spritesheet_handle: Handle<SpriteSheet>, i: u32) {
    let mut rng = rand::thread_rng();
    let x_pos = rng.gen_range(-500.0, 500.0);
    let y_pos = rng.gen_range(-350.0, 350.0);

    let mut transform = Transform::default();
    transform.set_translation_xyz(x_pos, y_pos, 0.0);

    let sprite = SpriteRender::new(spritesheet_handle, 1);

    world
        .create_entity()
        .with(Astroid::new(Ore::Ice))
        .with(transform)
        .with(sprite)
        .named(format!("astroid {}", i))
        .build();
}

//Background Sprites
fn init_background_sprites(world: &mut World, sprite_sheet: Handle<SpriteSheet>, parent: Entity) {
    let mut b1_transform = Transform::default();
    b1_transform.set_translation_xyz(CAMERA_WIDTH * 0.5, CAMERA_HEIGHT * 0.5, -10.0);
    let b1_scale = Vector3::new(0.5, 0.5, 1.0);
    b1_transform.set_scale(b1_scale);
    let b1_sprite = SpriteRender::new(sprite_sheet.clone(), 0);
    world
        .create_entity()
        .with(Background::new(0.0))
        .with(b1_transform)
        .with(b1_sprite)
        .with(Parent {
            entity: parent.clone(),
        })
        .named("background_01")
        .with(Transparent)
        .build();

    let mut b2_transform = Transform::default();
    b2_transform.set_translation_xyz(CAMERA_WIDTH * 0.5, CAMERA_HEIGHT * 0.5, -9.0);
    let b2_scale = Vector3::new(1.0, 1.0, 1.0);
    b2_transform.set_scale(b2_scale);
    let b2_sprite = SpriteRender::new(sprite_sheet.clone(), 1);
    world
        .create_entity()
        .with(Background::new(-0.2))
        .with(b2_transform)
        .with(b2_sprite)
        .with(Parent {
            entity: parent.clone(),
        })
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
fn init_camera(world: &mut World, parent: Entity) -> Entity {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    let sd_width = world.fetch::<ScreenDimensions>().width();
    let sd_height = world.fetch::<ScreenDimensions>().height();

    world
        .create_entity()
        .with(transform)
        .with(PlayerCamera::new())
        .with(Parent { entity: parent })
        .with(Camera::standard_2d(sd_width, sd_height))
        .named("camera")
        .build()
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


/* LIGHTWIEGHT COMPONENTS */
pub fn init_mining_ray(world: &mut World, parent: Entity) -> Entity {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(transform)
        .with(PlayerCamera::new())
        .with(Parent { entity: parent })
        .with(Camera::standard_2d(CAMERA_WIDTH, CAMERA_HEIGHT))
        .named("camera")
        .build()
}

//Mining Tools
#[derive(PartialEq, Eq)]
pub enum MiningTool {
    Laser,
    MiningLaser,
    ParticleDisruptor,
    MatterDisintegrator,
}
#[derive(PartialEq, Eq)]
pub enum Weapon {
    Laser,
    PlasmaLauncher,
    VoidBeam,
}




//Background Component
pub struct Background {
    pub movement_speed: f32,
}

impl Background {
    pub fn new(speed: f32) -> Background {
        Background {
            movement_speed: speed,
        }
    }
}

impl Component for Background {
    type Storage = DenseVecStorage<Self>;
}

//Player Component
pub struct Player {
    pub hull: u32,
    pub sheild: u32,
    pub mining_tool: MiningTool,
    pub weapon: Weapon,
    pub speed: f32,
    pub wealth: f32,
    pub position: (f32, f32),
    pub zoom: f32,
    pub target: Option<String>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            hull: 100,
            sheild: 100,
            mining_tool: MiningTool::Laser,
            weapon: Weapon::Laser,
            speed: 1.0,
            wealth: 0.0,
            position: (0.0, 0.0),
            zoom: 2.0,
            target: None,
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
            width: CAMERA_WIDTH,
            height: CAMERA_HEIGHT,
        }
    }
}

impl Component for PlayerCamera {
    type Storage = DenseVecStorage<Self>;
}

//Astroid components

pub struct Astroid {
    pub ore: Ore,
    pub resources: u32,
}

#[derive(PartialEq, Eq)]
pub enum Ore {
    Ice,
    Copper,
    Iron,
    Gold,
    Diamond,
    Plasma,
}

impl Astroid {
    pub fn new(ore: Ore) -> Astroid {
        Astroid {
            ore,
            resources: 1000,
        }
    }
}

impl Component for Astroid {
    type Storage = DenseVecStorage<Self>;
}



/* LEIGHTWEIGHT ITEMS */

//Racast Mining Components
pub struct MiningRay{
    visible: bool,
    starting_transform: Transform,
    ending_transform: Transform,
}
