use crate::config::*;
use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;

pub fn create_obj_by_config(
    map_obj_config: &GameMapObjConfig,
    world_info: &mut WorldInfo,
    game_map: &mut GameMap,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<(), MyError> {
    let config_index = game_lib.get_game_obj_config_index(&map_obj_config.config_name)?;
    let pos = arr_to_vec2(&map_obj_config.pos);
    let direction = arr_to_vec2(&map_obj_config.direction).normalize();

    create_obj_by_index(
        config_index,
        pos,
        direction,
        None,
        world_info,
        game_map,
        game_lib,
        commands,
    )
}

pub fn create_obj_by_index(
    config_index: usize,
    pos: Vec2,
    direction: Vec2,
    speed: Option<f32>,
    world_info: &mut WorldInfo,
    game_map: &mut GameMap,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<(), MyError> {
    if !world_info.contains(&pos) {
        let msg = format!("Failed to create GameObj: Position {} is out of map", pos);
        error!(msg);
        return Err(MyError::Other(msg));
    }

    let obj_config = game_lib.get_game_obj_config(config_index);
    let obj = GameObj {
        config_index,
        pos,
        direction,
        map_pos: game_map.get_map_pos(&pos),
    };
    match obj_config.obj_type {
        GameObjType::Bot => create_bot(
            obj, speed, obj_config, world_info, game_map, game_lib, commands,
        ),
        GameObjType::Tile => create_tile(obj, obj_config, world_info, game_map, game_lib, commands),
        GameObjType::Missile => create_missile(
            obj, speed, obj_config, world_info, game_map, game_lib, commands,
        ),
        GameObjType::Explosion => {
            create_explosion(obj, obj_config, world_info, game_map, game_lib, commands)
        }
    }
}

fn create_bot(
    obj: GameObj,
    speed: Option<f32>,
    obj_config: &GameObjConfig,
    world_info: &mut WorldInfo,
    game_map: &mut GameMap,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<(), MyError> {
    match obj_config.side {
        GameObjSide::Player => create_player(
            obj, speed, obj_config, world_info, game_map, game_lib, commands,
        ),
        GameObjSide::AI => create_ai_bot(
            obj, speed, obj_config, world_info, game_map, game_lib, commands,
        ),
        GameObjSide::Neutral => {
            let msg = "Cannot create netural bot".to_string();
            error!(msg);
            Err(MyError::Other(msg))
        }
    }
}

fn create_player(
    obj: GameObj,
    speed: Option<f32>,
    obj_config: &GameObjConfig,
    world_info: &mut WorldInfo,
    game_map: &mut GameMap,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<(), MyError> {
    let visible = world_info.check_pos_visible(&obj.pos);
    let main_body = create_main_body(obj_config, visible, game_lib, commands)?;
    let transform = create_transform(&obj.pos, &obj.direction, obj_config, world_info);
    let move_comp = MoveComponent::new(speed.unwrap_or(0.0));
    let weapon_comp = create_weapon(main_body, obj_config, game_lib, commands)?;
    let hp_comp = create_hp_comp(obj_config)?;
    let mut cmd = commands.entity(main_body);

    add_obj(main_body, &obj, obj_config, world_info, game_map);

    cmd.insert(obj);
    cmd.insert(transform);
    cmd.insert(Player);
    cmd.insert(move_comp);
    cmd.insert(weapon_comp);
    cmd.insert(hp_comp);

    Ok(())
}

fn create_ai_bot(
    obj: GameObj,
    speed: Option<f32>,
    obj_config: &GameObjConfig,
    world_info: &mut WorldInfo,
    game_map: &mut GameMap,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<(), MyError> {
    let visible = world_info.check_pos_visible(&obj.pos);
    let main_body = create_main_body(obj_config, visible, game_lib, commands)?;
    let transform = create_transform(&obj.pos, &obj.direction, obj_config, world_info);
    let move_comp = MoveComponent::new(speed.unwrap_or(0.0));
    let weapon_comp = create_weapon(main_body, obj_config, game_lib, commands)?;
    let hp_comp = create_hp_comp(obj_config)?;
    let ai_comp = create_ai_comp(obj_config, game_lib)?;
    let mut cmd = commands.entity(main_body);

    add_obj(main_body, &obj, obj_config, world_info, game_map);

    cmd.insert(obj);
    cmd.insert(transform);
    cmd.insert(AIBot);
    cmd.insert(move_comp);
    cmd.insert(weapon_comp);
    cmd.insert(hp_comp);
    cmd.insert(ai_comp);

    if visible {
        cmd.insert(InView);
    }

    Ok(())
}

fn create_tile(
    obj: GameObj,
    obj_config: &GameObjConfig,
    world_info: &mut WorldInfo,
    game_map: &mut GameMap,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<(), MyError> {
    let visible = world_info.check_pos_visible(&obj.pos);
    let entity = create_main_body(obj_config, visible, game_lib, commands)?;
    let transform = create_transform(&obj.pos, &obj.direction, obj_config, world_info);
    let mut cmd = commands.entity(entity);

    add_obj(entity, &obj, obj_config, world_info, game_map);

    cmd.insert(obj);
    cmd.insert(transform);
    cmd.insert(TileComponent);

    Ok(())
}

fn create_missile(
    obj: GameObj,
    speed: Option<f32>,
    obj_config: &GameObjConfig,
    world_info: &mut WorldInfo,
    game_map: &mut GameMap,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<(), MyError> {
    if !world_info.check_pos_visible(&obj.pos) {
        return Ok(());
    }
    let entity = create_main_body(obj_config, true, game_lib, commands)?;
    let transform = create_transform(&obj.pos, &obj.direction, obj_config, world_info);
    let move_comp = MoveComponent::new(speed.unwrap_or(obj_config.speed));
    let mut cmd = commands.entity(entity);

    add_obj(entity, &obj, obj_config, world_info, game_map);

    cmd.insert(obj);
    cmd.insert(transform);
    cmd.insert(MissileComponent);
    cmd.insert(move_comp);

    Ok(())
}

fn create_explosion(
    obj: GameObj,
    obj_config: &GameObjConfig,
    world_info: &mut WorldInfo,
    game_map: &mut GameMap,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<(), MyError> {
    if !world_info.check_pos_visible(&obj.pos) {
        return Ok(());
    }

    let image = game_lib.get_image(&obj_config.image)?;
    let Some(play_config) = obj_config.play_config.as_ref() else {
        let msg = "Missing PlayConfig in GameObjConfig".to_string();
        error!(msg);
        return Err(MyError::Other(msg));
    };
    let layout = game_lib.get_tex_atlas_layout(&obj_config.name)?;
    let transform = create_transform(&obj.pos, &obj.direction, obj_config, world_info);
    let entity = commands
        .spawn((
            Sprite::from_atlas_image(image, TextureAtlas { layout, index: 0 }),
            transform,
            Visibility::Visible,
            PlayComponent::new(play_config),
            ExplosionComponent,
        ))
        .id();

    add_obj(entity, &obj, obj_config, world_info, game_map);

    commands.entity(entity).insert(obj);

    Ok(())
}

fn create_main_body(
    obj_config: &GameObjConfig,
    visible: bool,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<Entity, MyError> {
    let image = game_lib.get_image(&obj_config.image)?;
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
            visibility,
        ))
        .id();

    Ok(entity)
}

fn create_transform(
    pos: &Vec2,
    direction: &Vec2,
    obj_config: &GameObjConfig,
    world_info: &WorldInfo,
) -> Transform {
    let screen_pos = world_info.get_screen_pos(pos);
    Transform {
        translation: Vec3::new(screen_pos.x, screen_pos.y, obj_config.z),
        rotation: get_rotation(direction),
        ..default()
    }
}

fn create_weapon(
    main_body: Entity,
    obj_config: &GameObjConfig,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<WeaponComponent, MyError> {
    let Some(weapon_config) = obj_config.weapon_config.as_ref() else {
        let msg = "Failed to create ai_bot: WeaponConfig not specified".to_string();
        error!(msg);
        return Err(MyError::Other(msg));
    };

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
    obj: &GameObj,
    obj_config: &GameObjConfig,
    world_info: &mut WorldInfo,
    game_map: &mut GameMap,
) {
    game_map.add(&obj.map_pos, entity);
    world_info.update_max_collide_span(obj_config.collide_span);
}

fn create_hp_comp(obj_config: &GameObjConfig) -> Result<HPComponent, MyError> {
    let Some(hp) = obj_config.hp else {
        let msg = "Failed to create HPComponent: HP is missing".to_string();
        error!(msg);
        return Err(MyError::NotFound(msg));
    };

    Ok(HPComponent::new(hp))
}

fn create_ai_comp(obj_config: &GameObjConfig, game_lib: &GameLib) -> Result<AIComponent, MyError> {
    let Some(ai_config_name) = obj_config.ai.as_ref() else {
        let msg = "AI config is missing".to_string();
        error!(msg);
        return Err(MyError::Other(msg));
    };
    let ai_config = game_lib.get_ai_config(ai_config_name)?;

    let ai_comp = AIComponent::new(ai_config);

    Ok(ai_comp)
}
