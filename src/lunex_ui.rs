use bevy::prelude::*;
use bevy_lunex::prelude::*;

// Lunex UI Migration Module
// This module provides a gradual migration path from Bevy UI to Lunex UI
// Starting with simple components and expanding to complex layouts

pub struct LunexUiPlugin;

impl Plugin for LunexUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UiLunexPlugins)
            .add_systems(Startup, setup_lunex_system)
            // .add_systems(OnEnter(crate::AppState::MainMenu), setup_lunex_main_menu) // Temporarily disabled
            // .add_systems(OnExit(crate::AppState::MainMenu), cleanup_lunex_main_menu) // Temporarily disabled
            // .add_systems(Update, handle_lunex_main_menu_clicks.run_if(in_state(crate::AppState::MainMenu))) // Temporarily disabled
            .add_systems(Update, (
                create_lunex_settings_buttons,
                handle_lunex_button_clicks,
            ).run_if(in_state(crate::AppState::Settings)))
            .add_systems(Update, (
                setup_lunex_tutorial_ui,
                handle_lunex_tutorial_buttons,
                update_lunex_tutorial_content,
            ).run_if(in_state(crate::AppState::Playing)))
            .add_systems(OnEnter(crate::AppState::Journal), setup_lunex_journal)
            .add_systems(OnExit(crate::AppState::Journal), cleanup_lunex_journal)
            .add_systems(Update, (
                handle_lunex_journal_navigation,
                update_lunex_journal_content,
            ).run_if(in_state(crate::AppState::Journal)))
            .add_systems(Update, (
                setup_lunex_catalog_ui,
                handle_lunex_catalog_input,
                update_lunex_catalog_content,
            ).run_if(in_state(crate::AppState::Playing)));
    }
}

pub fn setup_lunex_system() {
    info!("Initializing Lunex UI system for gradual migration");
    
    // Foundation setup for Lunex UI migration
    // Individual roots will be created per app state as needed
}

// Migration utilities for converting Bevy UI to Lunex UI

#[derive(Component)]
pub struct LunexMigrationMarker;

// Migration helpers for common UI patterns

pub fn create_lunex_button(
    commands: &mut Commands,
    text: &str,
    width: f32,
    height: f32,
    x: f32,
    y: f32,
) -> Entity {
    commands.spawn((
        UiLayout::window().pos(Rl((x, y))).size((Rl(width), Rl(height))).pack(),
        UiColor::new(vec![
            (UiBase::id(), Color::srgb(0.6, 0.5, 0.4)),
            (UiHover::id(), Color::srgb(0.7, 0.6, 0.5)),
        ]),
        UiHover::new().forward_speed(20.0).backward_speed(4.0),
        LunexMigrationMarker,
        Name::new(format!("Lunex Button: {}", text)),
    )).with_children(|button| {
        button.spawn((
            Text2d::new(text),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            UiTextSize::from(Rh(60.0)),
            UiLayout::window().pack(),
            TextColor(Color::WHITE),
        ));
    }).id()
}

pub fn create_lunex_text_label(
    commands: &mut Commands,
    text: &str,
    font_size: f32,
    x: f32,
    y: f32,
) -> Entity {
    commands.spawn((
        Text2d::new(text),
        TextFont {
            font_size,
            ..default()
        },
        UiLayout::window().pos(Rl((x, y))).pack(),
        LunexMigrationMarker,
        Name::new(format!("Lunex Text: {}", text)),
    )).id()
}

// Migration priority components - which UI elements to migrate first

#[derive(Component)]
pub struct MigrationPriority {
    pub priority: u32, // Lower number = higher priority
    pub complexity: MigrationComplexity,
}

#[derive(Debug, Clone)]
pub enum MigrationComplexity {
    Simple,    // Basic buttons, text labels
    Medium,    // Layout containers, lists
    Complex,   // Interactive widgets, animations
    Advanced,  // Custom drawing, complex state
}

impl MigrationPriority {
    pub fn simple(priority: u32) -> Self {
        Self { priority, complexity: MigrationComplexity::Simple }
    }
    
    pub fn medium(priority: u32) -> Self {
        Self { priority, complexity: MigrationComplexity::Medium }
    }
    
    pub fn complex(priority: u32) -> Self {
        Self { priority, complexity: MigrationComplexity::Complex }
    }
}

