use crate::misc::AppState;
use bevy::prelude::*;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Menu), setup_menu);
}

fn setup_menu(mut commands: Commands) {
    let button_node = Node {
        width: px(300),
        height: px(65),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_font = TextFont {
        font_size: 33.0,
        ..default()
    };

    commands.spawn((
        DespawnOnExit(AppState::Menu),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            children![
                (
                    Button,
                    button_node.clone(),
                    children![(
                        Text::new("Play Game"),
                        button_text_font.clone(),
                        TextColor(TEXT_COLOR),
                    )]
                ),
                (
                    Button,
                    button_node.clone(),
                    children![(
                        Text::new("Exit"),
                        button_text_font.clone(),
                        TextColor(TEXT_COLOR),
                    )]
                )
            ]
        )],
    ));
}
