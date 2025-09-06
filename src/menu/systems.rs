use bevy::prelude::*;
use crate::menu::{components::*, resources::*};
use crate::save_load::resources::{SaveGameEvent, LoadGameEvent, SaveManager};
use crate::despawn::SafeDespawn;
use crate::ui_widgets::ToggleButton;
use crate::user_interface::slider::{SliderBuilder, SliderValueChangedEvent};
use crate::user_interface::dropdown::{DropdownBuilder, DropdownChangedEvent, DropdownChangeKind, DropdownConfig};
use crate::user_interface::toggle::{ToggleBuilder, ToggleChangedEvent};
use crate::user_interface::scrollable::ScrollableBuilder;
use crate::user_interface::tab_group::*;
use crate::audio::resources::AudioSettings;

// Setup Systems

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
        MenuUI,
    )).with_children(|parent| {
        // Menu container
        parent.spawn((
            Node {
                width: Val::Px(400.0),
                height: Val::Px(500.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                padding: UiRect::all(Val::Px(40.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.95, 0.92, 0.88)),
            BorderColor(Color::srgb(0.6, 0.4, 0.2)),
            BorderRadius::all(Val::Px(6.0)),
        )).with_children(|menu| {
            // Title
            menu.spawn((
                Text::new("Perch"),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor(Color::srgb(0.3, 0.2, 0.1)),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
                MenuTitle,
            ));
            
            // Menu buttons
            let buttons = [
                ("New Game", MainMenuAction::NewGame),
                ("Load Game", MainMenuAction::LoadGame),
                ("Settings", MainMenuAction::Settings),
                ("Quit", MainMenuAction::Quit),
            ];
            
            for (text, action) in buttons {
                menu.spawn((
                    Button,
                    Node {
                        width: Val::Px(280.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.6, 0.5, 0.4)),
                    BorderRadius::all(Val::Px(6.0)),
                    MainMenuButton { action },
                )).with_children(|button| {
                    button.spawn((
                        Text::new(text),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            }
        });
    });
}

pub fn setup_settings_menu(mut commands: Commands, settings: Res<GameSettings>) {
    // Create the fullscreen toggle widget first, before any UI hierarchy
    let toggle_config = crate::user_interface::toggle::ToggleConfig {
        size: Vec2::new(50.0, 25.0),
        on_color: Color::srgb(0.2, 0.8, 0.2),
        off_color: Color::srgb(0.5, 0.5, 0.5),
        ..default()
    };
    
    let fullscreen_toggle_entity = ToggleBuilder::new(&mut commands)
        .with_initial_state(settings.fullscreen)
        .with_config(toggle_config)
        .spawn_state_scoped(crate::AppState::Settings);
        
    // Tag the toggle for identification in event handling
    commands.entity(fullscreen_toggle_entity).insert(FullscreenToggle);

    // SCREEN POSITIONING
    // FULL SCREEN, CENTRE ALIGNED
    let menu_entity = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
        MenuUI,
    )).id();
    
    // Create settings container (window)
    let settings_container_entity = commands.spawn((
        Node {
            width: Val::Px(600.0),
            height: Val::Px(700.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(40.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.95, 0.92, 0.88)),
        BorderColor(Color::srgb(0.6, 0.4, 0.2)),
        BorderRadius::all(Val::Px(6.0)),
    )).id();
    
    // Add title to settings container
    let title_entity = commands.spawn((
        Text::new("Settings"),
        TextFont {
            font_size: 28.0,
            ..default()
        },
        TextColor(Color::srgb(0.3, 0.2, 0.1)),
        Node {
            margin: UiRect::bottom(Val::Px(30.0)),
            ..default()
        },
    )).id();
    


    // Create scrollable container
    let (scrollable_container, scrollable_content_entity) = ScrollableBuilder::new(&mut commands)
        .with_size(Vec2::new(520.0, 500.0)) // Container size
        .with_content_height(800.0) // Content will be taller than container
        .spawn();
    
    // Set up hierarchy: menu -> settings_container -> [title, scrollable_container]
    commands.entity(menu_entity).add_children(&[settings_container_entity]);
    commands.entity(settings_container_entity).add_children(&[title_entity, scrollable_container]);

    // Populate scrollable content
    commands.entity(scrollable_content_entity).with_children(|scrollable_content| {
            
            // Audio settings section with side-by-side sliders
            scrollable_content.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(15.0),
                    margin: UiRect::vertical(Val::Px(20.0)),
                    ..default()
                },
                AudioSection, // Add a marker component
            )).with_children(|section| {
                section.spawn((
                    Text::new("Audio"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.2, 0.2, 0.3)),
                    Node {
                        margin: UiRect::bottom(Val::Px(15.0)),
                        ..default()
                    },
                ));
                
                // Master Volume slider will be added by setup_audio_sliders_system
                
                // Music Volume slider will be added by setup_audio_sliders_system
                
                // SFX Volume slider will be added by setup_audio_sliders_system
            });
            
            // Graphics settings section
            scrollable_content.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(15.0),
                    margin: UiRect::vertical(Val::Px(20.0)),
                    ..default()
                },
            )).with_children(|section| {
                section.spawn((
                    Text::new("Graphics"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.2, 0.2, 0.3)),
                    Node {
                        margin: UiRect::bottom(Val::Px(15.0)),
                        ..default()
                    },
                ));
                
                // Resolution section container (side-by-side layout)
                let resolution_container = section.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(15.0)),
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.9, 0.9, 0.9)),
                    BorderRadius::all(Val::Px(6.0)),
                    GraphicsSection, // Mark this as the graphics section for dropdown setup
                )).with_children(|container| {
                    // Resolution label (left side)
                    container.spawn((
                        Text::new("Resolution"),
                        TextFont { font_size: 16.0, ..default() },
                        TextColor(Color::srgb(0.3, 0.2, 0.1)),
                        ResolutionDropdownLabel, // Marker component
                    ));
                    
                    // Dropdown container (right side) - dropdown will be added here by setup_resolution_dropdown_system
                }).id();
                
                // Graphics Quality selector (simplified for now)
                section.spawn((
                    Button,
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.9, 0.9, 0.9)),
                    BorderRadius::all(Val::Px(6.0)),
                    GraphicsQualityDropdown,
                )).with_children(|item| {
                    item.spawn((
                        Text::new("Graphics Quality"),
                        TextFont { font_size: 16.0, ..default() },
                        TextColor(Color::srgb(0.3, 0.2, 0.1)),
                    ));
                    item.spawn((
                        Text::new(format!("{} ▼", settings.graphics_quality.to_string())),
                        TextFont { font_size: 16.0, ..default() },
                        TextColor(Color::srgb(0.5, 0.3, 0.2)),
                    ));
                });
                
                // VSync toggle
                section.spawn((
                    Button,
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.9, 0.9, 0.9)),
                    BorderRadius::all(Val::Px(6.0)),
                    ToggleButton::new("VSync", settings.vsync_enabled),
                )).with_children(|item| {
                    item.spawn((
                        Text::new("VSync"),
                        TextFont { font_size: 16.0, ..default() },
                        TextColor(Color::srgb(0.3, 0.2, 0.1)),
                    ));
                    item.spawn((
                        Text::new(if settings.vsync_enabled { "ON" } else { "OFF" }),
                        TextFont { font_size: 16.0, ..default() },
                        TextColor(if settings.vsync_enabled { 
                            Color::srgb(0.2, 0.6, 0.2) 
                        } else { 
                            Color::srgb(0.6, 0.2, 0.2) 
                        }),
                    ));
                });
                
                // Fullscreen toggle container - using pre-created StateScoped toggle widget
                section.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.9, 0.9, 0.9)),
                    BorderRadius::all(Val::Px(6.0)),
                    FullscreenToggleContainer,
                )).with_children(|container| {
                    container.spawn((
                        Text::new("Fullscreen"),
                        TextFont { font_size: 16.0, ..default() },
                        TextColor(Color::srgb(0.3, 0.2, 0.1)),
                    ));
                }).add_child(fullscreen_toggle_entity);
            });
            
            // Gameplay settings section (more content to demonstrate scrolling)
            scrollable_content.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(15.0),
                    margin: UiRect::vertical(Val::Px(20.0)),
                    ..default()
                },
            )).with_children(|section| {
                section.spawn((
                    Text::new("Gameplay"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.2, 0.2, 0.3)),
                    Node {
                        margin: UiRect::bottom(Val::Px(15.0)),
                        ..default()
                    },
                ));
                
                // Auto-save toggle
                section.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.9, 0.9, 0.9)),
                    BorderRadius::all(Val::Px(6.0)),
                )).with_children(|container| {
                    container.spawn((
                        Text::new("Auto-Save"),
                        TextFont { font_size: 16.0, ..default() },
                        TextColor(Color::srgb(0.3, 0.2, 0.1)),
                    ));
                    container.spawn((
                        Text::new(if settings.auto_save_enabled { "ON" } else { "OFF" }),
                        TextFont { font_size: 16.0, ..default() },
                        TextColor(if settings.auto_save_enabled { 
                            Color::srgb(0.2, 0.6, 0.2) 
                        } else { 
                            Color::srgb(0.6, 0.2, 0.2) 
                        }),
                    ));
                });
                
                // Difficulty setting
                section.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.9, 0.9, 0.9)),
                    BorderRadius::all(Val::Px(6.0)),
                )).with_children(|container| {
                    container.spawn((
                        Text::new("Difficulty"),
                        TextFont { font_size: 16.0, ..default() },
                        TextColor(Color::srgb(0.3, 0.2, 0.1)),
                    ));
                    container.spawn((
                        Text::new("Normal"),
                        TextFont { font_size: 16.0, ..default() },
                        TextColor(Color::srgb(0.5, 0.3, 0.2)),
                    ));
                });
            });
            
            // Controls settings section
            scrollable_content.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(15.0),
                    margin: UiRect::vertical(Val::Px(20.0)),
                    ..default()
                },
            )).with_children(|section| {
                section.spawn((
                    Text::new("Controls"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.2, 0.2, 0.3)),
                    Node {
                        margin: UiRect::bottom(Val::Px(15.0)),
                        ..default()
                    },
                ));
                
                // Mouse sensitivity slider - placeholder for now
                section.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.9, 0.9, 0.9)),
                    BorderRadius::all(Val::Px(6.0)),
                )).with_children(|container| {
                    container.spawn((
                        Text::new("Mouse Sensitivity"),
                        TextFont { font_size: 16.0, ..default() },
                        TextColor(Color::srgb(0.3, 0.2, 0.1)),
                    ));
                    container.spawn((
                        Text::new("1.0"),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(Color::srgb(0.5, 0.5, 0.5)),
                    ));
                });
                
                // Key bindings button
                section.spawn((
                    Button,
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.8, 0.8, 0.9)),
                    BorderRadius::all(Val::Px(6.0)),
                )).with_children(|container| {
                    container.spawn((
                        Text::new("Configure Key Bindings"),
                        TextFont { font_size: 16.0, ..default() },
                        TextColor(Color::srgb(0.3, 0.2, 0.1)),
                    ));
                    container.spawn((
                        Text::new("→"),
                        TextFont { font_size: 18.0, ..default() },
                        TextColor(Color::srgb(0.5, 0.3, 0.2)),
                    ));
                });
            });
            
            let gameplay_settings = [
                ("Auto Save", SettingType::AutoSave, if settings.auto_save_enabled { 1.0 } else { 0.0 }),
            ];
            
            scrollable_content.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    margin: UiRect::vertical(Val::Px(20.0)),
                    ..default()
                },
            )).with_children(|section| {
                section.spawn((
                    Text::new("Gameplay"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.2, 0.2, 0.3)),
                    Node {
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    },
                ));
                
                for (label, _setting_type, value) in &gameplay_settings {
                    section.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.9, 0.9, 0.9)),
                    )).with_children(|item| {
                        item.spawn((
                            Text::new(*label),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.3, 0.2, 0.1)),
                        ));
                        
                        item.spawn((
                            Text::new(format!("{:.0}%", value * 100.0)),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.5, 0.3, 0.2)),
                        ));
                    });
                }
            });
    }); // End scrollable_content
    
    // FOOTER
    // Bottom buttons (outside scrollable area) - add directly to settings container
    commands.entity(settings_container_entity).with_children(|settings_container| {
            settings_container.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    margin: UiRect::top(Val::Px(40.0)),
                    ..default()
                },
            )).with_children(|buttons| {
                let button_configs = [
                    ("Back", SettingsAction::BackToMain),
                    ("Controls", SettingsAction::OpenControls),
                    ("Reset Defaults", SettingsAction::ResetToDefaults),
                    ("Apply", SettingsAction::ApplySettings),
                ];
                
                for (text, action) in button_configs {
                    buttons.spawn((
                        Button,
                        Node {
                            width: Val::Px(150.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.6, 0.5, 0.4)),
                        BorderRadius::all(Val::Px(6.0)),
                        SettingsButton { action },
                    )).with_children(|button| {
                        button.spawn((
                            Text::new(text),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
                }
            });
    });


}

pub fn setup_controls_menu(
    mut commands: Commands, 
    keybindings: Res<crate::keybindings::KeyBindings>
) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
        MenuUI,
    )).with_children(|parent| {
        // Controls container
        parent.spawn((
            Node {
                width: Val::Px(800.0),
                height: Val::Px(700.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                row_gap: Val::Px(15.0),
                padding: UiRect::all(Val::Px(40.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.95, 0.92, 0.88)),
            BorderColor(Color::srgb(0.6, 0.4, 0.2)),
        )).with_children(|controls_menu| {
            // Title
            controls_menu.spawn((
                Text::new("Controls"),
                TextFont { font_size: 28.0, ..default() },
                TextColor(Color::srgb(0.3, 0.2, 0.1)),
                Node { margin: UiRect::bottom(Val::Px(30.0)), ..default() },
            ));
            
            // Scrollable keybinding list
            controls_menu.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(500.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    ..default()
                },
            )).with_children(|scroll_area| {
                // Camera Controls Section
                scroll_area.spawn((
                    Text::new("Camera Controls"),
                    TextFont { font_size: 18.0, ..default() },
                    TextColor(Color::srgb(0.2, 0.2, 0.3)),
                    Node { margin: UiRect::vertical(Val::Px(10.0)), ..default() },
                ));
                
                let camera_actions = [
                    ("Move Up", crate::keybindings::GameAction::CameraMoveUp),
                    ("Move Down", crate::keybindings::GameAction::CameraMoveDown),
                    ("Move Left", crate::keybindings::GameAction::CameraMoveLeft),
                    ("Move Right", crate::keybindings::GameAction::CameraMoveRight),
                    ("Pan Camera", crate::keybindings::GameAction::CameraPan),
                ];
                
                for (label, action) in camera_actions {
                    scroll_area.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.9, 0.9, 0.9)),
                    )).with_children(|row| {
                        row.spawn((
                            Text::new(label),
                            TextFont { font_size: 16.0, ..default() },
                            TextColor(Color::srgb(0.3, 0.2, 0.1)),
                        ));
                        row.spawn((
                            Button,
                            Node {
                                width: Val::Px(120.0),
                                height: Val::Px(30.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.7, 0.7, 0.7)),
                            BorderRadius::all(Val::Px(4.0)),
                            KeybindingButton { action },
                        )).with_children(|button| {
                            button.spawn((
                                Text::new(keybindings.get_display_string(action)),
                                TextFont { font_size: 14.0, ..default() },
                                TextColor(Color::srgb(0.2, 0.2, 0.2)),
                                KeybindingText { action },
                            ));
                        });
                    });
                }
                
                // Photo Mode Section
                scroll_area.spawn((
                    Text::new("Photo Mode"),
                    TextFont { font_size: 18.0, ..default() },
                    TextColor(Color::srgb(0.2, 0.2, 0.3)),
                    Node { margin: UiRect::vertical(Val::Px(10.0)), ..default() },
                ));
                
                let photo_actions = [
                    ("Toggle Photo Mode", crate::keybindings::GameAction::TogglePhotoMode),
                    ("Take Photo", crate::keybindings::GameAction::TakePhoto),
                ];
                
                for (label, action) in photo_actions {
                    scroll_area.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.9, 0.9, 0.9)),
                    )).with_children(|row| {
                        row.spawn((
                            Text::new(label),
                            TextFont { font_size: 16.0, ..default() },
                            TextColor(Color::srgb(0.3, 0.2, 0.1)),
                        ));
                        row.spawn((
                            Button,
                            Node {
                                width: Val::Px(120.0),
                                height: Val::Px(30.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.7, 0.7, 0.7)),
                            BorderRadius::all(Val::Px(4.0)),
                            KeybindingButton { action },
                        )).with_children(|button| {
                            button.spawn((
                                Text::new(keybindings.get_display_string(action)),
                                TextFont { font_size: 14.0, ..default() },
                                TextColor(Color::srgb(0.2, 0.2, 0.2)),
                                KeybindingText { action },
                            ));
                        });
                    });
                }
                
                // UI Navigation Section
                scroll_area.spawn((
                    Text::new("UI Navigation"),
                    TextFont { font_size: 18.0, ..default() },
                    TextColor(Color::srgb(0.2, 0.2, 0.3)),
                    Node { margin: UiRect::vertical(Val::Px(10.0)), ..default() },
                ));
                
                let ui_actions = [
                    ("Open Journal", crate::keybindings::GameAction::OpenJournal),
                    ("Close Menu", crate::keybindings::GameAction::CloseMenu),
                ];
                
                for (label, action) in ui_actions {
                    scroll_area.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.9, 0.9, 0.9)),
                    )).with_children(|row| {
                        row.spawn((
                            Text::new(label),
                            TextFont { font_size: 16.0, ..default() },
                            TextColor(Color::srgb(0.3, 0.2, 0.1)),
                        ));
                        row.spawn((
                            Button,
                            Node {
                                width: Val::Px(120.0),
                                height: Val::Px(30.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.7, 0.7, 0.7)),
                            BorderRadius::all(Val::Px(4.0)),
                            KeybindingButton { action },
                        )).with_children(|button| {
                            button.spawn((
                                Text::new(keybindings.get_display_string(action)),
                                TextFont { font_size: 14.0, ..default() },
                                TextColor(Color::srgb(0.2, 0.2, 0.2)),
                                KeybindingText { action },
                            ));
                        });
                    });
                }
            });
            
            // Bottom buttons
            controls_menu.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    margin: UiRect::top(Val::Px(30.0)),
                    ..default()
                },
            )).with_children(|buttons| {
                let button_configs = [
                    ("Back to Settings", SettingsAction::BackToSettings),
                    ("Apply", SettingsAction::ApplySettings),
                ];
                
                for (text, action) in button_configs {
                    buttons.spawn((
                        Button,
                        Node {
                            width: Val::Px(150.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.6, 0.5, 0.4)),
                        BorderRadius::all(Val::Px(6.0)),
                        SettingsButton { action },
                    )).with_children(|button| {
                        button.spawn((
                            Text::new(text),
                            TextFont { font_size: 16.0, ..default() },
                            TextColor(Color::WHITE),
                        ));
                    });
                }
            });
        });
    });
}


