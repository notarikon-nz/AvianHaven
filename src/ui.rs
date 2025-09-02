use bevy::prelude::*;

use crate::{AppState, resources::{BirdCount, SpawnBirdEvent}};
use crate::environment::resources::{TimeState, WeatherState};
use crate::photo_mode::resources::CurrencyResource;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), setup_ui)
            .add_systems(OnExit(AppState::Playing), cleanup_gameplay_ui)
            .add_systems(
                Update,
                (
                    button_interaction,
                    update_bird_counter,
                    update_environment_ui,
                    update_currency_ui,
                ).run_if(in_state(AppState::Playing))
            );
    }
}

#[derive(Component)]
struct SpawnButton;

#[derive(Component)]
struct BirdCounterText;

#[derive(Component)]
struct EnvironmentText;

#[derive(Component)]
struct CurrencyText;

fn setup_ui(mut commands: Commands) {
    // Bottom UI container (non-blocking)
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Auto,
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(0.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: Val::Px(20.0),
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
    )).with_children(|parent| {
            // Bird counter
            parent.spawn((
                Text::new("Birds: 0"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                BirdCounterText,
            ));
            
            // Environment info
            parent.spawn((
                Text::new("Spring | 08:00 | Clear (20°C)"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                EnvironmentText,
            ));
            
            // Currency display
            parent.spawn((
                Text::new("Currency: 0"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.6, 0.2)),
                CurrencyText,
            ));
            
            // Spawn bird button
            parent.spawn((
                Button,
                SpawnButton,
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.6, 0.2)),
                BorderColor(Color::srgb(0.1, 0.4, 0.1)),
            )).with_children(|parent| {
                parent.spawn((
                    Text::new("Spawn Bird"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });
    }

fn button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SpawnButton>),
    >,
    mut spawn_events: EventWriter<SpawnBirdEvent>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.1, 0.4, 0.1).into();
                spawn_events.write(SpawnBirdEvent);
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.25, 0.65, 0.25).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.2, 0.6, 0.2).into();
            }
        }
    }
}

fn update_bird_counter(
    bird_count: Res<BirdCount>,
    mut text_query: Query<&mut Text, With<BirdCounterText>>,
) {
    if bird_count.is_changed() {
        for mut text in text_query.iter_mut() {
            **text = format!("Birds: {}", bird_count.0);
        }
    }
}

fn update_environment_ui(
    time_state: Res<TimeState>,
    weather_state: Res<WeatherState>,
    mut text_query: Query<&mut Text, With<EnvironmentText>>,
) {
    if time_state.is_changed() || weather_state.is_changed() {
        for mut text in text_query.iter_mut() {
            let season = time_state.get_season();
            let hour = time_state.hour as u32;
            let minute = ((time_state.hour - hour as f32) * 60.0) as u32;
            
            **text = format!(
                "{:?} | {:02}:{:02} | {:?} ({}°C)", 
                season, hour, minute, weather_state.current_weather, weather_state.temperature as i32
            );
        }
    }
}

fn update_currency_ui(
    currency: Res<CurrencyResource>,
    mut text_query: Query<&mut Text, With<CurrencyText>>,
) {
    if currency.is_changed() {
        for mut text in text_query.iter_mut() {
            **text = format!("Currency: {}", currency.0);
        }
    }
}

fn cleanup_gameplay_ui(
    mut commands: Commands,
    ui_query: Query<Entity, Or<(
        With<SpawnButton>, 
        With<BirdCounterText>, 
        With<EnvironmentText>, 
        With<CurrencyText>
    )>>,
) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}