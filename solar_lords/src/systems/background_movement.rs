use amethyst::{
    core::{timing::Time, transform::Transform, math::{Vector2, Vector3, Matrix4}},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings, VirtualKeyCode},
};

use crate::solar_lords::{Background, PlayerCamera};

#[derive(SystemDesc)]
pub struct BackgroundMovementSystem;

impl<'s> System<'s> for BackgroundMovementSystem {
    type SystemData = (
        ReadStorage<'s, PlayerCamera>,
        ReadStorage<'s, Background>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (cameras, backgrounds, mut transforms, input): Self::SystemData){
        let x_move = input.axis_value("entity_x").unwrap();
        let y_move = input.axis_value("entity_y").unwrap();
        let rotation = (x_move as i32, y_move as i32);
        
        for (background, transform) in (&backgrounds, &mut transforms).join() {
            transform.prepend_translation_x(background.movement_speed * x_move);
            transform.prepend_translation_y(background.movement_speed * y_move);
        }

        for (camera, transform) in (&cameras, &mut transforms).join() {
            transform.set_rotation_2d(match rotation {
                (1,0) => -std::f32::consts::PI*3.0/2.0,
                (1,1) => std::f32::consts::PI/4.0,
                (0,1) => 0.0,
                (-1,1) => std::f32::consts::PI*7.0/4.0,
                (-1,0) => std::f32::consts::PI*3.0/2.0,
                (-1,-1) => std::f32::consts::PI*5.0/4.0,
                (0,-1) => std::f32::consts::PI,
                (1,-1) => std::f32::consts::PI*3.0/4.0,
                (_,_) => continue,
            });


        }
    }
}