pub fn setup_load_game_menu(
    mut commands: Commands,
    save_manager: Res<SaveManager>,
) {
    let save_files = save_manager.list_save_files();
    
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
        MenuUI,
    )).with_children(|parent| {
        // Load game container
        parent.spawn((
            Node {
                width: Val::Px(800.0),
                height: Val::Px(600.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                padding: UiRect::all(Val::Px(40.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.95, 0.92, 0.88)),
            BorderColor(Color::srgb(0.6, 0.4, 0.2)),
        )).with_children(|load_menu| {
            // Title
            load_menu.spawn((
                Text::new("Load Game"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::srgb(0.3, 0.2, 0.1)),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));
            
            // Save slots grid
            load_menu.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(400.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    overflow: Overflow::scroll_y(),
                    ..default()
                },
            )).with_children(|slots| {
                // Show available save slots (0-9)
                for slot in 0..10 {
                    let save_info = save_files.iter().find(|s| s.slot == slot);
                    
                    let (bg_color, text_color, is_enabled) = if save_info.is_some() {
                        (Color::srgb(0.9, 0.9, 0.9), Color::srgb(0.2, 0.2, 0.2), true)
                    } else {
                        (Color::srgb(0.7, 0.7, 0.7), Color::srgb(0.5, 0.5, 0.5), false)
                    };
                    
                    let mut entity_commands = slots.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(60.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(15.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            margin: UiRect::vertical(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(bg_color),
                        BorderColor(Color::srgb(0.5, 0.5, 0.5)),
                        BorderRadius::all(Val::Px(6.0)),
                        SaveSlotCard { slot },
                    ));
                    
                    if is_enabled {
                        entity_commands.insert((
                            Button,
                            LoadGameButton { save_slot: slot },
                        ));
                    }
                    
                    entity_commands.with_children(|card| {
                        card.spawn((
                            Text::new(format!("Save Slot {}", slot)),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(text_color),
                        ));
                        
                        if let Some(info) = save_info {
                            if let Ok(modified) = info.last_modified.elapsed() {
                                let time_text = if modified.as_secs() < 60 {
                                    "Less than a minute ago".to_string()
                                } else if modified.as_secs() < 3600 {
                                    format!("{} minutes ago", modified.as_secs() / 60)
                                } else if modified.as_secs() < 86400 {
                                    format!("{} hours ago", modified.as_secs() / 3600)
                                } else {
                                    format!("{} days ago", modified.as_secs() / 86400)
                                };
                                
                                card.spawn((
                                    Text::new(time_text),
                                    TextFont {
                                        font_size: 12.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.5, 0.5, 0.5)),
                                ));
                            }
                        } else {
                            card.spawn((
                                Text::new("Empty"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.6, 0.6, 0.6)),
                            ));
                        }
                    });
                }
            });
            
            // Back button
            load_menu.spawn((
                Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
            )).with_children(|back_container| {
                back_container.spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.6, 0.5, 0.4)),
                    BorderRadius::all(Val::Px(6.0)),
                    SettingsButton { action: SettingsAction::BackToMain },
                )).with_children(|button| {
                    button.spawn((
                        Text::new("Back to Main Menu"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            });
        });
    });
}

