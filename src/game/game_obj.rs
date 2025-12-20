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
        let Some(image) = game_lib.images.get(&obj_config.image).cloned() else {
            error!("Cannot find {} in images", obj_config.image);
            return Err(MyError::NotFound(obj_config.image.clone()));
        };
        let size = arr_to_vec2(&obj_config.size);
        let screen_pos = map.get_screen_pos(&self.pos);

        let entity = commands.spawn((
            Sprite {
                image,
                custom_size: Some(size),
                image_mode: SpriteImageMode::Scale(ScalingMode::FillCenter),
                ..default()
            },
            Transform {
                translation: Vec3::new(screen_pos.x, screen_pos.y, obj_config.z),
                rotation: get_rotation(&self.direction),
                ..default()
            },
        ));

        Ok(entity.id())
    }
}
