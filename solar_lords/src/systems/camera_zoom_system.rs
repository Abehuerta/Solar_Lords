use crate::solar_lords::PlayerCamera;
use amethyst::{
    core::{timing::Time, transform::Transform, math::{Vector2, Vector3, Matrix4}},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};


#[derive(SystemDesc)]
pub struct ZoomSystem;


impl<'s> System<'s> for ZoomSystem {
    type SystemData = (
        ReadStorage<'s, PlayerCamera>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (cameras, mut transforms, input): Self::SystemData){
        
        for (camera, transform) in (&cameras, &mut transforms).join() {
            if let zoom_level = input.mouse_wheel_value(false) {
                if (zoom_level > 0.0 && transform.scale() > &Vector3::new(1.0, 1.0, 0.0)){
                    let scale = transform.scale() - Vector3::new(0.1,0.1,0.0);
                    transform.set_scale(scale);
                    println!("zooming {}", zoom_level);
                }else if (zoom_level < 0.0 && transform.scale() < &Vector3::new(2.0, 2.0, 2.0)){
                    let scale = transform.scale() + Vector3::new(0.1,0.1,0.0);
                    transform.set_scale(scale);
                    println!("zooming {}, scale{}", zoom_level, transform.scale());
                }
            }
        }
    }
}