// Interaction Systems

pub fn main_menu_button_system(
    mut interaction_query: Query<
        (&Interaction, &MainMenuButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu_nav_events: EventWriter<MenuNavigationEvent>,
    mut app_exit_events: EventWriter<AppExit>,
    mut _save_events: EventWriter<SaveGameEvent>,
    mut save_manager: ResMut<SaveManager>,
) {
    for (interaction, menu_button, mut bg_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = Color::srgb(0.5, 0.7, 0.5).into();
                
                match menu_button.action {
                    MainMenuAction::NewGame => {
                        // Start a new game - switch to playing state
                        save_manager.current_save_slot = Some(0); // Default to slot 0
                        menu_nav_events.write(MenuNavigationEvent {
                            target_menu: MenuType::InGame,
                            target_app_state: Some(crate::AppState::Playing),
                        });
                    }
                    MainMenuAction::LoadGame => {
                        menu_nav_events.write(MenuNavigationEvent {
                            target_menu: MenuType::LoadGame,
                            target_app_state: Some(crate::AppState::LoadGame),
                        });
                    }
                    MainMenuAction::Settings => {
                        menu_nav_events.write(MenuNavigationEvent {
                            target_menu: MenuType::Settings,
                            target_app_state: Some(crate::AppState::Settings),
                        });
                    }
                    MainMenuAction::Quit => {
                        app_exit_events.write(AppExit::Success);
                    }
                }
            }
            Interaction::Hovered => {
                *bg_color = Color::srgb(0.7, 0.6, 0.5).into();
            }
            Interaction::None => {
                *bg_color = Color::srgb(0.6, 0.5, 0.4).into();
            }
        }
    }
}

