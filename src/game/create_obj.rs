use crate::config::*;
use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;

pub fn create_obj_by_config(
    map_obj_config: &GameMapObjConfig,
    world_info: &WorldInfo,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<(), MyError> {
    let pos = arr_to_vec2(&map_obj_config.pos);
    let direction = arr_to_vec2(&map_obj_config.direction).normalize();
    let config_index = game_lib.get_game_obj_config_index(&map_obj_config.config_name)?;

    create_obj_by_index(
        config_index,
        pos,
        direction,
        None,
        world_info,
        game_map,
        game_obj_lib,
        game_lib,
        commands,
    )
}

pub fn create_obj_by_index(
    config_index: usize,
    pos: Vec2,
    direction: Vec2,
    speed: Option<f32>,
    world_info: &WorldInfo,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<(), MyError> {
    if !world_info.contains(&pos) {
        let msg = format!("Failed to create GameObj: Position {} is out of map", pos);
        error!(msg);
        return Err(MyError::Other(msg));
    }

    let named_config = game_lib.get_game_obj_config(config_index);

    let entity = match &named_config.config {
        GameObjConfig::Bot(config) => create_bot_entity(
            &pos, &direction, speed, config, world_info, game_lib, commands,
        )?,
        GameObjConfig::Tile(config) => {
            create_tile_entity(&pos, &direction, config, world_info, game_lib, commands)?
        }
        GameObjConfig::Missile(config) => create_missile_entity(
            &pos, &direction, speed, config, world_info, game_lib, commands,
        )?,
        GameObjConfig::PlayFrame(config) => create_play_frame_entity(
            &named_config.name,
            &pos,
            &direction,
            config,
            world_info,
            game_lib,
            commands,
        )?,
    };

    add_obj(entity, config_index, pos, direction, game_map, game_obj_lib);

    Ok(())
}

fn create_bot_entity(
    pos: &Vec2,
    direction: &Vec2,
    speed: Option<f32>,
    config: &BotConfig,
    world_info: &WorldInfo,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<Entity, MyError> {
    let visible = world_info.check_pos_visible(pos);
    let size = arr_to_vec2(&config.size);
    let entity = create_main_body(&config.image, size, visible, game_lib, commands)?;
    let weapon_comp = create_weapon(entity, &config.weapon_config, game_lib, commands)?;
    let mut cmd = commands.entity(entity);

    cmd.insert(create_transform(pos, direction, config.z, world_info));
    cmd.insert(MoveComponent::new(speed.unwrap_or(0.0)));
    cmd.insert(weapon_comp);
    cmd.insert(HPComponent::new(config.hp));

    match config.side {
        GameObjSide::Player => {
            cmd.insert(PlayerComponent);
        }
        GameObjSide::AI => {
            cmd.insert(AIBotComponent);
            if let Some(ai_config_name) = config.ai.as_ref() {
                let ai_comp = create_ai_comp(ai_config_name, game_lib)?;
                cmd.insert(ai_comp);

                if visible {
                    cmd.insert(InView);
                }
            }
        }
    }

    debug!("created player {}", entity);

    Ok(entity)
}

fn create_tile_entity(
    pos: &Vec2,
    direction: &Vec2,
    tile_config: &TileConfig,
    world_info: &WorldInfo,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<Entity, MyError> {
    let visible = world_info.check_pos_visible(pos);
    let size = arr_to_vec2(&tile_config.size);
    let entity = create_main_body(&tile_config.image, size, visible, game_lib, commands)?;
    let mut cmd = commands.entity(entity);

    cmd.insert(create_transform(pos, direction, tile_config.z, world_info));
    cmd.insert(TileComponent);

    debug!("created tile {}", entity);

    Ok(entity)
}

pub fn create_missile_entity(
    pos: &Vec2,
    direction: &Vec2,
    speed: Option<f32>,
    config: &MissileConfig,
    world_info: &WorldInfo,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<Entity, MyError> {
    let size = arr_to_vec2(&config.size);
    let entity = create_main_body(&config.image, size, true, game_lib, commands)?;
    let mut cmd = commands.entity(entity);

    cmd.insert(create_transform(pos, direction, config.z, world_info));
    cmd.insert(MoveComponent::new(speed.unwrap_or(config.speed)));
    cmd.insert(MissileComponent);

    debug!("created missile {}", entity);

    Ok(entity)
}

fn create_play_frame_entity(
    config_name: &String,
    pos: &Vec2,
    direction: &Vec2,
    config: &PlayFrameConfig,
    world_info: &WorldInfo,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<Entity, MyError> {
    let image = game_lib.get_image(&config.image)?;
    let layout = game_lib.get_tex_atlas_layout(config_name)?;
    let transform = create_transform(pos, direction, config.z, world_info);
    let entity = commands
        .spawn((
            Sprite::from_atlas_image(image, TextureAtlas { layout, index: 0 }),
            transform,
            Visibility::Visible,
            PlayComponent::new(config.frames_per_second, config.frame_count),
            ExplosionComponent,
        ))
        .id();

    debug!("created frame-play {}", entity);

    Ok(entity)
}

fn create_main_body(
    image_name: &String,
    size: Vec2,
    visible: bool,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<Entity, MyError> {
    let image = game_lib.get_image(&image_name)?;
    let visibility = if visible {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    let entity = commands
        .spawn((
            Sprite {
                image,
                custom_size: Some(size),
                ..default()
            },
            visibility,
        ))
        .id();

    Ok(entity)
}

fn create_transform(pos: &Vec2, direction: &Vec2, z: f32, world_info: &WorldInfo) -> Transform {
    let screen_pos = world_info.get_screen_pos(pos);
    Transform {
        translation: Vec3::new(screen_pos.x, screen_pos.y, z),
        rotation: get_rotation(direction),
        ..default()
    }
}

fn create_weapon(
    main_body: Entity,
    weapon_config: &WeaponConfig,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<WeaponComponent, MyError> {
    add_guns(main_body, weapon_config, game_lib, commands)?;

    let weapon_comp = WeaponComponent::new(weapon_config, game_lib)?;

    Ok(weapon_comp)
}

fn add_guns(
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
                Visibility::Inherited,
            ))
            .id();

        commands.entity(main_body).add_child(gun);
    }

    Ok(())
}

fn add_obj(
    entity: Entity,
    config_index: usize,
    pos: Vec2,
    direction: Vec2,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
) {
    let obj = GameObj {
        config_index,
        pos,
        direction,
        map_pos: game_map.get_map_pos(&pos),
        is_phasing: false,
    };
    game_map.add(&obj.map_pos, entity);
    game_obj_lib.insert(entity, obj);
}

fn create_ai_comp(ai_config_name: &String, game_lib: &GameLib) -> Result<AIComponent, MyError> {
    let ai_config = game_lib.get_ai_config(ai_config_name)?;
    let ai_comp = AIComponent::new(ai_config);
    Ok(ai_comp)
}
