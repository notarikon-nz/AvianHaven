use bevy::prelude::*;
use crate::menu::{components::*, resources::*};
use crate::save_load::resources::{SaveGameEvent, LoadGameEvent, SaveManager};

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
        )).with_children(|menu| {
            // Title
            menu.spawn((
                Text::new("🐦 Avian Haven"),
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
        // Settings container
        parent.spawn((
            Node {
                width: Val::Px(600.0),
                height: Val::Px(700.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                padding: UiRect::all(Val::Px(40.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.95, 0.92, 0.88)),
            BorderColor(Color::srgb(0.6, 0.4, 0.2)),
        )).with_children(|settings_menu| {
            // Title
            settings_menu.spawn((
                Text::new("⚙️ Settings"),
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
            
            // Audio settings section
            let audio_settings = [
                ("Master Volume", SettingType::MasterVolume, settings.master_volume),
                ("Music Volume", SettingType::MusicVolume, settings.music_volume),
                ("SFX Volume", SettingType::SfxVolume, settings.sfx_volume),
            ];
            
            settings_menu.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    margin: UiRect::vertical(Val::Px(20.0)),
                    ..default()
                },
            )).with_children(|section| {
                section.spawn((
                    Text::new("🔊 Audio"),
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
                
                for (label, _setting_type, value) in &audio_settings {
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
            
            // Gameplay settings section
            let gameplay_settings = [
                ("Auto Save", SettingType::AutoSave, if settings.auto_save_enabled { 1.0 } else { 0.0 }),
            ];
            
            settings_menu.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    margin: UiRect::vertical(Val::Px(20.0)),
                    ..default()
                },
            )).with_children(|section| {
                section.spawn((
                    Text::new("🎮 Gameplay"),
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
            
            // Bottom buttons
            settings_menu.spawn((
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
                Text::new("📁 Load Game"),
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
    mut save_events: EventWriter<SaveGameEvent>,
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
                        menu_nav_events.send(MenuNavigationEvent {
                            target_menu: MenuType::InGame,
                            target_app_state: Some(crate::AppState::Playing),
                        });
                    }
                    MainMenuAction::LoadGame => {
                        menu_nav_events.send(MenuNavigationEvent {
                            target_menu: MenuType::LoadGame,
                            target_app_state: Some(crate::AppState::LoadGame),
                        });
                    }
                    MainMenuAction::Settings => {
                        menu_nav_events.send(MenuNavigationEvent {
                            target_menu: MenuType::Settings,
                            target_app_state: Some(crate::AppState::Settings),
                        });
                    }
                    MainMenuAction::Quit => {
                        app_exit_events.send(AppExit::Success);
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
        (&Interaction, &SettingsButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu_nav_events: EventWriter<MenuNavigationEvent>,
    mut settings: ResMut<GameSettings>,
) {
    for (interaction, settings_button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match settings_button.action {
                SettingsAction::BackToMain => {
                    menu_nav_events.send(MenuNavigationEvent {
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
            load_events.send(LoadGameEvent {
                slot: load_button.save_slot,
            });
            
            menu_nav_events.send(MenuNavigationEvent {
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
        menu_state.previous_menu = Some(menu_state.current_menu);
        menu_state.current_menu = nav_event.target_menu;
        
        if let Some(target_state) = &nav_event.target_app_state {
            app_state.set(*target_state);
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
                menu_nav_events.send(MenuNavigationEvent {
                    target_menu: MenuType::MainMenu,
                    target_app_state: Some(crate::AppState::MainMenu),
                });
            }
            crate::AppState::LoadGame => {
                menu_nav_events.send(MenuNavigationEvent {
                    target_menu: MenuType::MainMenu,
                    target_app_state: Some(crate::AppState::MainMenu),
                });
            }
            crate::AppState::Playing => {
                menu_nav_events.send(MenuNavigationEvent {
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
        commands.entity(entity).despawn_recursive();
    }
}