pub fn settings_button_system(
    mut interaction_query: Query<
        (&Interaction, &SettingsButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu_nav_events: EventWriter<MenuNavigationEvent>,
    mut settings: ResMut<GameSettings>,
) {
    for (interaction, settings_button, mut bg_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = Color::srgb(0.5, 0.7, 0.5).into();

            info!("Settings button pressed: {:?}", settings_button.action);
            match settings_button.action {
                SettingsAction::BackToMain => {
                    menu_nav_events.write(MenuNavigationEvent {
                        target_menu: MenuType::MainMenu,
                        target_app_state: Some(crate::AppState::MainMenu),
                    });
                }
                SettingsAction::ResetToDefaults => {
                    *settings = GameSettings::default();
                    info!("Settings reset to defaults");
                }
                SettingsAction::ApplySettings => {
                    if let Err(e) = settings.save_to_file() {
                        error!("Failed to save settings: {}", e);
                    } else {
                        info!("Settings saved successfully");
                    }
                }
                SettingsAction::OpenControls => {
                    menu_nav_events.write(MenuNavigationEvent {
                        target_menu: MenuType::SettingsControls,
                        target_app_state: None,
                    });
                }
                SettingsAction::BackToSettings => {
                    menu_nav_events.write(MenuNavigationEvent {
                        target_menu: MenuType::Settings,
                        target_app_state: Some(crate::AppState::Settings),
                    });
                }
            }
            }
            Interaction::Hovered => {
                *bg_color = Color::srgb(0.7, 0.6, 0.5).into();
            }
            Interaction::None => {
                *bg_color = Color::srgb(0.6, 0.5, 0.4).into();
            }            
        }
    }
}

