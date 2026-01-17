use crate::config::*;
use crate::game::*;
use crate::game_utils::*;
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
        game_map: &GameMap,
        game_lib: &GameLib,
        commands: &mut Commands,
        asset_server: &AssetServer,
    ) -> Result<(Self, Entity), MyError> {
        let obj = Self {
            config_index,
            pos: pos.clone(),
            map_pos: game_map.get_map_pos(pos),
            direction: direction.clone(),
        };
        let obj_config = game_lib.get_game_obj_config(config_index);
        let visible = game_map.check_pos_visible(&obj.pos);
        let entity = if obj_config.obj_type == GameObjType::Explosion {
            obj.create_explosion(
                obj_config,
                visible,
                game_lib,
                game_map,
                commands,
                asset_server,
            )?
        } else {
            obj.create_regular_obj(obj_config, visible, game_lib, game_map, commands)?
        };

        Ok((obj, entity))
    }

    fn create_regular_obj(
        &self,
        obj_config: &GameObjConfig,
        visible: bool,
        game_lib: &GameLib,
        game_map: &GameMap,
        commands: &mut Commands,
    ) -> Result<Entity, MyError> {
        let image = game_lib.get_image(&obj_config.image)?;
        let screen_pos = game_map.get_screen_pos(&self.pos);
        let visibility = if visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        let entity = commands
            .spawn((
                Sprite {
                    image,
                    custom_size: Some(obj_config.size()),
                    ..default()
                },
                Transform {
                    translation: Vec3::new(screen_pos.x, screen_pos.y, obj_config.z),
                    rotation: get_rotation(&self.direction.normalize()),
                    ..default()
                },
                visibility,
            ))
            .id();

        if obj_config.obj_type == GameObjType::Bot {
            if obj_config.side == GameObjSide::Player {
                commands.entity(entity).insert(PlayerComponent::new());
            } else if obj_config.side == GameObjSide::AI {
                commands.entity(entity).insert(AIComponent);
            }
        } else if obj_config.obj_type == GameObjType::Missile {
            commands.entity(entity).insert(MissileComponent);
        }

        if let Some(weapon_config) = obj_config.weapon_config.as_ref() {
            let weapon_component = WeaponComponent::new(weapon_config, game_lib)?;
            commands.entity(entity).insert(weapon_component);
            self.add_guns(entity, weapon_config, game_lib, commands)?;
        }

        Ok(entity)
    }

    fn create_explosion(
        &self,
        obj_config: &GameObjConfig,
        visible: bool,
        game_lib: &GameLib,
        map: &GameMap,
        commands: &mut Commands,
        asset_server: &AssetServer,
    ) -> Result<Entity, MyError> {
        let image = game_lib.get_image(&obj_config.image)?;
        let Some(play_config) = obj_config.play_config.as_ref() else {
            error!("Missing PlayConfig in GameObjConfig");
            return Err(MyError::NotFound("PlayConfig".into()));
        };
        let size = obj_config.size();
        let screen_pos = map.get_screen_pos(&self.pos);
        let visibility = if visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        let tile_size = UVec2 {
            x: size.x as u32,
            y: size.y as u32,
        };
        let layout = asset_server.add(TextureAtlasLayout::from_grid(
            tile_size,
            play_config.frame_count as u32,
            1,
            None,
            None,
        ));

        let entity = commands
            .spawn((
                Sprite::from_atlas_image(image, TextureAtlas { layout, index: 0 }),
                Transform::from_xyz(screen_pos.x, screen_pos.y, obj_config.z),
                visibility,
                PlayComponent::new(play_config),
                ExplosionComponent,
            ))
            .id();

        Ok(entity)
    }

    fn add_guns(
        &self,
        main_body: Entity,
        weapon_config: &WeaponConfig,
        game_lib: &GameLib,
        commands: &mut Commands,
    ) -> Result<(), MyError> {
        for gun_comp_config in weapon_config.gun_components.iter() {
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
