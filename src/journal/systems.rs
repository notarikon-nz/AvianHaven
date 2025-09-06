use bevy::prelude::*;
use crate::journal::{components::*, resources::*};
use crate::photo_mode::components::PhotoTakenEvent;
use crate::photo_mode::resources::PhotoCollection;
use crate::achievements::{AchievementProgress, Achievement};
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
    photo_collection: Res<PhotoCollection>,
    research_manager: Res<ResearchMissionManager>,
    achievement_progress: Res<AchievementProgress>,
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
                    // TODO: NEEDS TO BE A SPRITE HERE NOT A TEXT OBJECT
                    Text::new("x"),
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
                flex_grow: 1.0,
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
                    // Photos tab content - Display actual photo collection
                    content.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(15.0),
                            ..default()
                        },
                    )).with_children(|photos_content| {
                        // Gallery header with stats
                        photos_content.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                margin: UiRect::bottom(Val::Px(15.0)),
                                ..default()
                            },
                        )).with_children(|header| {
                            header.spawn((
                                Text::new("Photo Gallery"),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.3, 0.2, 0.1)),
                            ));
                            
                            header.spawn((
                                Text::new(format!("Photos: {}", photo_collection.photos.len())),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.4, 0.3, 0.2)),
                            ));
                        });
                        
                        if photo_collection.photos.is_empty() {
                            // Empty state message
                            photos_content.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(200.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0.9, 0.87, 0.83, 0.5)),
                            )).with_children(|empty| {
                                empty.spawn((
                                    Text::new("No photos yet!\nPress P to enter Photo Mode and start building your collection."),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.5, 0.4, 0.3)),
                                ));
                            });
                        } else {
                            // Best photos section
                            let best_photos = photo_collection.get_best_photos(6);
                            
                            photos_content.spawn((
                                Text::new("Best Photos"),
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
                            
                            // Photo grid
                            photos_content.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    flex_direction: FlexDirection::Row,
                                    flex_wrap: FlexWrap::Wrap,
                                    row_gap: Val::Px(15.0),
                                    column_gap: Val::Px(15.0),
                                    ..default()
                                },
                            )).with_children(|grid| {
                                for photo in best_photos {
                                    grid.spawn((
                                        Button,
                                        Node {
                                            width: Val::Px(200.0),
                                            height: Val::Px(160.0),
                                            flex_direction: FlexDirection::Column,
                                            border: UiRect::all(Val::Px(2.0)),
                                            padding: UiRect::all(Val::Px(10.0)),
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgb(0.98, 0.95, 0.92)),
                                        BorderColor(Color::srgb(0.7, 0.6, 0.5)),
                                        PhotoCard { timestamp: photo.timestamp },
                                    )).with_children(|card| {
                                        // Photo image placeholder (would show actual image)
                                        card.spawn((
                                            Node {
                                                width: Val::Percent(100.0),
                                                height: Val::Px(100.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            BackgroundColor(Color::srgb(0.8, 0.8, 0.8)),
                                        )).with_children(|img| {
                                            img.spawn((
                                                Text::new("📸"),
                                                TextFont {
                                                    font_size: 32.0,
                                                    ..default()
                                                },
                                                TextColor(Color::srgb(0.5, 0.5, 0.5)),
                                            ));
                                        });
                                        
                                        // Photo metadata
                                        card.spawn((
                                            Node {
                                                width: Val::Percent(100.0),
                                                flex_direction: FlexDirection::Column,
                                                row_gap: Val::Px(5.0),
                                                ..default()
                                            },
                                        )).with_children(|meta| {
                                            // Species name
                                            let species_text = if let Some(species) = photo.species {
                                                format!("{:?}", species)
                                            } else {
                                                "Unknown species".to_string()
                                            };
                                            
                                            meta.spawn((
                                                Text::new(species_text),
                                                TextFont {
                                                    font_size: 12.0,
                                                    ..default()
                                                },
                                                TextColor(Color::srgb(0.3, 0.2, 0.1)),
                                            ));
                                            
                                            // Score and date
                                            meta.spawn((
                                                Text::new(format!("Score: {} | Day {:.0}", 
                                                    photo.score.total_score, photo.timestamp)),
                                                TextFont {
                                                    font_size: 10.0,
                                                    ..default()
                                                },
                                                TextColor(Color::srgb(0.5, 0.4, 0.3)),
                                            ));
                                        });
                                    });
                                }
                            });
                        }
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
                    // Research missions tab content - Display actual missions
                    content.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(15.0),
                            ..default()
                        },
                    )).with_children(|research_content| {
                        // Research header with stats
                        research_content.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                margin: UiRect::bottom(Val::Px(15.0)),
                                ..default()
                            },
                        )).with_children(|header| {
                            header.spawn((
                                Text::new("Research Missions"),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.3, 0.2, 0.1)),
                            ));
                            
                            header.spawn((
                                Text::new(format!("Points: {} | Active: {}", 
                                    research_manager.research_points, 
                                    research_manager.active_missions.len())),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.4, 0.3, 0.2)),
                            ));
                        });
                        
                        // Introduction text
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
                        
                        // Active missions list
                        if research_manager.active_missions.is_empty() {
                            research_content.spawn((
                                Text::new("No active missions. Complete basic objectives to unlock research opportunities!"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.6, 0.5, 0.4)),
                            ));
                        } else {
                            for mission in &research_manager.active_missions {
                                research_content.spawn((
                                    Button,
                                    Node {
                                        width: Val::Percent(100.0),
                                        flex_direction: FlexDirection::Column,
                                        padding: UiRect::all(Val::Px(15.0)),
                                        border: UiRect::all(Val::Px(2.0)),
                                        margin: UiRect::bottom(Val::Px(10.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.98, 0.95, 0.92)),
                                    BorderColor(match mission.difficulty {
                                        MissionDifficulty::Citizen => Color::srgb(0.2, 0.8, 0.2),
                                        MissionDifficulty::Student => Color::srgb(0.2, 0.5, 0.8),
                                        MissionDifficulty::Researcher => Color::srgb(0.8, 0.5, 0.2),
                                        MissionDifficulty::Expert => Color::srgb(0.8, 0.2, 0.2),
                                    }),
                                    ResearchMissionCard { mission_id: mission.id },
                                )).with_children(|card| {
                                    // Mission header
                                    card.spawn((
                                        Node {
                                            width: Val::Percent(100.0),
                                            flex_direction: FlexDirection::Row,
                                            justify_content: JustifyContent::SpaceBetween,
                                            align_items: AlignItems::Center,
                                            margin: UiRect::bottom(Val::Px(8.0)),
                                            ..default()
                                        },
                                    )).with_children(|mission_header| {
                                        mission_header.spawn((
                                            Text::new(&mission.title),
                                            TextFont {
                                                font_size: 16.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.2, 0.1, 0.05)),
                                        ));
                                        
                                        mission_header.spawn((
                                            Text::new(format!("{:?}", mission.difficulty)),
                                            TextFont {
                                                font_size: 12.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.4, 0.3, 0.2)),
                                        ));
                                    });
                                    
                                    // Mission description
                                    card.spawn((
                                        Text::new(&mission.description),
                                        TextFont {
                                            font_size: 12.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.4, 0.3, 0.2)),
                                        Node {
                                            margin: UiRect::bottom(Val::Px(8.0)),
                                            ..default()
                                        },
                                    ));
                                    
                                    // Progress bar
                                    card.spawn((
                                        Node {
                                            width: Val::Percent(100.0),
                                            height: Val::Px(20.0),
                                            border: UiRect::all(Val::Px(1.0)),
                                            ..default()
                                        },
                                        BorderColor(Color::srgb(0.6, 0.5, 0.4)),
                                        BackgroundColor(Color::srgb(0.9, 0.9, 0.9)),
                                        MissionProgressBar,
                                    )).with_children(|progress_bar| {
                                        progress_bar.spawn((
                                            Node {
                                                width: Val::Percent(mission.progress.completion_percentage * 100.0),
                                                height: Val::Percent(100.0),
                                                ..default()
                                            },
                                            BackgroundColor(match mission.difficulty {
                                                MissionDifficulty::Citizen => Color::srgb(0.2, 0.8, 0.2),
                                                MissionDifficulty::Student => Color::srgb(0.2, 0.5, 0.8),
                                                MissionDifficulty::Researcher => Color::srgb(0.8, 0.5, 0.2),
                                                MissionDifficulty::Expert => Color::srgb(0.8, 0.2, 0.2),
                                            }),
                                        ));
                                    });
                                    
                                    // Mission details
                                    card.spawn((
                                        Node {
                                            width: Val::Percent(100.0),
                                            flex_direction: FlexDirection::Row,
                                            justify_content: JustifyContent::SpaceBetween,
                                            margin: UiRect::top(Val::Px(8.0)),
                                            ..default()
                                        },
                                    )).with_children(|details| {
                                        details.spawn((
                                            Text::new(format!("Progress: {:.0}%", 
                                                mission.progress.completion_percentage * 100.0)),
                                            TextFont {
                                                font_size: 11.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.5, 0.4, 0.3)),
                                        ));
                                        
                                        if let Some(partner) = &mission.citizen_science_partner {
                                            details.spawn((
                                                Text::new(format!("Partner: {}", partner)),
                                                TextFont {
                                                    font_size: 11.0,
                                                    ..default()
                                                },
                                                TextColor(Color::srgb(0.3, 0.5, 0.7)),
                                            ));
                                        }
                                        
                                        details.spawn((
                                            Text::new(format!("Reward: {} pts", 
                                                mission.rewards.research_points)),
                                            TextFont {
                                                font_size: 11.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.6, 0.4, 0.1)),
                                        ));
                                    });
                                });
                            }
                        }
                    });
                },
                JournalTab::Achievements => {
                    // Achievements tab content - Display actual achievements
                    content.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(15.0),
                            ..default()
                        },
                    )).with_children(|achievements_content| {
                        // Achievement header with stats
                        achievements_content.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                margin: UiRect::bottom(Val::Px(15.0)),
                                ..default()
                            },
                        )).with_children(|header| {
                            header.spawn((
                                Text::new("Achievements & Progress"),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.3, 0.2, 0.1)),
                            ));
                            
                            header.spawn((
                                Text::new(format!("Unlocked: {}/11", 
                                    achievement_progress.unlocked.len())),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.4, 0.3, 0.2)),
                            ));
                        });
                        
                        // Progress statistics
                        achievements_content.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(8.0),
                                padding: UiRect::all(Val::Px(15.0)),
                                border: UiRect::all(Val::Px(1.0)),
                                margin: UiRect::bottom(Val::Px(15.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.95, 0.92, 0.88, 0.5)),
                            BorderColor(Color::srgb(0.7, 0.6, 0.5)),
                        )).with_children(|stats| {
                            stats.spawn((
                                Text::new("Progress Statistics"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.3, 0.2, 0.1)),
                                Node {
                                    margin: UiRect::bottom(Val::Px(8.0)),
                                    ..default()
                                },
                            ));
                            
                            stats.spawn((
                                Text::new(format!(
                                    "Photos Taken: {} | Species Discovered: {} | Action Shots: {} | Multi-Bird Shots: {}", 
                                    achievement_progress.photos_taken,
                                    achievement_progress.species_discovered,
                                    achievement_progress.action_shots_taken,
                                    achievement_progress.multi_bird_shots
                                )),
                                TextFont {
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.5, 0.4, 0.3)),
                            ));
                        });
                        
                        // Achievement grid
                        achievements_content.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Row,
                                flex_wrap: FlexWrap::Wrap,
                                row_gap: Val::Px(10.0),
                                column_gap: Val::Px(10.0),
                                ..default()
                            },
                        )).with_children(|grid| {
                            // List all achievements
                            let all_achievements = [
                                Achievement::FirstPhoto,
                                Achievement::PhotoMaster,
                                Achievement::ActionShot,
                                Achievement::MultiSpeciesShot,
                                Achievement::FirstSpecies,
                                Achievement::CommonCollector,
                                Achievement::Ornithologist,
                                Achievement::Wealthy,
                                Achievement::Millionaire,
                                Achievement::FeederMaintainer,
                                Achievement::FeederExpert,
                            ];
                            
                            for achievement in all_achievements.iter() {
                                let is_unlocked = achievement_progress.is_unlocked(&achievement);
                                let (bg_color, border_color, text_color) = if is_unlocked {
                                    (
                                        Color::srgb(0.85, 0.95, 0.85),
                                        Color::srgb(0.2, 0.8, 0.2),
                                        Color::srgb(0.1, 0.4, 0.1)
                                    )
                                } else {
                                    (
                                        Color::srgb(0.9, 0.9, 0.9),
                                        Color::srgb(0.6, 0.6, 0.6),
                                        Color::srgb(0.5, 0.5, 0.5)
                                    )
                                };
                                
                                grid.spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(280.0),
                                        height: Val::Px(100.0),
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::SpaceBetween,
                                        padding: UiRect::all(Val::Px(12.0)),
                                        border: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    BackgroundColor(bg_color),
                                    BorderColor(border_color),
                                    AchievementCard { achievement: achievement.clone() },
                                )).with_children(|card| {
                                    // Achievement header
                                    card.spawn((
                                        Node {
                                            width: Val::Percent(100.0),
                                            flex_direction: FlexDirection::Row,
                                            justify_content: JustifyContent::SpaceBetween,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                    )).with_children(|achievement_header| {
                                        achievement_header.spawn((
                                            Text::new(achievement.name()),
                                            TextFont {
                                                font_size: 16.0,
                                                ..default()
                                            },
                                            TextColor(text_color),
                                        ));
                                        
                                        let status_text = if is_unlocked { "✓" } else { "○" };
                                        achievement_header.spawn((
                                            Text::new(status_text),
                                            TextFont {
                                                font_size: 16.0,
                                                ..default()
                                            },
                                            TextColor(if is_unlocked { 
                                                Color::srgb(0.2, 0.8, 0.2) 
                                            } else { 
                                                Color::srgb(0.6, 0.6, 0.6) 
                                            }),
                                        ));
                                    });
                                    
                                    // Achievement description
                                    card.spawn((
                                        Text::new(achievement.description()),
                                        TextFont {
                                            font_size: 12.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.4, 0.3, 0.2)),
                                    ));
                                    
                                    // Reward info
                                    card.spawn((
                                        Text::new(format!("Reward: {} currency", achievement.currency_reward())),
                                        TextFont {
                                            font_size: 11.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.6, 0.4, 0.1)),
                                    ));
                                });
                            }
                        });
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
) {
    for (interaction, tab_button, mut bg_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = Color::srgb(0.95, 0.92, 0.88).into();
                
                if journal_state.current_tab != tab_button.tab {
                    info!("🔵 JOURNAL TAB: Switching from {:?} to {:?}", journal_state.current_tab, tab_button.tab);
                    journal_state.current_tab = tab_button.tab;
                    // Note: Content regeneration will be handled by journal_state_monitor_system
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
    interaction_query: Query<
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

pub fn journal_state_monitor_system(
    journal_state: Res<JournalState>,
    mut commands: Commands,
    content_query: Query<Entity, With<JournalTabContent>>,
    journal_query: Query<Entity, With<JournalMenu>>,
    children_query: Query<&Children>,
    discovered: Res<DiscoveredSpecies>,
    education_data: Res<BirdEducationData>,
    photo_collection: Res<PhotoCollection>,
    research_manager: Res<ResearchMissionManager>,
    achievement_progress: Res<AchievementProgress>,
) {
    if journal_state.is_changed() && journal_state.is_open {
        info!("🔵 JOURNAL STATE: Journal state changed, regenerating content");
        
        if let Ok(journal_entity) = journal_query.single() {
            regenerate_journal_content(
                &mut commands,
                journal_entity,
                &children_query,
                &content_query,
                &journal_state,
                &discovered,
                &education_data,
                &photo_collection,
                &research_manager,
                &achievement_progress,
            );
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
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<CloseButton>)>,
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

// Helper function to regenerate journal content when switching tabs
fn regenerate_journal_content(
    commands: &mut Commands,
    journal_entity: Entity,
    children_query: &Query<&Children>,
    content_query: &Query<Entity, With<JournalTabContent>>,
    journal_state: &JournalState,
    discovered: &DiscoveredSpecies,
    education_data: &BirdEducationData,
    photo_collection: &PhotoCollection,
    research_manager: &ResearchMissionManager,
    achievement_progress: &AchievementProgress,
) {
    // Find the existing content area and clear its children instead of despawning it
    if let Ok(journal_children) = children_query.get(journal_entity) {
        // Find the content area (it should be the third child after header and tabs)
        for child in journal_children.iter() {
            if content_query.contains(child) {
                // Clear existing children of the content area
                if let Ok(content_children) = children_query.get(child) {
                    for content_child in content_children.iter() {
                        commands.entity(content_child).safe_despawn();
                    }
                }
                
                // Add new content to the existing content area
                commands.entity(child).with_children(|content| {
                    match journal_state.current_tab {
                        JournalTab::Species => {
                            content.spawn((
                                Text::new(format!("Species discovered: {}\nEducational data loaded: {}", 
                                    discovered.0.len(), education_data.species_facts.len())),
                                TextFont { font_size: 16.0, ..default() },
                                TextColor(Color::srgb(0.3, 0.2, 0.1)),
                            ));
                        },
                        JournalTab::Photos => {
                            content.spawn((
                                Text::new(format!("Photos: {}", photo_collection.photos.len())),
                                TextFont { font_size: 16.0, ..default() },
                                TextColor(Color::srgb(0.3, 0.2, 0.1)),
                            ));
                        },
                        JournalTab::Research => {
                            content.spawn((
                                Text::new(format!("Research missions: {} active, {} completed", 
                                    research_manager.active_missions.len(), research_manager.completed_missions.len())),
                                TextFont { font_size: 16.0, ..default() },
                                TextColor(Color::srgb(0.3, 0.2, 0.1)),
                            ));
                        },
                        JournalTab::Achievements => {
                            content.spawn((
                                Text::new(format!("Achievements unlocked: {}/11\nPhotos taken: {}", 
                                    achievement_progress.unlocked.len(), achievement_progress.photos_taken)),
                                TextFont { font_size: 16.0, ..default() },
                                TextColor(Color::srgb(0.3, 0.2, 0.1)),
                            ));
                        },
                        JournalTab::Conservation => {
                            content.spawn((
                                Text::new(format!("Conservation data for {} species", discovered.0.len())),
                                TextFont { font_size: 16.0, ..default() },
                                TextColor(Color::srgb(0.3, 0.2, 0.1)),
                            ));
                        },
                        JournalTab::Migration => {
                            content.spawn((
                                Text::new(format!("Migration data for {} species", discovered.0.len())),
                                TextFont { font_size: 16.0, ..default() },
                                TextColor(Color::srgb(0.3, 0.2, 0.1)),
                            ));
                        },
                    }
                });
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::journal::components::JournalTab;

    #[test]
    fn test_journal_state_default() {
        let state = JournalState::default();
        assert!(!state.is_open);
        assert_eq!(state.current_tab, JournalTab::Species);
        assert_eq!(state.selected_species, None);
    }

    #[test]
    fn test_journal_state_tab_switching() {
        let mut state = JournalState::default();
        
        // Test initial state
        assert_eq!(state.current_tab, JournalTab::Species);
        
        // Test switching to different tabs
        state.current_tab = JournalTab::Photos;
        assert_eq!(state.current_tab, JournalTab::Photos);
        
        state.current_tab = JournalTab::Research;
        assert_eq!(state.current_tab, JournalTab::Research);
        
        state.current_tab = JournalTab::Achievements;
        assert_eq!(state.current_tab, JournalTab::Achievements);
    }

    #[test]
    fn test_journal_state_open_close() {
        let mut state = JournalState::default();
        
        // Test initial closed state
        assert!(!state.is_open);
        
        // Test opening
        state.is_open = true;
        assert!(state.is_open);
        
        // Test closing
        state.is_open = false;
        assert!(!state.is_open);
    }

    #[test]
    fn test_discovered_species_empty() {
        let discovered = DiscoveredSpecies::default();
        assert_eq!(discovered.0.len(), 0);
        assert!(discovered.0.is_empty());
    }

    #[test]
    fn test_discovered_species_add() {
        let mut discovered = DiscoveredSpecies::default();
        
        discovered.0.insert(crate::bird::BirdSpecies::Robin);
        assert_eq!(discovered.0.len(), 1);
        assert!(discovered.0.contains(&crate::bird::BirdSpecies::Robin));
        
        // Adding the same species again should not increase the count (Set behavior)
        discovered.0.insert(crate::bird::BirdSpecies::Robin);
        assert_eq!(discovered.0.len(), 1);
        
        // Adding a different species should increase the count
        discovered.0.insert(crate::bird::BirdSpecies::Cardinal);
        assert_eq!(discovered.0.len(), 2);
        assert!(discovered.0.contains(&crate::bird::BirdSpecies::Cardinal));
    }

    #[test]
    fn test_conservation_status_color() {
        use super::ConservationStatus;
        
        // Test that each conservation status has a proper color assigned
        let least_concern = ConservationStatus::LeastConcern;
        let endangered = ConservationStatus::Endangered;
        let extinct = ConservationStatus::Extinct;
        
        // Colors should be different for different statuses
        assert_ne!(least_concern.color(), endangered.color());
        assert_ne!(endangered.color(), extinct.color());
        assert_ne!(least_concern.color(), extinct.color());
        
        // Colors should be consistent for the same status
        assert_eq!(least_concern.color(), ConservationStatus::LeastConcern.color());
    }

    #[test]
    fn test_conservation_status_label() {
        use super::ConservationStatus;
        
        assert_eq!(ConservationStatus::LeastConcern.label(), "Least Concern");
        assert_eq!(ConservationStatus::Endangered.label(), "Endangered");
        assert_eq!(ConservationStatus::Extinct.label(), "Extinct");
    }
}