// Recommended migration order:
// 1. Simple buttons (Settings menu buttons) - Priority 1
// 2. Text labels and static content - Priority 2  
// 3. Layout containers (menu panels) - Priority 3
// 4. Interactive widgets (sliders, toggles) - Priority 4
// 5. Complex layouts (journal tabs) - Priority 5
// 6. Advanced features (tooltips, animations) - Priority 6

// Lunex settings button component
#[derive(Component)]
pub struct LunexSettingsButton {
    pub action: crate::menu::components::SettingsAction,
}

// System to create Lunex versions of settings buttons
pub fn create_lunex_settings_buttons(
    mut commands: Commands,
    lunex_root_query: Query<Entity, (With<UiLayoutRoot>, Without<LunexSettingsButton>)>,
    settings_button_query: Query<&crate::menu::components::SettingsButton>,
) {
    // Only create buttons if we have settings buttons but no Lunex versions yet
    if settings_button_query.is_empty() {
        return;
    }
    
    // Find the existing Lunex root or create one
    let lunex_root = if let Ok(root_entity) = lunex_root_query.single() {
        root_entity
    } else {
        return; // No root available yet
    };
    
    // Add Lunex versions of settings buttons
    commands.entity(lunex_root).with_children(|ui| {
        let buttons = [
            ("Back to Main", crate::menu::components::SettingsAction::BackToMain),
            ("Apply Settings", crate::menu::components::SettingsAction::ApplySettings),
            ("Reset Defaults", crate::menu::components::SettingsAction::ResetToDefaults),
            ("Controls", crate::menu::components::SettingsAction::OpenControls),
        ];
        
        for (i, (text, action)) in buttons.iter().enumerate() {
            ui.spawn((
                UiLayout::window().pos(Rl((300.0, 100.0 + i as f32 * 60.0))).size((Rl(180.0), Rl(40.0))).pack(),
                UiColor::new(vec![
                    (UiBase::id(), Color::srgb(0.6, 0.5, 0.4)),
                    (UiHover::id(), Color::srgb(0.7, 0.6, 0.5)),
                ]),
                UiHover::new().forward_speed(20.0).backward_speed(4.0),
                // UiPressable doesn't exist, use UiHover for now
                LunexSettingsButton { action: *action },
                LunexMigrationMarker,
                Name::new(format!("Lunex Settings Button: {}", text)),
            )).with_children(|button| {
                button.spawn((
                    Text2d::new(*text),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    UiTextSize::from(Rh(60.0)),
                    UiLayout::window().pack(),
                    TextColor(Color::WHITE),
                ));
            });
        }
    });
}

// System to handle Lunex button clicks
pub fn handle_lunex_button_clicks(
    mut interaction_query: Query<(&LunexSettingsButton, &UiHover), Changed<UiHover>>,
    mut menu_nav_events: EventWriter<crate::menu::resources::MenuNavigationEvent>,
    mut settings: ResMut<crate::menu::resources::GameSettings>,
) {
    for (settings_button, hover) in interaction_query.iter() {
        // Temporarily disabled to prevent accidental triggering - need proper click detection  
        if false {
            info!("Lunex button pressed: {:?}", settings_button.action);
            
            match settings_button.action {
                crate::menu::components::SettingsAction::BackToMain => {
                    menu_nav_events.write(crate::menu::resources::MenuNavigationEvent {
                        target_menu: crate::menu::resources::MenuType::MainMenu,
                        target_app_state: Some(crate::AppState::MainMenu),
                    });
                }
                crate::menu::components::SettingsAction::ApplySettings => {
                    if let Err(e) = settings.save_to_file() {
                        error!("Failed to save settings: {}", e);
                    } else {
                        info!("Settings saved successfully via Lunex button");
                    }
                }
                crate::menu::components::SettingsAction::ResetToDefaults => {
                    *settings = crate::menu::resources::GameSettings::default();
                    info!("Settings reset to defaults via Lunex button");
                }
                crate::menu::components::SettingsAction::OpenControls => {
                    menu_nav_events.write(crate::menu::resources::MenuNavigationEvent {
                        target_menu: crate::menu::resources::MenuType::SettingsControls,
                        target_app_state: None,
                    });
                }
                crate::menu::components::SettingsAction::BackToSettings => {
                    menu_nav_events.write(crate::menu::resources::MenuNavigationEvent {
                        target_menu: crate::menu::resources::MenuType::Settings,
                        target_app_state: Some(crate::AppState::Settings),
                    });
                }
            }
        }
    }
}

