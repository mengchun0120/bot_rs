use crate::config::game_obj_config::*;
use crate::game_utils::{game_lib::*, game_map::*};
use crate::misc::{my_error::*, utils::*};
use bevy::prelude::*;

#[derive(Clone, Resource)]
pub struct GameObj {
    pub config_name: String,
    pub pos: Vec2,
    pub map_pos: MapPos,
    pub direction: Vec2,
    pub side: GameObjSide,
    pub obj_type: GameObjType,
    pub speed: f32,
    pub collide_span: f32,
}

impl GameObj {
    pub fn new(
        config_name: &String,
        pos: &Vec2,
        map_pos: &MapPos,
        direction: &Vec2,
        game_lib: &GameLib,
        screen_coord: &ScreenCoord,
        commands: &mut Commands,
    ) -> Result<(Self, Entity), MyError> {
        let Some(obj_config) = game_lib.game_obj_configs.get(config_name) else {
            error!("Cannot find {} in game_obj_configs", config_name);
            return Err(MyError::NotFound(config_name.clone()));
        };

        let obj = Self {
            config_name: config_name.clone(),
            pos: pos.clone(),
            map_pos: map_pos.clone(),
            direction: direction.clone(),
            side: obj_config.side,
            obj_type: obj_config.obj_type,
            speed: obj_config.speed,
            collide_span: obj_config.collide_span,
        };

        let entity = obj.create_entity(obj_config, game_lib, screen_coord, commands)?;

        Ok((obj, entity))
    }

    fn create_entity(
        &self,
        obj_config: &GameObjConfig,
        game_lib: &GameLib,
        screen_coord: &ScreenCoord,
        commands: &mut Commands,
    ) -> Result<Entity, MyError> {
        let Some(image) = game_lib.images.get(&obj_config.image).cloned() else {
            error!("Cannot find {} in images", obj_config.image);
            return Err(MyError::NotFound(obj_config.image.clone()));
        };
        let size = arr_to_vec2(&obj_config.size);
        let screen_pos = screen_coord.screen_pos(&self.pos);

        info!("create_entity: {:?} {:?}", screen_pos, self.pos);

        let mut entity = commands.spawn((
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
