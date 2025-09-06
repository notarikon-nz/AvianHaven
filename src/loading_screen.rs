use bevy::prelude::*;
use crate::user_interface::progress_bar::*;

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
                (update_loading_progress, update_custom_progress_bar).run_if(in_state(crate::AppState::LoadingScreen))
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
    let root_entity = commands
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
        )).id();

    // Loading text
    let text_entity = commands.spawn((
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
    )).id();

    // Custom Progress Bar using the new ProgressBarBuilder
    let progress_bar_visuals = ProgressBarVisuals {
        track_color: PROGRESS_BAR_BACKGROUND,
        fill_color: PROGRESS_BAR_FILL,
        border_color: TEXT_COLOR,
        text_color: TEXT_COLOR,
        show_text: false,
        animation_duration: 0.2, // Smooth animation over 0.2 seconds
        easing: EasingFunction::EaseOut,
        ..default()
    };
    
    let progress_bar_entity = ProgressBarBuilder::new(&mut commands)
        .with_value(0.0, 1.0) // Use 0.0-1.0 range for simpler fraction handling
        .with_size(Val::Px(300.0), Val::Px(20.0))
        .with_visuals(progress_bar_visuals)
        .with_animation(true)
        .spawn();
    
    // Add children to root
    commands.entity(root_entity).add_children(&[text_entity, progress_bar_entity]);
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

fn update_custom_progress_bar(
    loading_progress: Res<LoadingProgress>,
    mut progress_bar_query: Query<&mut crate::user_interface::progress_bar::ProgressBar, With<ProgressBarRoot>>,
) {
    if let Ok(mut progress_bar) = progress_bar_query.single_mut() {
        // loading_progress.progress is already 0.0-1.0 from timer.fraction()
        progress_bar.set_value(loading_progress.progress);
    }
}

// OLD PROGRESS BAR UPDATE (commented out - replaced by custom progress bar)
/*
fn update_progress_bar(
    loading_progress: Res<LoadingProgress>,
    mut progress_bar_query: Query<&mut Node, With<ProgressBarFill>>,
) {
    if let Ok(mut style) = progress_bar_query.single_mut() {
        style.width = Val::Percent(loading_progress.progress * 100.0);
    }
}
*/

fn cleanup_loading_screen(
    mut commands: Commands,
    loading_screen_query: Query<Entity, With<LoadingScreenRoot>>,
) {
    for entity in loading_screen_query.iter() {
        commands.entity(entity).despawn();
    }
}