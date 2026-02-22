use crate::misc::*;
use bevy::prelude::*;

const DIALOG_BACKGROUND_COLOR: Color = Color::srgba(0.3, 0.3, 0.3, 0.9);
const TITLE_BACKGROUND_COLOR: Color = Color::srgba(0.1, 0.1, 0.1, 1.0);
const BUTTON_BACKGROUND_COLOR: Color = Color::srgba(0.5, 0.5, 0.5, 1.0);

pub fn gameover(
    mut commands: Commands,
    game_info: Res<GameInfo>,
) {
    let msg = match game_info.game_result() {
        GameResult::Win => "You won".to_string(),
        GameResult::Fail => "You failed".to_string(),
        _ => {
            warn!("Game over with wrong result");
            return;
        }
    };
    commands.spawn((
        DespawnOnExit(GameState::GameOver),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }
    ))
    .insert(children![(
        Node {
            flex_direction: FlexDirection::Column,
            width: px(200),
            height: px(150),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(DIALOG_BACKGROUND_COLOR.into()),
        children![
            (
                Node {
                    width: px(200),
                    height: px(25),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(TITLE_BACKGROUND_COLOR.into()),
                children![
                    (
                        Text::new("Game Over"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                    )
                ]
            ),
            (
                Node {
                    width: px(200),
                    height: px(90),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![
                    (
                        Text(msg),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        }
                    )
                ]
            ),
            (
                Node {
                    width: px(80),
                    height: px(20),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(BUTTON_BACKGROUND_COLOR.into()),
                children![
                    (
                        Button,
                        Text::new("OK"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        }
                    )
                ]
            ),
            (
                Node {
                    height: px(15),
                    ..default()
                },
            )
        ]
    )]);
}

pub fn wait_gameover(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<NextState<GameState>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            game_state.set(GameState::End);
            app_state.set(AppState::Menu);
        }
    }
}