pub fn graphics_toggle_system(
    mut interaction_query: Query<
        (&Interaction, &GraphicsToggle, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut settings: ResMut<GameSettings>,
) {
    for (interaction, toggle, children) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match toggle.setting_type {
                GraphicsSettingType::VSync => {
                    settings.vsync_enabled = !settings.vsync_enabled;
                    info!("VSync toggled: {}", settings.vsync_enabled);
                    
                    // Update text display
                    for child in children.iter() {
                        if let Ok(mut text) = text_query.get_mut(child) {
                            if text.contains("ON") || text.contains("OFF") {
                                **text = if settings.vsync_enabled { "ON" } else { "OFF" }.to_string();
                            }
                        }
                    }
                }
                GraphicsSettingType::Fullscreen => {
                    settings.fullscreen = !settings.fullscreen;
                    info!("Fullscreen toggled: {}", settings.fullscreen);
                    
                    // Update text display
                    for child in children.iter() {
                        if let Ok(mut text) = text_query.get_mut(child) {
                            if text.contains("ON") || text.contains("OFF") {
                                **text = if settings.fullscreen { "ON" } else { "OFF" }.to_string();
                            }
                        }
                    }
                }
                GraphicsSettingType::Resolution => {
                    // These are handled by the new dropdown systems
                }
                GraphicsSettingType::GraphicsQuality => {
                    // These are handled by the new dropdown systems
                }
            }
        }
    }
}

