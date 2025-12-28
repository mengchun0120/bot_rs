use crate::config::game_obj_config::*;
use crate::game::components::*;
use crate::game_utils::{game_lib::*, game_map::*};
use crate::misc::{my_error::*, utils::*};
use bevy::prelude::*;

#[derive(Clone, Resource)]
pub struct GameObj {
    pub config_index: usize,
    pub pos: Vec2,
    pub map_pos: MapPos,
    pub direction: Vec2,
}

impl GameObj {
    pub fn new(
        config_index: usize,
        pos: &Vec2,
        direction: &Vec2,
        map: &GameMap,
        game_lib: &GameLib,
        commands: &mut Commands,
    ) -> Result<(Self, Entity), MyError> {
        let obj = Self {
            config_index,
            pos: pos.clone(),
            map_pos: map.get_map_pos(pos),
            direction: direction.clone(),
        };

        let entity = obj.create_entity(config_index, game_lib, map, commands)?;

        Ok((obj, entity))
    }

    fn create_entity(
        &self,
        config_index: usize,
        game_lib: &GameLib,
        map: &GameMap,
        commands: &mut Commands,
    ) -> Result<Entity, MyError> {
        let obj_config = game_lib.get_game_obj_config(config_index);
        let image = game_lib.get_image(&obj_config.image)?;
        let size = arr_to_vec2(&obj_config.size);
        let screen_pos = map.get_screen_pos(&self.pos);

        let main_body = commands
            .spawn((
                Sprite {
                    image,
                    custom_size: Some(size),
                    ..default()
                },
                Transform {
                    translation: Vec3::new(screen_pos.x, screen_pos.y, obj_config.z),
                    rotation: get_rotation(&self.direction.normalize()),
                    ..default()
                },
            ))
            .id();

        self.add_guns(main_body, obj_config, game_lib, commands)?;
        self.add_components(main_body, obj_config, commands);

        Ok(main_body)
    }

    fn add_components(
        &self,
        main_body: Entity,
        obj_config: &GameObjConfig,
        commands: &mut Commands,
    ) {
        let mut entity = commands.entity(main_body);
        if obj_config.obj_type == GameObjType::Bot {
            if obj_config.side == GameObjSide::Player {
                entity.insert(PlayerComponent::new());
            } else if obj_config.side == GameObjSide::AI {
                entity.insert(AIComponent);
            }
        }
    }

    fn add_guns(
        &self,
        main_body: Entity,
        obj_config: &GameObjConfig,
        game_lib: &GameLib,
        commands: &mut Commands,
    ) -> Result<(), MyError> {
        let Some(gun_configs) = obj_config.gun_configs.as_ref() else {
            return Ok(());
        };

        for gun_comp_config in gun_configs.iter() {
            let gun_config = game_lib.get_gun_config(&gun_comp_config.config_name)?;
            let gun_img = game_lib.get_image(&gun_config.image)?;
            let gun_size = arr_to_vec2(&gun_config.size);
            let gun_direction = arr_to_vec2(&gun_comp_config.direction).normalize();
            let gun = commands
                .spawn((
                    Sprite {
                        image: gun_img,
                        custom_size: Some(gun_size),
                        ..default()
                    },
                    Transform {
                        translation: Vec3::new(
                            gun_comp_config.pos[0],
                            gun_comp_config.pos[1],
                            gun_config.z,
                        ),
                        rotation: get_rotation(&gun_direction),
                        ..default()
                    },
                ))
                .id();

            commands.entity(main_body).add_child(gun);
        }

        Ok(())
    }
}
