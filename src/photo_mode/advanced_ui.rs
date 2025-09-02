// Advanced Photography UI - Phase 4
use bevy::prelude::*;
use crate::photo_mode::advanced_photo::*;
use crate::photo_mode::resources::PhotoModeSettings;

// UI Components
#[derive(Component)]
pub struct AdvancedPhotoUI;

#[derive(Component)]
pub struct LensSelectionPanel;

#[derive(Component)]
pub struct FilterSelectionPanel;

#[derive(Component)]
pub struct CameraSettingsPanel;

#[derive(Component)]
pub struct PhotoCollectionPanel;

#[derive(Component)]
pub struct LensButton {
    pub lens_type: LensType,
}

#[derive(Component)]
pub struct FilterButton {
    pub filter_type: PhotoFilter,
}

#[derive(Component)]
pub struct CompositionToggle {
    pub guide_type: CompositionGuideType,
}

#[derive(Debug, Clone, Copy)]
pub enum CompositionGuideType {
    RuleOfThirds,
    GoldenSpiral,
    CenterGuides,
    DiagonalGuides,
    SafeZones,
}

// Setup System
pub fn setup_advanced_photo_ui(mut commands: Commands) {
    // Main advanced photo mode container (initially hidden)
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            display: Display::None,
            ..default()
        },
        AdvancedPhotoUI,
    )).with_children(|parent| {
        
        // Left panel - Lens selection
        parent.spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                padding: UiRect::all(Val::Px(15.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
            BorderColor(Color::srgb(0.4, 0.4, 0.4)),
            LensSelectionPanel,
        )).with_children(|lens_panel| {
            // Lens panel title
            lens_panel.spawn((
                Text::new("Lenses"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));
            
            // Lens buttons
            let lenses = [
                (LensType::Standard, true),   // Always available
                (LensType::Telephoto, false), // Needs unlock
                (LensType::Macro, false),
                (LensType::WideAngle, false),
            ];
            
            for (lens_type, available) in lenses {
                let (bg_color, text_color) = if available {
                    (Color::srgb(0.2, 0.4, 0.6), Color::WHITE)
                } else {
                    (Color::srgb(0.3, 0.3, 0.3), Color::srgb(0.6, 0.6, 0.6))
                };
                
                lens_panel.spawn((
                    Button,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(bg_color),
                    LensButton { lens_type },
                )).with_children(|button| {
                    button.spawn((
                        Text::new(lens_type.name()),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(text_color),
                    ));
                });
            }
        });
        
        // Right panel - Filters and settings
        parent.spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                right: Val::Px(20.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                padding: UiRect::all(Val::Px(15.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
            BorderColor(Color::srgb(0.4, 0.4, 0.4)),
            FilterSelectionPanel,
        )).with_children(|filter_panel| {
            // Filter panel title
            filter_panel.spawn((
                Text::new("Filters"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));
            
            // Filter buttons
            let filters = [
                (PhotoFilter::None, true),
                (PhotoFilter::Warm, false),
                (PhotoFilter::Cool, false),
                (PhotoFilter::Vibrant, false),
                (PhotoFilter::BlackWhite, false),
                (PhotoFilter::Vintage, false),
                (PhotoFilter::HighContrast, false),
            ];
            
            for (filter_type, available) in filters {
                let (bg_color, text_color) = if available {
                    (Color::srgb(0.4, 0.2, 0.6), Color::WHITE)
                } else {
                    (Color::srgb(0.3, 0.3, 0.3), Color::srgb(0.6, 0.6, 0.6))
                };
                
                filter_panel.spawn((
                    Button,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(35.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(3.0)),
                        ..default()
                    },
                    BackgroundColor(bg_color),
                    FilterButton { filter_type },
                )).with_children(|button| {
                    button.spawn((
                        Text::new(filter_type.name()),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(text_color),
                    ));
                });
            }
        });
        
        // Bottom panel - Composition guides
        parent.spawn((
            Node {
                width: Val::Percent(60.0),
                height: Val::Px(80.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(20.0),
                left: Val::Percent(20.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
            BorderColor(Color::srgb(0.4, 0.4, 0.4)),
        )).with_children(|guides_panel| {
            guides_panel.spawn((
                Text::new("Composition Guides:"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::right(Val::Px(15.0)),
                    ..default()
                },
            ));
            
            let guides = [
                (CompositionGuideType::RuleOfThirds, "Rule of Thirds"),
                (CompositionGuideType::GoldenSpiral, "Golden Spiral"),
                (CompositionGuideType::CenterGuides, "Center"),
                (CompositionGuideType::DiagonalGuides, "Diagonal"),
                (CompositionGuideType::SafeZones, "Safe Zones"),
            ];
            
            for (guide_type, name) in guides {
                guides_panel.spawn((
                    Button,
                    Node {
                        width: Val::Px(80.0),
                        height: Val::Px(30.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                    CompositionToggle { guide_type },
                )).with_children(|button| {
                    button.spawn((
                        Text::new(name),
                        TextFont {
                            font_size: 10.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            }
        });
        
        // Camera info display (top center)
        parent.spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Px(60.0),
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                left: Val::Percent(50.0),
                margin: UiRect::left(Val::Px(-150.0)), // Center it
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
            BorderColor(Color::srgb(0.4, 0.4, 0.4)),
        )).with_children(|info_panel| {
            info_panel.spawn((
                Text::new("Standard 50mm | No Filter"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                CameraInfoDisplay,
            ));
            
            info_panel.spawn((
                Text::new("f/2.8 | 1/60s | ISO 200"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                CameraSettingsDisplay,
            ));
        });
    });
}

#[derive(Component)]
pub struct CameraInfoDisplay;

#[derive(Component)]
pub struct CameraSettingsDisplay;

// Input handling systems
pub fn advanced_photo_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut photo_settings: ResMut<PhotoModeSettings>,
    mut advanced_ui_query: Query<&mut Node, With<AdvancedPhotoUI>>,
    mut camera_settings: ResMut<AdvancedCameraSettings>,
) {
    // Toggle advanced photo UI with F key
    if keyboard.just_pressed(KeyCode::KeyF) && photo_settings.is_active {
        for mut ui_node in advanced_ui_query.iter_mut() {
            ui_node.display = match ui_node.display {
                Display::None => Display::Flex,
                Display::Flex => Display::None,
                _ => Display::None,
            };
        }
    }
    
    // Quick lens switching with number keys
    if photo_settings.is_active {
        if keyboard.just_pressed(KeyCode::Digit1) {
            camera_settings.current_lens = LensType::Standard;
        }
        if keyboard.just_pressed(KeyCode::Digit2) && 
           camera_settings.available_lenses.contains(&LensType::Telephoto) {
            camera_settings.current_lens = LensType::Telephoto;
        }
        if keyboard.just_pressed(KeyCode::Digit3) &&
           camera_settings.available_lenses.contains(&LensType::Macro) {
            camera_settings.current_lens = LensType::Macro;
        }
        if keyboard.just_pressed(KeyCode::Digit4) &&
           camera_settings.available_lenses.contains(&LensType::WideAngle) {
            camera_settings.current_lens = LensType::WideAngle;
        }
    }
}

// Button interaction systems
pub fn lens_button_system(
    mut interaction_query: Query<
        (&Interaction, &LensButton, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut camera_settings: ResMut<AdvancedCameraSettings>,
    mut lens_events: EventWriter<LensSwitchEvent>,
) {
    for (interaction, lens_button, mut bg_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if camera_settings.available_lenses.contains(&lens_button.lens_type) {
                    camera_settings.current_lens = lens_button.lens_type;
                    lens_events.send(LensSwitchEvent {
                        new_lens: lens_button.lens_type,
                    });
                    *bg_color = Color::srgb(0.1, 0.6, 0.9).into();
                }
            }
            Interaction::Hovered => {
                if camera_settings.available_lenses.contains(&lens_button.lens_type) {
                    *bg_color = Color::srgb(0.3, 0.5, 0.7).into();
                } else {
                    *bg_color = Color::srgb(0.4, 0.4, 0.4).into();
                }
            }
            Interaction::None => {
                if camera_settings.current_lens == lens_button.lens_type {
                    *bg_color = Color::srgb(0.2, 0.4, 0.6).into();
                } else if camera_settings.available_lenses.contains(&lens_button.lens_type) {
                    *bg_color = Color::srgb(0.2, 0.4, 0.6).into();
                } else {
                    *bg_color = Color::srgb(0.3, 0.3, 0.3).into();
                }
            }
        }
    }
}

pub fn filter_button_system(
    mut interaction_query: Query<
        (&Interaction, &FilterButton, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut camera_settings: ResMut<AdvancedCameraSettings>,
    mut filter_events: EventWriter<FilterChangeEvent>,
) {
    for (interaction, filter_button, mut bg_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if camera_settings.available_filters.contains(&filter_button.filter_type) {
                    camera_settings.current_filter = filter_button.filter_type;
                    filter_events.send(FilterChangeEvent {
                        new_filter: filter_button.filter_type,
                    });
                    *bg_color = Color::srgb(0.6, 0.1, 0.9).into();
                }
            }
            Interaction::Hovered => {
                if camera_settings.available_filters.contains(&filter_button.filter_type) {
                    *bg_color = Color::srgb(0.5, 0.3, 0.7).into();
                } else {
                    *bg_color = Color::srgb(0.4, 0.4, 0.4).into();
                }
            }
            Interaction::None => {
                if camera_settings.current_filter == filter_button.filter_type {
                    *bg_color = Color::srgb(0.4, 0.2, 0.6).into();
                } else if camera_settings.available_filters.contains(&filter_button.filter_type) {
                    *bg_color = Color::srgb(0.4, 0.2, 0.6).into();
                } else {
                    *bg_color = Color::srgb(0.3, 0.3, 0.3).into();
                }
            }
        }
    }
}

pub fn composition_guide_system(
    mut interaction_query: Query<
        (&Interaction, &CompositionToggle, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut guides: ResMut<CompositionGuides>,
) {
    for (interaction, toggle, mut bg_color) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            let (current_state, new_color) = match toggle.guide_type {
                CompositionGuideType::RuleOfThirds => {
                    guides.rule_of_thirds = !guides.rule_of_thirds;
                    (guides.rule_of_thirds, Color::srgb(0.2, 0.7, 0.2))
                }
                CompositionGuideType::GoldenSpiral => {
                    guides.golden_spiral = !guides.golden_spiral;
                    (guides.golden_spiral, Color::srgb(0.7, 0.7, 0.2))
                }
                CompositionGuideType::CenterGuides => {
                    guides.center_guides = !guides.center_guides;
                    (guides.center_guides, Color::srgb(0.2, 0.2, 0.7))
                }
                CompositionGuideType::DiagonalGuides => {
                    guides.diagonal_guides = !guides.diagonal_guides;
                    (guides.diagonal_guides, Color::srgb(0.7, 0.2, 0.7))
                }
                CompositionGuideType::SafeZones => {
                    guides.safe_zones = !guides.safe_zones;
                    (guides.safe_zones, Color::srgb(0.7, 0.5, 0.2))
                }
            };
            
            *bg_color = if current_state {
                new_color.into()
            } else {
                Color::srgb(0.3, 0.3, 0.3).into()
            };
        }
    }
}

// Camera info display update
pub fn update_camera_info_display(
    camera_settings: Res<AdvancedCameraSettings>,
    mut info_query: Query<&mut Text, With<CameraInfoDisplay>>,
    mut settings_query: Query<&mut Text, (With<CameraSettingsDisplay>, Without<CameraInfoDisplay>)>,
) {
    if camera_settings.is_changed() {
        // Update camera info
        for mut text in info_query.iter_mut() {
            **text = format!("{} | {}", 
                camera_settings.current_lens.name(),
                camera_settings.current_filter.name()
            );
        }
        
        // Update camera settings display
        for mut text in settings_query.iter_mut() {
            **text = format!("f/{:.1} | 1/{}s | ISO {}", 
                2.8, // Placeholder - would come from actual camera settings
                60,
                200
            );
        }
    }
}