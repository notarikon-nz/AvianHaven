use bevy::prelude::*;
use crate::journal::{components::*, resources::*};
use crate::photo_mode::components::PhotoTakenEvent;
use crate::despawn::SafeDespawn;

pub fn load_education_data(mut education_data: ResMut<BirdEducationData>) {
    education_data.load_from_files();
}

pub fn toggle_journal_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<crate::AppState>>,
    current_state: Res<State<crate::AppState>>,
    mut journal_state: ResMut<JournalState>,
) {
    if keyboard.just_pressed(KeyCode::KeyJ) {
        info!("🔵 JOURNAL TOGGLE: J key pressed in state: {:?}", current_state.get());
        match current_state.get() {
            crate::AppState::Playing => {
                info!("🔵 JOURNAL TOGGLE: Opening journal, transitioning Playing -> Journal");
                journal_state.is_open = true;
                next_state.set(crate::AppState::Journal);
            }
            crate::AppState::Journal => {
                info!("🔵 JOURNAL TOGGLE: Closing journal, transitioning Journal -> Playing");
                journal_state.is_open = false;
                next_state.set(crate::AppState::Playing);
            }
            _ => {
                info!("🔵 JOURNAL TOGGLE: J key pressed but in invalid state: {:?}", current_state.get());
            }
        }
    }
}

