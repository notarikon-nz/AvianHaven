use bevy::prelude::*;
use bevy::app::AppExit;
use crate::user_interface::styles::*;
use crate::journal::{components::*, resources::*, ui_builder::*};
use crate::photo_mode::components::PhotoTakenEvent;
use crate::animation::components::BirdSpecies;

pub fn toggle_journal_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<crate::AppState>>,
    current_state: Res<State<crate::AppState>>,
    mut writer: EventWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::KeyJ) {
        match current_state.get() {
            crate::AppState::Playing => next_state.set(crate::AppState::Journal),
            crate::AppState::Journal => next_state.set(crate::AppState::Playing),
        }
    }
    if keyboard.just_pressed(KeyCode::Escape)  {
        match current_state.get() {
            crate::AppState::Playing => { writer.write(AppExit::Success); },
            crate::AppState::Journal => next_state.set(crate::AppState::Playing),
        }
    }
}

pub fn setup_journal_menu_system(
    mut commands: Commands,
    discovered: Res<DiscoveredSpecies>,
    button_style: Res<ButtonStyle>,
    panel_style: Res<PanelStyle>,
) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        JournalMenu,
    )).with_children(|parent| {
        // Title bar
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(80.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(panel_style.background_color),
        )).with_children(|parent| {
            parent.spawn((
                Text::new("Avian Journal"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));


        });
        
        // Main content
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
        )).with_children(|parent| {
            // Left panel - Species grid
            parent.spawn((
                Node {
                    width: Val::Percent(60.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
            )).with_children(|parent| {
                parent.spawn((
                    Node {
                        display: Display::Grid,
                        grid_template_columns: vec![RepeatedGridTrack::flex(5, 1.0)],
                        row_gap: Val::Px(10.0),
                        column_gap: Val::Px(10.0),
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                )).with_children(|parent| {
                    let all_species = vec![
                        BirdSpecies::Cardinal,
                        BirdSpecies::BlueJay, 
                        BirdSpecies::Sparrow,
                        BirdSpecies::Cardinal,
                        BirdSpecies::BlueJay,
                    ];
                    
                    for species in all_species {
                        let is_discovered = discovered.0.contains(&species);
                        parent.spawn((
                            Button,
                            Node {
                                width: button_style.width,
                                height: button_style.height,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(if is_discovered { button_style.normal_color } else { Color::srgb(0.05, 0.05, 0.05) }),
                            BorderColor(button_style.border_color),
                            SpeciesButton(species),
                        )).with_children(|parent| {
                            let color = if is_discovered {
                                species_color(species)
                            } else {
                                Color::srgb(0.2, 0.2, 0.2)
                            };
                            
                            parent.spawn((
                                Node {
                                    width: Val::Px(60.0),
                                    height: Val::Px(60.0),
                                    ..default()
                                },
                                BackgroundColor(color),
                            ));
                            
                            if is_discovered {
                                parent.spawn((
                                    Text::new(&format!("{:?}", species)),
                                    TextFont {
                                        font_size: 12.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            }
                        });
                    }
                });
            });
            
            // Right panel - Detail view
            parent.spawn((
                Node {
                    width: Val::Percent(40.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(panel_style.background_color),
                DetailPanel,
            )).with_children(|parent| {
                parent.spawn((
                    Text::new("Select a species to view details"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.7, 0.7, 0.7)),
                ));
            });
        });
    });
}

pub fn teardown_journal_menu_system(
    mut commands: Commands,
    journal_query: Query<Entity, With<JournalMenu>>,
) {
    for entity in &journal_query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_journal_on_discovery_system(
    mut photo_events: EventReader<PhotoTakenEvent>,
    mut discovered: ResMut<DiscoveredSpecies>,
) {
    for event in photo_events.read() {
        if let Some(species) = event.species {
            discovered.0.insert(species);
        }
    }
}

pub fn journal_interaction_system(
    interaction_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<Button>)>,
    species_query: Query<&SpeciesButton>,
    mut background_query: Query<&mut BackgroundColor>,
    mut journal_data: ResMut<JournalData>,
    detail_query: Query<Entity, With<DetailPanel>>,
    discovered: Res<DiscoveredSpecies>,
    button_style: Res<ButtonStyle>,
    mut next_state: ResMut<NextState<crate::AppState>>,
    mut commands: Commands,
) {
    for (entity, interaction) in interaction_query.iter() {
        if let (Ok(species_button), Ok(mut color)) = (species_query.get(entity), background_query.get_mut(entity)) {

        let is_discovered = discovered.0.contains(&species_button.0);
        
        match *interaction {
            Interaction::Pressed => {
                if is_discovered {
                    journal_data.selected_species = Some(species_button.0);
                    
                    if let Ok(detail_entity) = detail_query.get_single() {
                        commands.entity(detail_entity)
                            .despawn_related::<Children>()
                            .with_children(|parent| {
                                // Add new children here
                                parent.spawn((
                                    Node {
                                        width: Val::Px(200.0),
                                        height: Val::Px(200.0),
                                        margin: UiRect::bottom(Val::Px(20.0)),
                                        ..default()
                                    },
                                    BackgroundColor(species_color(species_button.0)),
                                ));
                                
                                parent.spawn((
                                    Text::new(&format!("{:?}", species_button.0)),
                                    TextFont {
                                        font_size: 24.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });
                    }
                }
            }
            Interaction::Hovered => {
                *color = if is_discovered { button_style.hovered_color } else { Color::srgb(0.1, 0.1, 0.1) }.into();
            }
            Interaction::None => {
                *color = if is_discovered { button_style.normal_color } else { Color::srgb(0.05, 0.05, 0.05) }.into();
            }
        }
    }
}
}
fn species_color(species: BirdSpecies) -> Color {
    match species {
        BirdSpecies::Cardinal => Color::srgb(0.8, 0.2, 0.2),
        BirdSpecies::BlueJay => Color::srgb(0.2, 0.4, 0.8),
        BirdSpecies::Sparrow => Color::srgb(0.5, 0.4, 0.3),
    }
}

fn get_rarity(species: BirdSpecies) -> &'static str {
    match species {
        BirdSpecies::Sparrow => "Common",
        BirdSpecies::Cardinal => "Uncommon",
        BirdSpecies::BlueJay => "Rare",
    }
}