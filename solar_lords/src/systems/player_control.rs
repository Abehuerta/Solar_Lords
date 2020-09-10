use amethyst::{
    core::{timing::Time, transform::Transform, math::{Vector2, Vector3, Matrix4}},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::solar_lords::Player;

#[derive(SystemDesc)]
pub struct PlayerControlSystem;

impl<'s> System<'s> for ZoomSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (cameras, mut transforms, input): Self::SystemData){}
}