pub fn setup_journal_menu_system(
    mut commands: Commands,
    discovered: Res<DiscoveredSpecies>,
    journal_state: Res<JournalState>,
    education_data: Res<BirdEducationData>,
) {
    // Main journal container - field notebook style
    commands.spawn((
        Node {
            width: Val::Percent(90.0),
            height: Val::Percent(85.0),
            position_type: PositionType::Absolute,
            left: Val::Percent(5.0),
            top: Val::Percent(7.5),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgb(0.95, 0.92, 0.88)), // Aged paper color
        JournalMenu,
        JournalBackground,
    )).with_children(|journal| {
        // Journal header with binding rings effect
        journal.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                border: UiRect::bottom(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.9, 0.87, 0.82)),
            BorderColor(Color::srgb(0.7, 0.6, 0.5)),
            BorderRadius::all(Val::Px(6.0)),
        )).with_children(|header| {
            header.spawn((
                Text::new("Field Journal"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.3, 0.2, 0.1)),
            ));
            
            // Close button
            header.spawn((
                Button,
                Node {
                    width: Val::Px(40.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.8, 0.4, 0.4)),
                CloseButton,
            )).with_children(|button| {
                button.spawn((
                    Text::new("×"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });
        
        // Tab navigation
        journal.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                flex_direction: FlexDirection::Row,
                border: UiRect::bottom(Val::Px(1.0)),
                ..default()
            },
            BorderColor(Color::srgb(0.6, 0.5, 0.4)),
        )).with_children(|tabs| {
            let tab_configs = [
                (JournalTab::Species, "Species", "Discovered bird species"),
                (JournalTab::Photos, "Photos", "Photo collection"),
                (JournalTab::Conservation, "Conservation", "Species status & protection"),
                (JournalTab::Migration, "Migration", "Migration patterns & routes"),
                (JournalTab::Research, "Research", "Active research missions"),
                (JournalTab::Achievements, "Progress", "Achievements & milestones"),
            ];
            
            for (tab, title, _tooltip) in tab_configs {
                let is_active = tab == journal_state.current_tab;
                let (bg_color, text_color) = if is_active {
                    (Color::srgb(0.95, 0.92, 0.88), Color::srgb(0.2, 0.1, 0.05))
                } else {
                    (Color::srgb(0.85, 0.82, 0.78), Color::srgb(0.4, 0.3, 0.2))
                };
                
                tabs.spawn((
                    Button,
                    Node {
                        width: Val::Percent(20.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::right(Val::Px(1.0)),
                        ..default()
                    },
                    BackgroundColor(bg_color),
                    BorderColor(Color::srgb(0.6, 0.5, 0.4)),
                    JournalTabButton { tab },
                )).with_children(|button| {
                    button.spawn((
                        Text::new(title),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(text_color),
                    ));
                });
            }
        });
        
        // Main content area
        journal.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                overflow: Overflow::scroll_y(),
                ..default()
            },
            JournalTabContent { tab: journal_state.current_tab },
        )).with_children(|content| {
            match journal_state.current_tab {
                JournalTab::Species => {
                    // Species tab content
                    content.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(15.0),
                            ..default()
                        },
                    )).with_children(|species_content| {
                        species_content.spawn((
                            Text::new(format!("Species Discovered: {}", discovered.0.len())),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.3, 0.2, 0.1)),
                            Node {
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                        ));
                        
                        // Species grid
                        species_content.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Row,
                                flex_wrap: FlexWrap::Wrap,
                                row_gap: Val::Px(10.0),
                                column_gap: Val::Px(10.0),
                                ..default()
                            },
                        )).with_children(|grid| {
                            for species in &discovered.0 {
                                let facts = education_data.species_facts.get(species);
                                let conservation = education_data.conservation_status.get(species)
                                    .unwrap_or(&ConservationStatus::LeastConcern);
                                
                                grid.spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(280.0),
                                        height: Val::Px(120.0),
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::SpaceBetween,
                                        padding: UiRect::all(Val::Px(15.0)),
                                        border: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.98, 0.95, 0.92)),
                                    BorderColor(conservation.color()),
                                    SpeciesCard { species: *species },
                                )).with_children(|card| {
                                    // Species name
                                    card.spawn((
                                        Text::new(if let Some(facts) = facts {
                                            &facts.common_name
                                        } else {
                                            "Unknown Species"
                                        }),
                                        TextFont {
                                            font_size: 16.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.2, 0.1, 0.05)),
                                    ));
                                    
                                    // Scientific name
                                    card.spawn((
                                        Text::new(if let Some(facts) = facts {
                                            &facts.scientific_name
                                        } else {
                                            "Species unknown"
                                        }),
                                        TextFont {
                                            font_size: 12.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.4, 0.3, 0.2)),
                                    ));
                                    
                                    // Conservation status badge
                                    card.spawn((
                                        Node {
                                            width: Val::Px(100.0),
                                            height: Val::Px(20.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BackgroundColor(conservation.color()),
                                        ConservationStatusBadge,
                                    )).with_children(|badge| {
                                        badge.spawn((
                                            Text::new(conservation.label()),
                                            TextFont {
                                                font_size: 10.0,
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                        ));
                                    });
                                });
                            }
                        });
                    });
                },
                JournalTab::Photos => {
                    // Photos tab content
                    content.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                    )).with_children(|photos_content| {
                        photos_content.spawn((
                            Text::new("Photo Gallery"),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.3, 0.2, 0.1)),
                        ));
                        
                        photos_content.spawn((
                            Text::new("Your best bird photographs will appear here.\nTake photos in Photo Mode (P) to build your collection."),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.5, 0.4, 0.3)),
                            Node {
                                margin: UiRect::top(Val::Px(20.0)),
                                ..default()
                            },
                        ));
                    });
                },
                JournalTab::Conservation => {
                    // Conservation tab content
                    content.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(15.0),
                            ..default()
                        },
                    )).with_children(|conservation_content| {
                        conservation_content.spawn((
                            Text::new("Conservation Status"),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.3, 0.2, 0.1)),
                            Node {
                                margin: UiRect::bottom(Val::Px(15.0)),
                                ..default()
                            },
                        ));
                        
                        // Conservation status breakdown
                        let mut status_counts = std::collections::HashMap::new();
                        for species in &discovered.0 {
                            let status = education_data.conservation_status.get(species)
                                .unwrap_or(&ConservationStatus::LeastConcern);
                            *status_counts.entry(*status).or_insert(0) += 1;
                        }
                        
                        conservation_content.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(8.0),
                                ..default()
                            },
                        )).with_children(|status_list| {
                            for (status, count) in status_counts {
                                status_list.spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(30.0),
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceBetween,
                                        align_items: AlignItems::Center,
                                        padding: UiRect::all(Val::Px(10.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgba(0.9, 0.9, 0.9, 0.5)),
                                )).with_children(|row| {
                                    row.spawn((
                                        Text::new(status.label()),
                                        TextFont {
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(status.color()),
                                    ));
                                    
                                    row.spawn((
                                        Text::new(format!("{} species", count)),
                                        TextFont {
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.4, 0.3, 0.2)),
                                    ));
                                });
                            }
                        });
                        
                        // Educational text
                        conservation_content.spawn((
                            Text::new("Learn about conservation efforts and how you can help protect bird species in your area. Each species' conservation status is based on IUCN Red List data."),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.5, 0.4, 0.3)),
                            Node {
                                margin: UiRect::top(Val::Px(20.0)),
                                ..default()
                            },
                        ));
                    });
                },
                JournalTab::Migration => {
                    // Migration tab content - show migratory species with educational data
                    content.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(15.0),
                            ..default()
                        },
                    )).with_children(|migration_content| {
                        migration_content.spawn((
                            Text::new("Migration Patterns"),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.3, 0.2, 0.1)),
                            Node {
                                margin: UiRect::bottom(Val::Px(15.0)),
                                ..default()
                            },
                        ));
                        
                        // Migration statistics
                        let migratory_count = discovered.0.iter()
                            .filter_map(|species| education_data.migration_data.get(species))
                            .filter(|data| data.is_migratory)
                            .count();
                        let resident_count = discovered.0.len() - migratory_count;
                        
                        migration_content.spawn((
                            Text::new(format!("Migratory: {} | Resident: {}", migratory_count, resident_count)),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.4, 0.3, 0.2)),
                        ));
                    });
                },
                JournalTab::Research => {
                    // Research missions tab content
                    content.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(15.0),
                            padding: UiRect::all(Val::Px(20.0)),
                            ..default()
                        },
                    )).with_children(|research_content| {
                        research_content.spawn((
                            Text::new("Active Research Missions"),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.3, 0.2, 0.1)),
                        ));
                        
                        research_content.spawn((
                            Text::new("Contribute to citizen science by completing research missions.\nEarn research points and unlock advanced content."),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.5, 0.4, 0.3)),
                            Node {
                                margin: UiRect::bottom(Val::Px(15.0)),
                                ..default()
                            },
                        ));
                        
                        // Placeholder for research missions list
                        research_content.spawn((
                            Text::new("Dawn Chorus Study (Citizen Level)\n   Progress: 0/10 observations\n   Partner: eBird/Cornell Lab\n\n📋 Feeder Interaction Study (Student Level)\n   Progress: 0/20 interactions documented\n   Partner: Project FeederWatch\n\n📋 Climate Impact Assessment (Researcher Level)\n   Progress: 0/100 data points\n   Partner: Audubon Climate Watch"),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.4, 0.3, 0.2)),
                        ));
                    });
                },
                JournalTab::Achievements => {
                    // Achievements tab content
                    content.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                    )).with_children(|achievements_content| {
                        achievements_content.spawn((
                            Text::new("Achievements & Progress"),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.3, 0.2, 0.1)),
                        ));
                        
                        achievements_content.spawn((
                            Text::new("Your birding achievements and milestones will appear here.\\nComplete goals to unlock rewards and new content."),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.5, 0.4, 0.3)),
                            Node {
                                margin: UiRect::top(Val::Px(20.0)),
                                ..default()
                            },
                        ));
                    });
                }
            }
        });
    });
}