// Lunex main menu button component
#[derive(Component)]
pub struct LunexMainMenuButton {
    pub action: crate::menu::components::MainMenuAction,
}

// System to cleanup Lunex main menu on state exit
pub fn cleanup_lunex_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<LunexMainMenuButton>>,
    root_query: Query<Entity, With<UiLayoutRoot>>,
) {
    // Clean up all main menu entities
    for entity in main_menu_query.iter() {
        commands.entity(entity).despawn();
    }
    
    // Clean up root if it exists
    for entity in root_query.iter() {
        commands.entity(entity).despawn();
    }
}

// System to setup Lunex main menu on state entry  
pub fn setup_lunex_main_menu(mut commands: Commands) {
    // Create Lunex UI root for main menu
    commands.spawn((
        UiLayoutRoot::new_2d(),
        UiFetchFromCamera::<0>,
        Name::new("Lunex Main Menu Root"),
    )).with_children(|ui| {
        // Main menu container - centered on screen
        ui.spawn((
            UiLayout::window().pos(Rl((300.0, 150.0))).size((Rl(400.0), Rl(500.0))).pack(),
            UiColor::new(vec![
                (UiBase::id(), Color::srgb(0.95, 0.92, 0.88)),
            ]),
            Name::new("Lunex Main Menu Container"),
            LunexMigrationMarker,
        )).with_children(|menu_container| {
            // Title
            menu_container.spawn((
                Text2d::new("Avian Haven"),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                UiTextSize::from(Rh(80.0)),
                UiLayout::window().pos(Rl((0.0, 50.0))).pack(),
                TextColor(Color::srgb(0.3, 0.2, 0.1)),
            ));
            
            // Menu buttons
            let buttons = [
                ("New Game", crate::menu::components::MainMenuAction::NewGame),
                ("Load Game", crate::menu::components::MainMenuAction::LoadGame),
                ("Settings", crate::menu::components::MainMenuAction::Settings),
                ("Quit", crate::menu::components::MainMenuAction::Quit),
            ];
            
            for (i, (text, action)) in buttons.iter().enumerate() {
                menu_container.spawn((
                    UiLayout::window().pos(Rl((50.0, 150.0 + i as f32 * 70.0))).size((Rl(300.0), Rl(50.0))).pack(),
                    UiColor::new(vec![
                        (UiBase::id(), Color::srgb(0.6, 0.5, 0.4)),
                        (UiHover::id(), Color::srgb(0.7, 0.6, 0.5)),
                    ]),
                    UiHover::new().forward_speed(20.0).backward_speed(4.0),
                    LunexMainMenuButton { action: *action },
                    LunexMigrationMarker,
                    Name::new(format!("Lunex Main Menu Button: {}", text)),
                )).with_children(|button| {
                    button.spawn((
                        Text2d::new(*text),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        UiTextSize::from(Rh(60.0)),
                        UiLayout::window().pack(),
                        TextColor(Color::WHITE),
                    ));
                });
            }
        });
    });
}

