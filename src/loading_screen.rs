use bevy::prelude::*;

// Style constants for easy adjustment
const BACKGROUND_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const PROGRESS_BAR_BACKGROUND: Color = Color::srgb(0.3, 0.3, 0.3);
const PROGRESS_BAR_FILL: Color = Color::srgb(0.2, 0.6, 0.8);
const TEXT_FONT_SIZE: f32 = 36.0;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LoadingProgress>()
            .add_systems(OnEnter(crate::AppState::LoadingScreen), setup_loading_screen)
            .add_systems(OnExit(crate::AppState::LoadingScreen), cleanup_loading_screen)
            .add_systems(
                Update,
                (update_loading_progress, update_progress_bar).run_if(in_state(crate::AppState::LoadingScreen))
            );
    }
}

#[derive(Resource, Default)]
struct LoadingProgress {
    progress: f32,
    timer: Timer,
}

#[derive(Component)]
struct LoadingScreenRoot;

#[derive(Component)]
struct LoadingText;

#[derive(Component)]
struct ProgressBar;

#[derive(Component)]
struct ProgressBarFill;

fn setup_loading_screen(mut commands: Commands) {
    // Root container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(BACKGROUND_COLOR),
            LoadingScreenRoot,
        ))
        .with_children(|parent| {
            // Loading text
            parent.spawn((
                Text::new("Loading..."),
                TextFont {
                    font_size: TEXT_FONT_SIZE,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
                LoadingText,
            ));

            // Progress bar container
            parent
                .spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(20.0),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(PROGRESS_BAR_BACKGROUND),
                    BorderColor(TEXT_COLOR),
                    ProgressBar,
                ))
                .with_children(|parent| {
                    // Progress bar fill
                    parent.spawn((
                        Node {
                            width: Val::Percent(0.0), // Will be updated
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(PROGRESS_BAR_FILL),
                        ProgressBarFill,
                    ));
                });
        });
}

fn update_loading_progress(
    time: Res<Time>,
    mut loading_progress: ResMut<LoadingProgress>,
    mut next_state: ResMut<NextState<crate::AppState>>,
) {
    // Initialize timer if not set
    if loading_progress.timer.duration().as_secs_f32() == 0.0 {
        loading_progress.timer = Timer::from_seconds(2.0, TimerMode::Once);
    }
    
    loading_progress.timer.tick(time.delta());
    loading_progress.progress = loading_progress.timer.fraction();

    // Transition to main menu when loading is complete
    if loading_progress.timer.finished() {
        next_state.set(crate::AppState::MainMenu);
    }
}

fn update_progress_bar(
    loading_progress: Res<LoadingProgress>,
    mut progress_bar_query: Query<&mut Node, With<ProgressBarFill>>,
) {
    if let Ok(mut style) = progress_bar_query.single_mut() {
        style.width = Val::Percent(loading_progress.progress * 100.0);
    }
}

fn cleanup_loading_screen(
    mut commands: Commands,
    loading_screen_query: Query<Entity, With<LoadingScreenRoot>>,
) {
    for entity in loading_screen_query.iter() {
        commands.entity(entity).despawn();
    }
}