pub fn teardown_journal_menu_system(
    mut commands: Commands,
    journal_query: Query<Entity, With<JournalMenu>>,
) {
    for entity in journal_query.iter() {
        commands.entity(entity).safe_despawn();
    }
}

pub fn journal_tab_system(
    mut interaction_query: Query<
        (&Interaction, &JournalTabButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut journal_state: ResMut<JournalState>,
    mut commands: Commands,
    content_query: Query<Entity, With<JournalTabContent>>,
    discovered: Res<DiscoveredSpecies>,
    education_data: Res<BirdEducationData>,
) {
    for (interaction, tab_button, mut bg_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = Color::srgb(0.95, 0.92, 0.88).into();
                
                if journal_state.current_tab != tab_button.tab {
                    journal_state.current_tab = tab_button.tab;
                    
                    // Refresh content area
                    for entity in content_query.iter() {
                        commands.entity(entity).safe_despawn();
                    }
                    
                    // Note: In a full implementation, we'd re-spawn the content here
                    // For now, the content refresh happens in the next frame
                }
            }
            Interaction::Hovered => {
                if journal_state.current_tab != tab_button.tab {
                    *bg_color = Color::srgb(0.9, 0.87, 0.83).into();
                }
            }
            Interaction::None => {
                if journal_state.current_tab != tab_button.tab {
                    *bg_color = Color::srgb(0.85, 0.82, 0.78).into();
                }
            }
        }
    }
}

pub fn journal_species_detail_system(
    mut interaction_query: Query<
        (&Interaction, &SpeciesCard),
        (Changed<Interaction>, With<Button>),
    >,
    mut journal_state: ResMut<JournalState>,
) {
    for (interaction, species_card) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            journal_state.selected_species = Some(species_card.species);
        }
    }
}

pub fn update_journal_on_discovery_system(
    mut discovered: ResMut<DiscoveredSpecies>,
    mut photo_events: EventReader<PhotoTakenEvent>,
) {
    for photo_event in photo_events.read() {
        if let Some(species) = photo_event.species {
            discovered.0.insert(species);
        }
    }
}

pub fn journal_interaction_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<CloseButton>)>,
    mut next_state: ResMut<NextState<crate::AppState>>,
    mut journal_state: ResMut<JournalState>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            journal_state.is_open = false;
            next_state.set(crate::AppState::Playing);
        }
    }
}

// Research missions setup system
pub fn setup_research_missions(mut research_manager: ResMut<ResearchMissionManager>) {
    research_manager.active_missions = ResearchMissionManager::generate_starter_missions();
    info!("Initialized {} research missions", research_manager.active_missions.len());
}