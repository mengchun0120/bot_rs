use crate::misc::states::AppState;
use bevy::prelude::*;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

pub fn splash_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Splash), splash_setup)
        .add_systems(Update, countdown.run_if(in_state(AppState::Splash)));
}

fn splash_setup(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(AppState::Splash),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Text::new("Loading ..."),
            TextFont {
                font_size: 67.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
            Node {
                margin: UiRect::all(px(50)),
                ..default()
            },
        )],
    ));
    commands.insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

fn countdown(
    mut app_state: ResMut<NextState<AppState>>,
    mut splash_timer: ResMut<SplashTimer>,
    mut commands: Commands,
    time: Res<Time>,
) {
    if splash_timer.tick(time.delta()).is_finished() {
        commands.remove_resource::<SplashTimer>();
        app_state.set(AppState::Menu);
    }
}
