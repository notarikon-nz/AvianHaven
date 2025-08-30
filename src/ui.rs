use bevy::prelude::*;

use crate::{AppState, resources::{BirdCount, SpawnBirdEvent}};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(
                Update,
                (
                    button_interaction,
                    update_bird_counter,
                ).run_if(in_state(AppState::Playing))
            );
    }
}

#[derive(Component)]
struct SpawnButton;

#[derive(Component)]
struct BirdCounterText;

fn setup_ui(mut commands: Commands) {
    // Root UI container
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
    )).with_children(|parent| {
        // Title
        parent.spawn((
            Text::new("Avian Haven Prototype"),
            TextFont {
                font_size: 48.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
        
        // Bottom UI container
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(20.0),
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