pub fn load_game_button_system(
    mut interaction_query: Query<
        (&Interaction, &LoadGameButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut load_events: EventWriter<LoadGameEvent>,
    mut menu_nav_events: EventWriter<MenuNavigationEvent>,
    mut save_manager: ResMut<SaveManager>,
) {
    for (interaction, load_button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            save_manager.current_save_slot = Some(load_button.save_slot);
            load_events.write(LoadGameEvent {
                slot: load_button.save_slot,
            });
            
            menu_nav_events.write(MenuNavigationEvent {
                target_menu: MenuType::InGame,
                target_app_state: Some(crate::AppState::Playing),
            });
        }
    }
}

pub fn menu_navigation_system(
    mut menu_nav_events: EventReader<MenuNavigationEvent>,
    mut menu_state: ResMut<MenuState>,
    mut app_state: ResMut<NextState<crate::AppState>>,
) {
    for nav_event in menu_nav_events.read() {
        info!("Processing menu navigation event: {:?} -> {:?}", nav_event.target_menu, nav_event.target_app_state);
        menu_state.previous_menu = Some(menu_state.current_menu);
        menu_state.current_menu = nav_event.target_menu;
        
        if let Some(target_state) = &nav_event.target_app_state {
            app_state.set(*target_state);
        }
        
        // Handle special menu setups
        match nav_event.target_menu {
            MenuType::SettingsControls => {
                // Controls menu will be handled in setup_controls_menu
            }
            _ => {}
        }
    }
}

pub fn handle_controls_menu(
    mut commands: Commands,
    menu_state: Res<MenuState>,
    keybindings: Res<crate::keybindings::KeyBindings>,
    menu_query: Query<Entity, With<MenuUI>>,
) {
    if menu_state.is_changed() && menu_state.current_menu == MenuType::SettingsControls {
        // Clear existing UI
        for entity in &menu_query {
            commands.entity(entity).despawn();
        }
        
        // Setup controls menu
        setup_controls_menu(commands, keybindings);
    }
}

pub fn tab_test_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    existing_tab_query: Query<Entity, With<TabGroup>>,
) {
    if keyboard.just_pressed(KeyCode::KeyT) {
        info!("T key pressed - spawning tab group test");
        
        // Remove existing tab group if present
        for entity in existing_tab_query.iter() {
            commands.entity(entity).despawn();
        }
        
        // Create tab group test
        let tab_group_entity = commands.spawn((
            TabTestWindow,
            Node {
                width: Val::Px(800.0),
                height: Val::Px(600.0),
                position_type: PositionType::Absolute,
                left: Val::Px(50.0),
                top: Val::Px(50.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.9, 0.9, 0.9)),
            BorderRadius::all(Val::Px(8.0)),
            TabGroup { selected_tab: 0 },
            TabGroupConfig::default(),
        )).id();
        
        // Create individual entities first, then set up the hierarchy
        
        // Create tab buttons first
        let tab1_button = commands.spawn((
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.9, 0.9, 0.9)), // Active color
            TabButton { tab_index: 0, group_entity: tab_group_entity },
            TabActive, // Tab 1 starts active
        )).id();
        
        commands.entity(tab1_button).with_children(|button| {
            button.spawn((
                Text::new("Tab 1"),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::BLACK),
            ));
        });
        
        let tab2_button = commands.spawn((
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.65, 0.65, 0.65)), // Inactive color
            TabButton { tab_index: 1, group_entity: tab_group_entity },
            TabInactive, // Tab 2 starts inactive
        )).id();
        
        commands.entity(tab2_button).with_children(|button| {
            button.spawn((
                Text::new("Tab 2"),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::BLACK),
            ));
        });
        
        // Create content entities
        let content1 = commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.95, 0.95, 0.95)),
            TabContent { tab_index: 0, group_entity: tab_group_entity },
            Visibility::Visible,
        )).id();
        
        commands.entity(content1).with_children(|content| {
            content.spawn((
                Text::new("This is Tab 1 Content"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::BLACK),
            ));
        });
        
        let content2 = commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.95, 0.95, 1.0)),
            TabContent { tab_index: 1, group_entity: tab_group_entity },
            Visibility::Hidden,
        )).id();
        
        commands.entity(content2).with_children(|content| {
            content.spawn((
                Text::new("This is Tab 2 Content"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::BLACK),
            ));
        });
        
        // Create the UI hierarchy
        let tab_bar = commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            BackgroundColor(Color::srgb(0.8, 0.8, 0.8)),
        )).id();
        
        // Set up the hierarchy
        commands.entity(tab_group_entity)
            .add_child(tab_bar)
            .add_child(content1)
            .add_child(content2);
            
        commands.entity(tab_bar)
            .add_child(tab1_button)
            .add_child(tab2_button);
        
        // Add TabGroupMeta with all the collected entities
        commands.entity(tab_group_entity).insert(TabGroupMeta {
            tab_names: vec!["Tab 1".to_string(), "Tab 2".to_string()],
            button_entities: vec![tab1_button, tab2_button],
            content_entities: vec![content1, content2],
        });
    }
}

pub fn tab_test_escape_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    tab_test_query: Query<Entity, With<TabTestWindow>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        for entity in tab_test_query.iter() {
            #[cfg(debug_assertions)]
            info!("Escape pressed - closing tab test window {:?}", entity);
            
            commands.entity(entity).despawn();
        }
    }
}