// System to create Lunex versions of main menu layout (deprecated - use setup_lunex_main_menu)
pub fn create_lunex_main_menu_layout(
    mut commands: Commands,
    lunex_root_query: Query<Entity, (With<UiLayoutRoot>, Without<LunexMainMenuButton>)>,
    main_menu_button_query: Query<&crate::menu::components::MainMenuButton>,
) {
    // Only create if we have main menu buttons but no Lunex versions yet
    if main_menu_button_query.is_empty() {
        return;
    }
    
    // Find the existing Lunex root or create one
    let lunex_root = if let Ok(root_entity) = lunex_root_query.single() {
        root_entity
    } else {
        return; // No root available yet
    };
    
    // Add Lunex versions of main menu
    commands.entity(lunex_root).with_children(|ui| {
        // Main menu container - positioned on the left side
        ui.spawn((
            UiLayout::window().pos(Rl((50.0, 50.0))).size((Rl(350.0), Rl(450.0))).pack(),
            UiColor::new(vec![
                (UiBase::id(), Color::srgb(0.95, 0.92, 0.88)),
            ]),
            Name::new("Lunex Main Menu Container"),
            LunexMigrationMarker,
        )).with_children(|menu_container| {
            // Title
            menu_container.spawn((
                Text2d::new("Avian Haven (Lunex)"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                UiTextSize::from(Rh(80.0)),
                UiLayout::window().pos(Rl((0.0, 50.0))).pack(),
                TextColor(Color::srgb(0.3, 0.2, 0.1)),
            ));
            
            // Menu buttons
            let buttons = [
                ("New Game", crate::menu::components::MainMenuAction::NewGame),
                ("Load Game", crate::menu::components::MainMenuAction::LoadGame),
                ("Settings", crate::menu::components::MainMenuAction::Settings),
                ("Quit", crate::menu::components::MainMenuAction::Quit),
            ];
            
            for (i, (text, action)) in buttons.iter().enumerate() {
                menu_container.spawn((
                    UiLayout::window().pos(Rl((25.0, 150.0 + i as f32 * 70.0))).size((Rl(300.0), Rl(50.0))).pack(),
                    UiColor::new(vec![
                        (UiBase::id(), Color::srgb(0.6, 0.5, 0.4)),
                        (UiHover::id(), Color::srgb(0.7, 0.6, 0.5)),
                    ]),
                    UiHover::new().forward_speed(20.0).backward_speed(4.0),
                    LunexMainMenuButton { action: *action },
                    LunexMigrationMarker,
                    Name::new(format!("Lunex Main Menu Button: {}", text)),
                )).with_children(|button| {
                    button.spawn((
                        Text2d::new(*text),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        UiTextSize::from(Rh(60.0)),
                        UiLayout::window().pack(),
                        TextColor(Color::WHITE),
                    ));
                });
            }
        });
    });
}

// System to handle Lunex main menu button clicks
pub fn handle_lunex_main_menu_clicks(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut app_state: ResMut<NextState<crate::AppState>>,
) {
    // Temporary keyboard controls for testing Lunex main menu:
    // 1 = New Game, 2 = Load Game, 3 = Settings, 4 = Quit
    if keyboard.just_pressed(KeyCode::Digit1) {
        info!("Lunex main menu: New Game");
        app_state.set(crate::AppState::Playing);
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        info!("Lunex main menu: Load Game");
        app_state.set(crate::AppState::LoadGame);
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        info!("Lunex main menu: Settings");
        app_state.set(crate::AppState::Settings);
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        info!("Lunex main menu: Quit");
        // Quit functionality would go here
    }
}

// Tutorial System Migration Components
#[derive(Component)]
pub struct LunexTutorialUI;

#[derive(Component)]
pub struct LunexTutorialDialog;

#[derive(Component)]
pub struct LunexTutorialButton {
    pub action: LunexTutorialAction,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LunexTutorialAction {
    Next,
    Skip,
}

// System to setup Lunex tutorial UI
pub fn setup_lunex_tutorial_ui(
    mut commands: Commands,
    tutorial_state: Res<crate::tutorial::resources::TutorialState>,
    tutorial_ui_query: Query<Entity, With<LunexTutorialUI>>,
    lunex_root_query: Query<Entity, With<UiLayoutRoot>>,
) {
    // Only create tutorial UI if tutorial is active, UI should show, and we don't already have Lunex UI
    if !tutorial_state.is_active || !tutorial_state.show_ui || !tutorial_ui_query.is_empty() {
        return;
    }
    
    // Find or create Lunex root
    let lunex_root = if let Ok(root_entity) = lunex_root_query.single() {
        root_entity
    } else {
        // Create root if it doesn't exist
        commands.spawn((
            UiLayoutRoot::new_2d(),
            UiFetchFromCamera::<0>,
            Name::new("Lunex Tutorial Root"),
        )).id()
    };
    
    // Add tutorial UI to the root
    commands.entity(lunex_root).with_children(|ui| {
        // Tutorial dialog box - positioned bottom-left
        ui.spawn((
            UiLayout::window().pos(Rl((20.0, 400.0))).size((Rl(400.0), Rl(200.0))).pack(),
            UiColor::new(vec![
                (UiBase::id(), Color::srgba(0.1, 0.1, 0.15, 0.9)),
            ]),
            Name::new("Lunex Tutorial Dialog"),
            LunexTutorialUI,
            LunexMigrationMarker,
        )).with_children(|dialog| {
            // Tutorial content text area
            dialog.spawn((
                Text2d::new("Tutorial Content"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                UiTextSize::from(Rh(60.0)),
                UiLayout::window().pos(Rl((20.0, 20.0))).size((Rl(360.0), Rl(120.0))).pack(),
                TextColor(Color::WHITE),
                LunexTutorialDialog,
            ));
            
            // Button container at bottom
            dialog.spawn((
                UiLayout::window().pos(Rl((20.0, 150.0))).size((Rl(360.0), Rl(40.0))).pack(),
                Name::new("Tutorial Button Container"),
            )).with_children(|buttons| {
                // Skip button
                buttons.spawn((
                    UiLayout::window().pos(Rl((0.0, 0.0))).size((Rl(80.0), Rl(30.0))).pack(),
                    UiColor::new(vec![
                        (UiBase::id(), Color::srgb(0.7, 0.3, 0.3)),
                        (UiHover::id(), Color::srgb(0.8, 0.4, 0.4)),
                    ]),
                    UiHover::new().forward_speed(20.0).backward_speed(4.0),
                    LunexTutorialButton { action: LunexTutorialAction::Skip },
                    Name::new("Tutorial Skip Button"),
                )).with_children(|button| {
                    button.spawn((
                        Text2d::new("Skip"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        UiTextSize::from(Rh(50.0)),
                        UiLayout::window().pack(),
                        TextColor(Color::WHITE),
                    ));
                });
                
                // Next button
                buttons.spawn((
                    UiLayout::window().pos(Rl((280.0, 0.0))).size((Rl(80.0), Rl(30.0))).pack(),
                    UiColor::new(vec![
                        (UiBase::id(), Color::srgb(0.3, 0.6, 0.3)),
                        (UiHover::id(), Color::srgb(0.4, 0.7, 0.4)),
                    ]),
                    UiHover::new().forward_speed(20.0).backward_speed(4.0),
                    LunexTutorialButton { action: LunexTutorialAction::Next },
                    Name::new("Tutorial Next Button"),
                )).with_children(|button| {
                    button.spawn((
                        Text2d::new("Next"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        UiTextSize::from(Rh(50.0)),
                        UiLayout::window().pack(),
                        TextColor(Color::WHITE),
                    ));
                });
            });
        });
    });
}

// System to handle Lunex tutorial button clicks
pub fn handle_lunex_tutorial_buttons(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut tutorial_events: EventWriter<crate::tutorial::resources::TutorialEvent>,
    tutorial_state: Res<crate::tutorial::resources::TutorialState>,
) {
    if !tutorial_state.is_active {
        return;
    }
    
    // Temporary keyboard controls for testing:
    // Enter = Next, Escape = Skip
    if keyboard.just_pressed(KeyCode::Enter) {
        info!("Lunex tutorial: Next step");
        tutorial_events.write(crate::tutorial::resources::TutorialEvent {
            action: crate::tutorial::resources::TutorialAction::NextStep,
        });
    } else if keyboard.just_pressed(KeyCode::Escape) {
        info!("Lunex tutorial: Skip");
        tutorial_events.write(crate::tutorial::resources::TutorialEvent {
            action: crate::tutorial::resources::TutorialAction::Skip,
        });
    }
}

// System to update tutorial content
pub fn update_lunex_tutorial_content(
    tutorial_state: Res<crate::tutorial::resources::TutorialState>,
    mut text_query: Query<&mut Text2d, With<LunexTutorialDialog>>,
    mut commands: Commands,
    tutorial_ui_query: Query<Entity, With<LunexTutorialUI>>,
) {
    // Clean up tutorial UI if tutorial is not active or UI shouldn't show
    if !tutorial_state.is_active || !tutorial_state.show_ui {
        for entity in tutorial_ui_query.iter() {
            commands.entity(entity).despawn();
        }
        return;
    }
    
    // Update tutorial content text
    if let Ok(mut text) = text_query.single_mut() {
        let content = format!("{}\n\n{}", 
            tutorial_state.current_step.title(),
            tutorial_state.current_step.description()
        );
        **text = content;
    }
}

// Journal System Migration Components
#[derive(Component)]
pub struct LunexJournalUI;

#[derive(Component)]
pub struct LunexJournalTab {
    pub tab: crate::journal::components::JournalTab,
}

#[derive(Component)]
pub struct LunexJournalContent;

#[derive(Component)]
pub struct LunexJournalClose;

// System to setup Lunex journal on state entry
pub fn setup_lunex_journal(mut commands: Commands) {
    info!("Setting up Lunex Journal UI");
    
    // Create Lunex UI root for journal
    commands.spawn((
        UiLayoutRoot::new_2d(),
        UiFetchFromCamera::<0>,
        Name::new("Lunex Journal Root"),
    )).with_children(|ui| {
        // Main journal container - full screen
        ui.spawn((
            UiLayout::window().pos(Rl((50.0, 50.0))).size((Rl(700.0), Rl(500.0))).pack(),
            UiColor::new(vec![
                (UiBase::id(), Color::srgb(0.95, 0.92, 0.88)),
            ]),
            Name::new("Lunex Journal Container"),
            LunexJournalUI,
            LunexMigrationMarker,
        )).with_children(|journal| {
            // Title bar
            journal.spawn((
                Text2d::new("Bird Journal"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                UiTextSize::from(Rh(60.0)),
                UiLayout::window().pos(Rl((20.0, 20.0))).pack(),
                TextColor(Color::srgb(0.3, 0.2, 0.1)),
            ));
            
            // Close button
            journal.spawn((
                UiLayout::window().pos(Rl((650.0, 20.0))).size((Rl(30.0), Rl(30.0))).pack(),
                UiColor::new(vec![
                    (UiBase::id(), Color::srgb(0.8, 0.3, 0.3)),
                    (UiHover::id(), Color::srgb(0.9, 0.4, 0.4)),
                ]),
                UiHover::new().forward_speed(20.0).backward_speed(4.0),
                LunexJournalClose,
                Name::new("Journal Close Button"),
            )).with_children(|button| {
                button.spawn((
                    Text2d::new("×"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    UiTextSize::from(Rh(70.0)),
                    UiLayout::window().pack(),
                    TextColor(Color::WHITE),
                ));
            });
            
            // Tab buttons
            let tabs = [
                ("Species", crate::journal::components::JournalTab::Species),
                ("Photos", crate::journal::components::JournalTab::Photos),
                ("Conservation", crate::journal::components::JournalTab::Conservation),
                ("Research", crate::journal::components::JournalTab::Research),
            ];
            
            for (i, (label, tab)) in tabs.iter().enumerate() {
                journal.spawn((
                    UiLayout::window().pos(Rl((20.0 + i as f32 * 120.0, 60.0))).size((Rl(100.0), Rl(30.0))).pack(),
                    UiColor::new(vec![
                        (UiBase::id(), Color::srgb(0.7, 0.6, 0.5)),
                        (UiHover::id(), Color::srgb(0.8, 0.7, 0.6)),
                    ]),
                    UiHover::new().forward_speed(20.0).backward_speed(4.0),
                    LunexJournalTab { tab: *tab },
                    Name::new(format!("Journal Tab: {}", label)),
                )).with_children(|button| {
                    button.spawn((
                        Text2d::new(*label),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        UiTextSize::from(Rh(50.0)),
                        UiLayout::window().pack(),
                        TextColor(Color::WHITE),
                    ));
                });
            }
            
            // Content area
            journal.spawn((
                UiLayout::window().pos(Rl((20.0, 100.0))).size((Rl(660.0), Rl(380.0))).pack(),
                UiColor::new(vec![
                    (UiBase::id(), Color::srgb(0.97, 0.94, 0.90)),
                ]),
                LunexJournalContent,
                Name::new("Journal Content Area"),
            )).with_children(|content| {
                // Default content text
                content.spawn((
                    Text2d::new("Welcome to your Bird Journal!\n\nSelect a tab above to explore different sections.\n\nPress Tab or Escape to close."),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    UiTextSize::from(Rh(50.0)),
                    UiLayout::window().pos(Rl((20.0, 20.0))).pack(),
                    TextColor(Color::srgb(0.4, 0.3, 0.2)),
                ));
            });
        });
    });
}

// System to cleanup Lunex journal on state exit
pub fn cleanup_lunex_journal(
    mut commands: Commands,
    journal_query: Query<Entity, With<LunexJournalUI>>,
    root_query: Query<Entity, (With<UiLayoutRoot>, Without<LunexJournalUI>)>,
) {
    // Clean up all journal entities
    for entity in journal_query.iter() {
        commands.entity(entity).despawn();
    }
    
    // Clean up journal root if it exists - simplified approach
    // Note: More targeted cleanup would require additional component markers
}

// System to handle Lunex journal navigation
pub fn handle_lunex_journal_navigation(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut app_state: ResMut<NextState<crate::AppState>>,
    mut journal_state: ResMut<crate::journal::resources::JournalState>,
) {
    // Keyboard navigation: Tab/Escape to close, 1-4 for tab switching
    if keyboard.just_pressed(KeyCode::Tab) || keyboard.just_pressed(KeyCode::Escape) {
        info!("Lunex journal: Closing");
        app_state.set(crate::AppState::Playing);
    } else if keyboard.just_pressed(KeyCode::Digit1) {
        info!("Lunex journal: Species tab");
        journal_state.current_tab = crate::journal::components::JournalTab::Species;
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        info!("Lunex journal: Photos tab");
        journal_state.current_tab = crate::journal::components::JournalTab::Photos;
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        info!("Lunex journal: Conservation tab");
        journal_state.current_tab = crate::journal::components::JournalTab::Conservation;
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        info!("Lunex journal: Research tab");
        journal_state.current_tab = crate::journal::components::JournalTab::Research;
    }
}

// System to update journal content based on current tab
pub fn update_lunex_journal_content(
    journal_state: Res<crate::journal::resources::JournalState>,
    mut text_query: Query<&mut Text2d, With<LunexJournalContent>>,
    discovered_species: Res<crate::journal::resources::DiscoveredSpecies>,
) {
    if journal_state.is_changed() {
        if let Ok(mut text) = text_query.single_mut() {
            let content = match journal_state.current_tab {
                crate::journal::components::JournalTab::Species => {
                    format!("Species Discovered: {}\n\nUse the original journal (press Tab twice) to see full species details.\n\nThis Lunex version shows basic information:\n\n- Total species found: {}\n- Current migration season active\n- Weather affects bird activity",
                        discovered_species.0.len(),
                        discovered_species.0.len())
                }
                crate::journal::components::JournalTab::Photos => {
                    "Photo Gallery\n\nYour captured bird photographs would be displayed here.\n\nFeatures:\n- Sort by date/species\n- View photo metadata\n- Share favorite shots\n- Photo quality ratings".to_string()
                }
                crate::journal::components::JournalTab::Conservation => {
                    "Conservation Status\n\nTrack conservation efforts and bird population data.\n\nInformation includes:\n- Species protection status\n- Local population trends\n- Conservation success stories\n- How you can help".to_string()
                }
                crate::journal::components::JournalTab::Research => {
                    "Research Missions\n\nParticipate in citizen science projects.\n\nActive missions:\n- Migration pattern tracking\n- Feeding behavior studies\n- Population surveys\n- Climate impact research".to_string()
                }
                _ => "Select a tab to view content".to_string(),
            };
            **text = content;
        }
    }
}

// Catalog System Migration Components
#[derive(Component)]
pub struct LunexCatalogUI;

#[derive(Component)]
pub struct LunexCatalogContent;

#[derive(Component)]
pub struct LunexCatalogVisible(pub bool);

// System to setup Lunex catalog UI
pub fn setup_lunex_catalog_ui(
    mut commands: Commands,
    catalog_ui_query: Query<Entity, With<LunexCatalogUI>>,
    catalog_state: Res<crate::catalog::resources::CatalogState>,
    lunex_root_query: Query<Entity, With<UiLayoutRoot>>,
) {
    // Only create catalog UI if it's supposed to be open and we don't already have Lunex catalog
    if !catalog_state.is_open || !catalog_ui_query.is_empty() {
        return;
    }
    
    // Find or create Lunex root
    let lunex_root = if let Ok(root_entity) = lunex_root_query.single() {
        root_entity
    } else {
        // Create root if it doesn't exist
        commands.spawn((
            UiLayoutRoot::new_2d(),
            UiFetchFromCamera::<0>,
            Name::new("Lunex Catalog Root"),
        )).id()
    };
    
    // Add catalog UI to the root
    commands.entity(lunex_root).with_children(|ui| {
        // Catalog panel - positioned on the right side
        ui.spawn((
            UiLayout::window().pos(Rl((550.0, 50.0))).size((Rl(350.0), Rl(450.0))).pack(),
            UiColor::new(vec![
                (UiBase::id(), Color::srgba(0.92, 0.89, 0.85, 0.95)),
            ]),
            Name::new("Lunex Catalog Panel"),
            LunexCatalogUI,
            LunexMigrationMarker,
        )).with_children(|catalog| {
            // Title
            catalog.spawn((
                Text2d::new("Shop & Catalog"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                UiTextSize::from(Rh(60.0)),
                UiLayout::window().pos(Rl((20.0, 20.0))).pack(),
                TextColor(Color::srgb(0.3, 0.2, 0.1)),
            ));
            
            // Close info
            catalog.spawn((
                Text2d::new("Press C to close"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                UiTextSize::from(Rh(40.0)),
                UiLayout::window().pos(Rl((250.0, 25.0))).pack(),
                TextColor(Color::srgb(0.5, 0.4, 0.3)),
            ));
            
            // Content area
            catalog.spawn((
                UiLayout::window().pos(Rl((20.0, 60.0))).size((Rl(310.0), Rl(370.0))).pack(),
                UiColor::new(vec![
                    (UiBase::id(), Color::srgb(0.98, 0.95, 0.91)),
                ]),
                LunexCatalogContent,
                Name::new("Catalog Content Area"),
            )).with_children(|content| {
                // Sample items display
                let items = [
                    ("Cardboard Box", "10 coins", "Simple box that birds love"),
                    ("Red Cushion", "25 coins", "Soft cushion for resting"),
                    ("Wooden Perch", "50 coins", "Natural perch for roosting"),
                    ("Bird Bath", "80 coins", "Water source for drinking"),
                    ("Premium Seeds", "75 coins", "High-quality seed mix"),
                ];
                
                for (i, (name, price, desc)) in items.iter().enumerate() {
                    // Item card
                    content.spawn((
                        UiLayout::window().pos(Rl((10.0, 10.0 + i as f32 * 70.0))).size((Rl(290.0), Rl(60.0))).pack(),
                        UiColor::new(vec![
                            (UiBase::id(), Color::srgb(0.9, 0.87, 0.83)),
                            (UiHover::id(), Color::srgb(0.92, 0.89, 0.85)),
                        ]),
                        UiHover::new().forward_speed(20.0).backward_speed(4.0),
                        Name::new(format!("Catalog Item: {}", name)),
                    )).with_children(|item| {
                        // Item name
                        item.spawn((
                            Text2d::new(*name),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            UiTextSize::from(Rh(45.0)),
                            UiLayout::window().pos(Rl((10.0, 5.0))).pack(),
                            TextColor(Color::srgb(0.3, 0.2, 0.1)),
                        ));
                        
                        // Item price
                        item.spawn((
                            Text2d::new(*price),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            UiTextSize::from(Rh(40.0)),
                            UiLayout::window().pos(Rl((200.0, 5.0))).pack(),
                            TextColor(Color::srgb(0.2, 0.6, 0.2)),
                        ));
                        
                        // Item description
                        item.spawn((
                            Text2d::new(*desc),
                            TextFont {
                                font_size: 10.0,
                                ..default()
                            },
                            UiTextSize::from(Rh(35.0)),
                            UiLayout::window().pos(Rl((10.0, 25.0))).pack(),
                            TextColor(Color::srgb(0.4, 0.3, 0.2)),
                        ));
                        
                        // Buy button placeholder
                        item.spawn((
                            Text2d::new("Buy"),
                            TextFont {
                                font_size: 10.0,
                                ..default()
                            },
                            UiTextSize::from(Rh(35.0)),
                            UiLayout::window().pos(Rl((250.0, 40.0))).pack(),
                            TextColor(Color::srgb(0.2, 0.4, 0.7)),
                        ));
                    });
                }
                
                // Instructions
                content.spawn((
                    Text2d::new("Use original catalog (C key twice) for full functionality.\n\nThis Lunex version shows the shop interface."),
                    TextFont {
                        font_size: 11.0,
                        ..default()
                    },
                    UiTextSize::from(Rh(35.0)),
                    UiLayout::window().pos(Rl((10.0, 360.0))).pack(),
                    TextColor(Color::srgb(0.6, 0.5, 0.4)),
                ));
            });
        });
    });
}

// System to handle Lunex catalog input
pub fn handle_lunex_catalog_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut catalog_state: ResMut<crate::catalog::resources::CatalogState>,
) {
    // Toggle catalog visibility with C key
    if keyboard.just_pressed(KeyCode::KeyC) {
        catalog_state.is_open = !catalog_state.is_open;
        info!("Lunex catalog visibility: {}", catalog_state.is_open);
    }
}

// System to update catalog content and visibility
pub fn update_lunex_catalog_content(
    mut commands: Commands,
    catalog_state: Res<crate::catalog::resources::CatalogState>,
    catalog_ui_query: Query<Entity, With<LunexCatalogUI>>,
) {
    // Clean up catalog UI if it should not be visible
    if !catalog_state.is_open {
        for entity in catalog_ui_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}