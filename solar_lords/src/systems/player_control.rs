use amethyst::{
    core::{timing::Time, transform::Transform, math::*},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings, VirtualKeyCode},
};

use crate::solar_lords::Player;

#[derive(SystemDesc)]
pub struct PlayerControlSystem;

impl<'s> System<'s> for PlayerControlSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (players, mut transforms, input): Self::SystemData){
        let x_move = input.axis_value("entity_x").unwrap();
        let y_move = input.axis_value("entity_y").unwrap();
        
        for (player, transform) in (&players, &mut transforms).join() {
            transform.prepend_translation_x(x_move as f32 * 2.0);
            transform.prepend_translation_y(y_move as f32 * 2.0);
            let rotation = (x_move as i32, y_move as i32);
            transform.set_rotation_2d(match rotation {
                (1,0) => -std::f32::consts::PI/2.0,
                (1,1) => std::f32::consts::PI*7.0/4.0,
                (0,1) => 0.0,
                (-1,1) => std::f32::consts::PI/4.0,
                (-1,0) => std::f32::consts::PI/2.0,
                (-1,-1) => std::f32::consts::PI*3.0/4.0,
                (0,-1) => std::f32::consts::PI,
                (1,-1) => std::f32::consts::PI*5.0/4.0,
                (_,_) => continue,
            });
            println!("{:?}", rotation);
            println!("{}", transform.rotation().clone().angle());
        }
    }
}