pub fn escape_key_system(
    input: Res<ButtonInput<KeyCode>>,
    menu_state: Res<MenuState>,
    mut menu_nav_events: EventWriter<MenuNavigationEvent>,
    current_state: Res<State<crate::AppState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            crate::AppState::Settings => {
                menu_nav_events.write(MenuNavigationEvent {
                    target_menu: MenuType::MainMenu,
                    target_app_state: Some(crate::AppState::MainMenu),
                });
            }
            crate::AppState::LoadGame => {
                menu_nav_events.write(MenuNavigationEvent {
                    target_menu: MenuType::MainMenu,
                    target_app_state: Some(crate::AppState::MainMenu),
                });
            }
            crate::AppState::Playing => {
                menu_nav_events.write(MenuNavigationEvent {
                    target_menu: MenuType::MainMenu,
                    target_app_state: Some(crate::AppState::MainMenu),
                });
            }
            _ => {}
        }
    }
}

pub fn cleanup_menu_ui(
    mut commands: Commands,
    menu_query: Query<Entity, With<MenuUI>>,
) {
    for entity in menu_query.iter() {
        commands.entity(entity).safe_despawn();
    }
}

// System to add resolution dropdown after UI setup
pub fn setup_resolution_dropdown_system(
    mut commands: Commands,
    settings: Res<GameSettings>,
    mut option_registry: ResMut<crate::user_interface::dropdown::DropdownOptionRegistry>,
    graphics_section_query: Query<Entity, With<GraphicsSection>>,
    label_query: Query<Entity, With<ResolutionDropdownLabel>>,
    dropdown_query: Query<Entity, With<ResolutionDropdown>>,
) {
    // Only run if no resolution dropdown exists yet to prevent infinite spawning
    if !dropdown_query.is_empty() {
        return;
    }
    
    info!("Setting up resolution dropdown system");
    
    // Find the graphics section and add the dropdown after the resolution label
    if let Some(label_entity) = label_query.iter().next() {
        info!("Found resolution label entity: {:?}", label_entity);
        let resolutions = GameSettings::get_common_resolutions();
        
        // Build dropdown with resolution options
        let mut dropdown_builder = DropdownBuilder::new();
        for (width, height) in &resolutions {
            dropdown_builder = dropdown_builder.with_option(format!("{}x{}", width, height), None);
        }
        
        let config = DropdownConfig {
            placeholder: format!("{}x{}", settings.window_resolution.0, settings.window_resolution.1),
            ..Default::default()
        };
        
        let dropdown_spawn_cmd = dropdown_builder
            .with_config(config)
            .build();
            
        let dropdown_entity = dropdown_spawn_cmd.spawn(&mut commands, &mut option_registry);
        info!("Created dropdown entity: {:?}", dropdown_entity);
        
        // Add marker component to identify this as the resolution dropdown
        commands.entity(dropdown_entity).insert(ResolutionDropdown);
        
        // Add it as a child of the graphics section (same parent as the label)
        for graphics_entity in graphics_section_query.iter() {
            info!("Adding dropdown to graphics section: {:?}", graphics_entity);
            commands.entity(graphics_entity).add_children(&[dropdown_entity]);
            break;
        }
    } else {
        info!("Resolution label not found!");
    }
}

// System to add volume sliders to the audio section after UI setup
pub fn setup_audio_sliders_system(
    mut commands: Commands,
    settings: Res<GameSettings>,
    audio_section_query: Query<Entity, With<AudioSection>>,
    volume_slider_query: Query<Entity, With<VolumeSlider>>,
) {
    // Only run if no volume sliders exist yet to prevent infinite spawning
    if !volume_slider_query.is_empty() {
        return;
    }
    
    // Add sliders to the audio section with side-by-side layout
    for section_entity in audio_section_query.iter() {
        // Master Volume Slider (side-by-side layout)
        let master_container = commands.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(10.0)),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.9, 0.9, 0.9)),
            BorderRadius::all(Val::Px(6.0)),
        )).id();
        
        let master_label = commands.spawn((
            Text::new("Master Volume"),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::srgb(0.3, 0.2, 0.1)),
        )).id();
        
        let master_slider = SliderBuilder::new(&mut commands)
            .with_range(0.0, 1.0)
            .with_value(settings.master_volume)
            .with_value_formatter(|value| format!("{}%", (value * 100.0) as u32))
            .spawn();
            
        commands.entity(master_slider).insert(VolumeSlider { 
            setting_type: SettingType::MasterVolume 
        });
        
        commands.entity(master_container).add_children(&[master_label, master_slider]);
        commands.entity(section_entity).add_children(&[master_container]);
        
        // Music Volume Slider (side-by-side layout)
        let music_container = commands.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(10.0)),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.9, 0.9, 0.9)),
            BorderRadius::all(Val::Px(6.0)),
        )).id();
        
        let music_label = commands.spawn((
            Text::new("Music Volume"),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::srgb(0.3, 0.2, 0.1)),
        )).id();
        
        let music_slider = SliderBuilder::new(&mut commands)
            .with_range(0.0, 1.0)
            .with_value(settings.music_volume)
            .with_value_formatter(|value| format!("{}%", (value * 100.0) as u32))
            .spawn();
            
        commands.entity(music_slider).insert(VolumeSlider { 
            setting_type: SettingType::MusicVolume 
        });
        
        commands.entity(music_container).add_children(&[music_label, music_slider]);
        commands.entity(section_entity).add_children(&[music_container]);
        
        // SFX Volume Slider (side-by-side layout)
        let sfx_container = commands.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(10.0)),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.9, 0.9, 0.9)),
            BorderRadius::all(Val::Px(6.0)),
        )).id();
        
        let sfx_label = commands.spawn((
            Text::new("SFX Volume"),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::srgb(0.3, 0.2, 0.1)),
        )).id();
        
        let sfx_slider = SliderBuilder::new(&mut commands)
            .with_range(0.0, 1.0)
            .with_value(settings.sfx_volume)
            .with_value_formatter(|value| format!("{}%", (value * 100.0) as u32))
            .spawn();
            
        commands.entity(sfx_slider).insert(VolumeSlider { 
            setting_type: SettingType::SfxVolume 
        });
        
        commands.entity(sfx_container).add_children(&[sfx_label, sfx_slider]);
        commands.entity(section_entity).add_children(&[sfx_container]);
    }
}

