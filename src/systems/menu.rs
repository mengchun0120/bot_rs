use crate::misc::states::AppState;
use bevy::prelude::*;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);

#[derive(Component)]
enum MenuAction {
    PlayGame,
    Exit,
}

#[derive(Resource)]
struct HoveredButton(Option<Entity>);

pub fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Menu), setup_menu)
        .add_systems(OnExit(AppState::Menu), cleanup)
        .add_systems(Update, menu_action.run_if(in_state(AppState::Menu)));
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
                    MenuAction::PlayGame,
                    children![(
                        Text::new("Play Game"),
                        button_text_font.clone(),
                        TextColor(TEXT_COLOR),
                    )]
                ),
                (
                    Button,
                    button_node.clone(),
                    MenuAction::Exit,
                    children![(
                        Text::new("Exit"),
                        button_text_font.clone(),
                        TextColor(TEXT_COLOR),
                    )]
                )
            ]
        )],
    ));
    commands.insert_resource(HoveredButton(None));
}

fn menu_action(
    interaction_query: Query<
        (Entity, &Interaction, &MenuAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut background_query: Query<&mut BackgroundColor>,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
    mut prev_hovered: ResMut<HoveredButton>,
) {
    for (entity, interaction, action) in interaction_query.iter() {
        match *interaction {
            Interaction::Pressed => match *action {
                MenuAction::PlayGame => {
                    app_state.set(AppState::Game);
                }
                MenuAction::Exit => {
                    app_exit_writer.write(AppExit::Success);
                }
            },
            Interaction::Hovered => {
                let Ok(mut cur_background) = background_query.get_mut(entity) else {
                    return;
                };
                *cur_background = HOVERED_BUTTON.into();

                if let Some(prev_button) = prev_hovered.0
                    && prev_button != entity
                {
                    let Ok(mut prev_background) = background_query.get_mut(prev_button) else {
                        return;
                    };
                    *prev_background = BackgroundColor::default();
                }

                prev_hovered.0 = Some(entity);
            }
            _ => {}
        }
    }
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<HoveredButton>();
}
