use crate::config::*;
use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;

pub fn create_obj_by_config(
    map_obj_config: &GameMapObjConfig,
    world_info: &mut WorldInfo,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
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
    world_info: &mut WorldInfo,
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

    let obj_config = game_lib.get_game_obj_config(config_index);
    match obj_config.obj_type {
        GameObjType::Bot => create_bot(
            config_index,
            pos,
            direction,
            speed,
            obj_config,
            world_info,
            game_map,
            game_obj_lib,
            game_lib,
            commands,
        ),
        GameObjType::Tile => create_tile(
            config_index,
            pos,
            direction,
            obj_config,
            world_info,
            game_map,
            game_obj_lib,
            game_lib,
            commands,
        ),
        GameObjType::Missile => create_missile(
            config_index,
            pos,
            direction,
            speed,
            obj_config,
            world_info,
            game_map,
            game_obj_lib,
            game_lib,
            commands,
        ),
        GameObjType::Explosion => create_explosion(
            config_index,
            pos,
            direction,
            obj_config,
            world_info,
            game_map,
            game_obj_lib,
            game_lib,
            commands,
        ),
    }
}

fn create_bot(
    config_index: usize,
    pos: Vec2,
    direction: Vec2,
    speed: Option<f32>,
    obj_config: &GameObjConfig,
    world_info: &mut WorldInfo,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<(), MyError> {
    match obj_config.side {
        GameObjSide::Player => create_player(
            config_index,
            pos,
            direction,
            speed,
            obj_config,
            world_info,
            game_map,
            game_obj_lib,
            game_lib,
            commands,
        ),
        GameObjSide::AI => create_ai_bot(
            config_index,
            pos,
            direction,
            speed,
            obj_config,
            world_info,
            game_map,
            game_obj_lib,
            game_lib,
            commands,
        ),
        GameObjSide::Neutral => {
            let msg = "Cannot create netural bot".to_string();
            error!(msg);
            Err(MyError::Other(msg))
        }
    }
}

fn create_player(
    config_index: usize,
    pos: Vec2,
    direction: Vec2,
    speed: Option<f32>,
    obj_config: &GameObjConfig,
    world_info: &mut WorldInfo,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<(), MyError> {
    let visible = world_info.check_pos_visible(&pos);
    let main_body = create_main_body(obj_config, visible, game_lib, commands)?;
    let transform = create_transform(&pos, &direction, obj_config, world_info);
    let move_comp = MoveComponent::new(speed.unwrap_or(0.0));
    let mut cmd = commands.entity(main_body);

    cmd.insert(transform);
    cmd.insert(Player);
    cmd.insert(move_comp);
    create_weapon(main_body, obj_config, game_lib, commands)?;

    add_obj(
        main_body,
        config_index,
        pos,
        direction,
        obj_config.hp,
        obj_config,
        world_info,
        game_map,
        game_obj_lib,
    );

    Ok(())
}

fn create_ai_bot(
    config_index: usize,
    pos: Vec2,
    direction: Vec2,
    speed: Option<f32>,
    obj_config: &GameObjConfig,
    world_info: &mut WorldInfo,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<(), MyError> {
    let visible = world_info.check_pos_visible(&pos);
    let main_body = create_main_body(obj_config, visible, game_lib, commands)?;
    let transform = create_transform(&pos, &direction, obj_config, world_info);
    let move_comp = MoveComponent::new(speed.unwrap_or(0.0));
    let mut cmd = commands.entity(main_body);

    cmd.insert(transform);
    cmd.insert(AIBot);
    cmd.insert(move_comp);
    create_weapon(main_body, obj_config, game_lib, commands)?;

    add_obj(
        main_body,
        config_index,
        pos,
        direction,
        obj_config.hp,
        obj_config,
        world_info,
        game_map,
        game_obj_lib,
    );

    Ok(())
}

fn create_tile(
    config_index: usize,
    pos: Vec2,
    direction: Vec2,
    obj_config: &GameObjConfig,
    world_info: &mut WorldInfo,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<(), MyError> {
    let visible = world_info.check_pos_visible(&pos);
    let entity = create_main_body(obj_config, visible, game_lib, commands)?;
    let transform = create_transform(&pos, &direction, obj_config, world_info);
    let mut cmd = commands.entity(entity);

    cmd.insert(transform);
    cmd.insert(TileComponent);

    add_obj(
        entity,
        config_index,
        pos,
        direction,
        obj_config.hp,
        obj_config,
        world_info,
        game_map,
        game_obj_lib,
    );

    Ok(())
}

fn create_missile(
    config_index: usize,
    pos: Vec2,
    direction: Vec2,
    speed: Option<f32>,
    obj_config: &GameObjConfig,
    world_info: &mut WorldInfo,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<(), MyError> {
    if !world_info.check_pos_visible(&pos) {
        return Ok(());
    }
    let entity = create_main_body(obj_config, true, game_lib, commands)?;
    let transform = create_transform(&pos, &direction, obj_config, world_info);
    let move_comp = MoveComponent::new(speed.unwrap_or(obj_config.speed));
    let mut cmd = commands.entity(entity);

    cmd.insert(transform);
    cmd.insert(MissileComponent);
    cmd.insert(move_comp);

    add_obj(
        entity,
        config_index,
        pos,
        direction,
        obj_config.hp,
        obj_config,
        world_info,
        game_map,
        game_obj_lib,
    );

    Ok(())
}

fn create_explosion(
    config_index: usize,
    pos: Vec2,
    direction: Vec2,
    obj_config: &GameObjConfig,
    world_info: &mut WorldInfo,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<(), MyError> {
    if !world_info.check_pos_visible(&pos) {
        return Ok(());
    }

    let image = game_lib.get_image(&obj_config.image)?;
    let Some(play_config) = obj_config.play_config.as_ref() else {
        let msg = "Missing PlayConfig in GameObjConfig".to_string();
        error!(msg);
        return Err(MyError::Other(msg));
    };
    let layout = game_lib.get_tex_atlas_layout(&obj_config.name)?;
    let transform = create_transform(&pos, &direction, obj_config, world_info);
    let entity = commands
        .spawn((
            Sprite::from_atlas_image(image, TextureAtlas { layout, index: 0 }),
            transform,
            Visibility::Visible,
            PlayComponent::new(play_config),
            ExplosionComponent,
        ))
        .id();

    add_obj(
        entity,
        config_index,
        pos,
        direction,
        obj_config.hp,
        obj_config,
        world_info,
        game_map,
        game_obj_lib,
    );

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
) -> Result<(), MyError> {
    let Some(weapon_config) = obj_config.weapon_config.as_ref() else {
        let msg = "Failed to create ai_bot: WeaponConfig not specified".to_string();
        error!(msg);
        return Err(MyError::Other(msg));
    };

    add_guns(main_body, weapon_config, game_lib, commands)?;

    let weapon_comp = WeaponComponent::new(weapon_config, game_lib)?;
    commands.entity(main_body).insert(weapon_comp);

    Ok(())
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
    hp: Option<f32>,
    obj_config: &GameObjConfig,
    world_info: &mut WorldInfo,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
) {
    let map_pos = game_map.get_map_pos(&pos);
    game_map.add(&map_pos, entity);

    let obj = GameObj {
        config_index,
        pos,
        direction,
        map_pos,
        hp,
    };
    game_obj_lib.insert(entity, obj);

    world_info.update_max_collide_span(obj_config.collide_span);
}