// System to handle volume slider changes
pub fn volume_slider_update_system(
    mut slider_events: EventReader<SliderValueChangedEvent>,
    volume_slider_query: Query<&VolumeSlider>,
    mut game_settings: ResMut<GameSettings>,
    mut audio_settings: ResMut<AudioSettings>,
) {
    for event in slider_events.read() {
        if let Ok(volume_slider) = volume_slider_query.get(event.entity) {
            match volume_slider.setting_type {
                SettingType::MasterVolume => {
                    game_settings.master_volume = event.new_value;
                    audio_settings.volume = event.new_value;
                }
                SettingType::MusicVolume => {
                    game_settings.music_volume = event.new_value;
                }
                SettingType::SfxVolume => {
                    game_settings.sfx_volume = event.new_value;
                }
                _ => {}
            }
            
            // Auto-save settings when changed
            if let Err(e) = game_settings.save_to_file() {
                eprintln!("Failed to save settings: {}", e);
            }
        }
    }
}

// Simplified dropdown systems (cycle through options on click)
pub fn resolution_dropdown_system(
    mut dropdown_events: EventReader<DropdownChangedEvent>,
    dropdown_query: Query<&crate::user_interface::dropdown::Dropdown, With<ResolutionDropdown>>,
    mut settings: ResMut<GameSettings>,
) {
    for event in dropdown_events.read() {
        // Check if this event is from our resolution dropdown
        if dropdown_query.get(event.dropdown_entity).is_ok() && 
           event.kind == DropdownChangeKind::SelectionChanged {
            
            if let Some(new_label) = &event.new_label {
                // Parse resolution from label like "1920x1080"
                if let Some((width_str, height_str)) = new_label.split_once('x') {
                    if let (Ok(width), Ok(height)) = (width_str.parse::<u32>(), height_str.parse::<u32>()) {
                        settings.window_resolution = (width, height);
                        info!("Resolution changed to: {}x{}", width, height);
                        
                        // Auto-save settings when changed
                        if let Err(e) = settings.save_to_file() {
                            eprintln!("Failed to save settings: {}", e);
                        }
                    }
                }
            }
        }
    }
}

pub fn graphics_quality_dropdown_system(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<GraphicsQualityDropdown>)>,
    mut text_query: Query<&mut Text>,
    mut settings: ResMut<GameSettings>,
) {
    for (interaction, children) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            let qualities = GraphicsQuality::all_qualities();
            let current_index = settings.graphics_quality.index();
            let next_index = (current_index + 1) % qualities.len();
            
            if let Some(&new_quality) = qualities.get(next_index) {
                settings.graphics_quality = new_quality;
                info!("Graphics quality changed to: {}", new_quality.to_string());
                
                // Update display text
                for child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(child) {
                        if text.contains("▼") && !text.contains("Graphics Quality") {
                            **text = format!("{} ▼", new_quality.to_string());
                        }
                    }
                }
                
                // Auto-save settings when changed
                if let Err(e) = settings.save_to_file() {
                    eprintln!("Failed to save settings: {}", e);
                }
            }
        }
    }
}

pub fn settings_toggle_system(
    mut interaction_query: Query<(Entity, &Interaction, &mut ToggleButton, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
    mut settings: ResMut<GameSettings>,
) {
    for (entity, interaction, mut toggle, children) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            toggle.toggle();
            
            // Determine which setting was toggled based on label
            match toggle.label.as_str() {
                "VSync" => {
                    settings.vsync_enabled = toggle.is_on;
                    info!("VSync toggled: {}", toggle.is_on);
                }
                "Fullscreen" => {
                    settings.fullscreen = toggle.is_on;
                    info!("Fullscreen toggled: {}", toggle.is_on);
                }
                _ => continue,
            }
            
            // Update button text display
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    if text.contains("ON") || text.contains("OFF") {
                        **text = if toggle.is_on { "ON" } else { "OFF" }.to_string();
                    }
                }
            }
            
            // Auto-save settings when changed
            if let Err(e) = settings.save_to_file() {
                eprintln!("Failed to save settings: {}", e);
            }
        }
    }
}

/// Handle toggle widget changes (StateScoped implementation)
pub fn fullscreen_toggle_system(
    mut toggle_events: EventReader<ToggleChangedEvent>,
    mut settings: ResMut<GameSettings>,
    toggle_query: Query<&crate::user_interface::toggle::Toggle, With<FullscreenToggle>>,
) {
    for event in toggle_events.read() {
        if let Ok(toggle) = toggle_query.get(event.toggle_entity) {
            settings.fullscreen = toggle.is_on;
            
            #[cfg(debug_assertions)]
            info!("StateScoped fullscreen toggle changed: {} -> {}", event.previous_state, event.new_state);
            
            info!("Fullscreen setting updated: {}", toggle.is_on);
            
            // Auto-save settings when changed
            if let Err(e) = settings.save_to_file() {
                error!("Failed to save graphics settings: {}", e);
            }
        }
    }
}