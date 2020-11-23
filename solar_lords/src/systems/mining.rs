use amethyst::{
    assets::{AssetStorage, Handle, Loader, Progress, ProgressCounter},
    core::{timing::Time, transform::Transform, math::*, Named, WithNamed,},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage, Entities},
    input::{InputHandler, StringBindings, VirtualKeyCode},
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture, Transparent},
};

use crate::solar_lords::{Player, Astroid};

#[derive(SystemDesc)]
pub struct MiningSystem;

impl<'s> System<'s> for MiningSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Astroid>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        ReadStorage<'s, Named>,
        Entities<'s>,
        Read<'s, InputHandler<StringBindings>>,
    );
    fn run(&mut self, (mut players, mut astroids, mut transforms, mut sprites, names, entities, input): Self::SystemData){

        if(input.action_is_down("mine").unwrap()){

            let mut player_target =  None;

            for (player) in (&mut players).join(){
                player_target = Some(player.target.as_ref().unwrap());
                //println!("{:?}", player_target.unwrap().to_string());
            }

            for (astroid, transform, sprite, name, entity) in (&mut astroids, &mut transforms, &mut sprites, &names, &entities).join(){
                if player_target.unwrap().to_string() == name.name.to_string() {
                    astroid.resources -= 1;
                    if(astroid.resources % 250 == 0){
                        sprite.sprite_number += 1
                    }
                    if(astroid.resources == 0){
                        entities.delete(entity);
                    }
                    println!("Hit!");
                }
                
            }
        }
        
    }
}