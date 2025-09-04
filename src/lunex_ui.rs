use bevy::prelude::*;
use bevy_lunex::prelude::*;
use std::time::Instant;

// Lunex UI Migration Module
// This module provides a gradual migration path from Bevy UI to Lunex UI
// Starting with simple components and expanding to complex layouts

pub struct LunexUiPlugin;

impl Plugin for LunexUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UiLunexPlugins)
            .add_systems(Startup, setup_lunex_system)
            .add_systems(OnEnter(crate::AppState::MainMenu), setup_lunex_main_menu)
            .add_systems(OnExit(crate::AppState::MainMenu), cleanup_lunex_main_menu)
            .add_systems(Update, handle_lunex_main_menu_clicks.run_if(in_state(crate::AppState::MainMenu)))
            .add_systems(Update, (
                create_lunex_settings_buttons,
                handle_lunex_button_clicks,
            ).run_if(in_state(crate::AppState::Settings)))
            .add_systems(Update, (
                setup_lunex_tutorial_ui,
                handle_lunex_tutorial_buttons,
                update_lunex_tutorial_content,
            ).run_if(in_state(crate::AppState::Playing)))
            .add_systems(OnEnter(crate::AppState::Journal), setup_lunex_journal_simple)
            .add_systems(OnExit(crate::AppState::Journal), cleanup_lunex_journal)
            .add_systems(Update, (
                handle_lunex_journal_navigation_simple,
                update_bevy_journal_content,
            ).run_if(in_state(crate::AppState::Journal)))
            // Debug system disabled - use F1 key diagnostic instead
            // .add_systems(Update, debug_journal_entities.run_if(in_state(crate::AppState::Journal)))
            .add_systems(OnEnter(crate::AppState::Catalog), setup_lunex_catalog)
            .add_systems(OnExit(crate::AppState::Catalog), cleanup_lunex_catalog)
            .add_systems(Update, (
                handle_lunex_catalog_navigation,
                update_catalog_currency_display,
                debug_lunex_catalog_state,
                catalog_health_monitor,
            ).run_if(in_state(crate::AppState::Catalog)));
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
pub struct BevyJournalUI; // Separate marker for Bevy UI fallback

#[derive(Component)]
pub struct JournalContentText; // Marker for journal content text that updates with tabs

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
                (UiBase::id(), Color::srgb(0.0, 0.0, 1.0)), // Bright blue for testing
            ]),
            Name::new("Lunex Journal Container"),
            LunexJournalUI,
            LunexMigrationMarker,
        )).with_children(|journal| {
            // Title
            journal.spawn((
                Text::new("Bird Journal"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
            ));
            
            // Placeholder content
            journal.spawn((
                Text::new("Journal content goes here - Press Escape or J to close"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
            ));
            
            // Close button (simplified)
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

// System to cleanup journal on state exit (both Lunex and Bevy UI)
pub fn cleanup_lunex_journal(
    mut commands: Commands,
    lunex_journal_query: Query<Entity, With<LunexJournalUI>>,
    bevy_journal_query: Query<Entity, With<BevyJournalUI>>,
    // root_query: Query<Entity, (With<UiLayoutRoot>, Without<LunexJournalUI>)>,
) {
    info!("🔴 JOURNAL: cleanup_lunex_journal called");
    
    let mut cleanup_count = 0;
    
    // Clean up Lunex journal entities
    for entity in lunex_journal_query.iter() {
        info!("🔴 JOURNAL: Despawning Lunex journal entity: {:?}", entity);
        commands.entity(entity).despawn();
        cleanup_count += 1;
    }
    
    // Clean up Bevy UI journal entities  
    for entity in bevy_journal_query.iter() {
        info!("🔴 JOURNAL: Despawning Bevy journal entity: {:?}", entity);
        commands.entity(entity).despawn();
        cleanup_count += 1;
    }
    
    info!("🔴 JOURNAL: Cleaned up {} journal entities total", cleanup_count);
    
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
pub struct BevyCatalogUI; // Bevy UI fallback for catalog

#[derive(Component)]
pub struct CatalogItemsGrid; // Marker for catalog items grid area

#[derive(Component)]
pub struct CatalogCurrencyText; // Marker for currency text in catalog

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
    if !catalog_state.is_open {
        return;
    }
    
    if !catalog_ui_query.is_empty() {
        return; // Already exists
    }
    
    info!("Setting up Lunex catalog UI");
    
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
                (UiBase::id(), Color::srgb(0.0, 1.0, 0.0)), // Bright green for testing
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
    if !catalog_state.is_open && !catalog_ui_query.is_empty() {
        info!("Cleaning up Lunex catalog UI");
        for entity in catalog_ui_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

// Debug system to track catalog state and UI presence
pub fn debug_lunex_catalog_state(
    catalog_state: Res<crate::catalog::resources::CatalogState>,
    catalog_ui_query: Query<Entity, With<LunexCatalogUI>>,
    lunex_root_query: Query<Entity, With<UiLayoutRoot>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyC) {
        info!("=== Catalog Debug ===");
        info!("Catalog state is_open: {}", catalog_state.is_open);
        info!("Catalog UI entities: {}", catalog_ui_query.iter().len());
        info!("Lunex root entities: {}", lunex_root_query.iter().len());
    }
}

// Test system to create a simple always-visible Lunex UI
pub fn setup_test_lunex_catalog(mut commands: Commands) {
    info!("Setting up test Lunex catalog");
    
    // Create simple test UI
    commands.spawn((
        UiLayoutRoot::new_2d(),
        UiFetchFromCamera::<0>,
        Name::new("Test Lunex Root"),
    )).with_children(|ui| {
        ui.spawn((
            UiLayout::window().pos(Rl((400.0, 100.0))).size((Rl(200.0), Rl(100.0))).pack(),
            UiColor::new(vec![
                (UiBase::id(), Color::srgb(1.0, 0.0, 0.0)), // Bright red for visibility
            ]),
            Name::new("Test Lunex Window"),
        )).with_children(|window| {
            window.spawn((
                UiLayout::solid().size((Rl(180.0), Rl(30.0))).pack(),
                Text::new("TEST LUNEX UI"),
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
            ));
        });
    });
}

// System to setup catalog using proper original design (restored from working version)
pub fn setup_lunex_catalog(
    mut commands: Commands,
    catalog_state: Res<crate::catalog::resources::CatalogState>,
    time: Res<Time>,
) {
    let setup_start = Instant::now();
    let timestamp = time.elapsed_secs();
    
    info!("🟢 CATALOG SETUP: === STARTING CATALOG SETUP AT {:.3}s ===", timestamp);
    info!("🟢 CATALOG SETUP: Restoring original catalog design with Bevy UI");
    info!("🟢 CATALOG SETUP: Catalog state is_open: {}", catalog_state.is_open);
    
    // Create full-screen modal catalog (original design restored)
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            left: Val::Percent(0.0),
            top: Val::Percent(0.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)), // Dark overlay
        Name::new("Catalog Modal Background"),
        BevyCatalogUI,
    )).with_children(|modal| {
        // Main catalog window (centered, 85% screen size)
        modal.spawn((
            Node {
                width: Val::Percent(85.0),
                height: Val::Percent(85.0),
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.95, 0.92, 0.88)), // Original aged paper color
            BorderColor(Color::srgb(0.6, 0.4, 0.2)), // Original brown border
            Name::new("Catalog Window"),
        )).with_children(|catalog| {
            // Title bar (original design)
            catalog.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(60.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::horizontal(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.7, 0.5, 0.3)), // Original brown header
            )).with_children(|title| {
                // Title text
                title.spawn((
                    Text::new("Bird Garden Catalog"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
                
                // Currency display  
                title.spawn((
                    Text::new("Credits: Loading..."),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.9, 0.1)), // Gold color
                    CatalogCurrencyText, // Component for currency updates
                ));
            });
            
            // Content area (original functional design restored)
            catalog.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
            )).with_children(|content| {
                // Category sidebar (original design)
                content.spawn((
                    Node {
                        width: Val::Percent(20.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(10.0)),
                        row_gap: Val::Px(5.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.85, 0.8, 0.75)),
                )).with_children(|sidebar| {
                    // Category title
                    sidebar.spawn((
                        Text::new("Categories"),
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
                    
                    // Category buttons (restored functional categories)
                    let categories = [
                        ("Comfort", "Comfort Items"),
                        ("Food", "Food & Feeders"),
                        ("Water", "Water Features"),
                        ("Decorative", "Decorative Items"),
                        ("Special", "Special Items"),
                    ];
                    
                    for (category, label) in categories {
                        sidebar.spawn((
                            Button,
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::vertical(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.75, 0.7, 0.65)),
                        )).with_children(|button| {
                            button.spawn((
                                Text::new(label),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.2, 0.1, 0.0)),
                            ));
                        });
                    }
                });
                
                // Items grid area (restored original with placeholder items)
                content.spawn((
                    Node {
                        width: Val::Percent(80.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(15.0)),
                        overflow: Overflow::scroll_y(),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.92, 0.88, 0.84)),
                    CatalogItemsGrid,
                )).with_children(|grid| {
                    // Create sample item cards (showing the structure)
                    let sample_items = [
                        ("Cardboard Box", 10, "A simple box that birds love to explore"),
                        ("Red Cushion", 25, "Soft red cushion for birds to rest on"),
                        ("Premium Seed Mix", 75, "High-quality seed mix for rare species"),
                        ("Fountain Birdbath", 200, "Elegant fountain that attracts more birds"),
                        ("Garden Gnome", 60, "Decorative gnome that some birds find intriguing"),
                        ("Mirror Toy", 85, "Reflective toy that fascinates certain species"),
                    ];
                    
                    // Create item rows
                    for chunk in sample_items.chunks(3) {
                        grid.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(200.0),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceEvenly,
                                align_items: AlignItems::Start,
                                margin: UiRect::vertical(Val::Px(10.0)),
                                ..default()
                            },
                        )).with_children(|row| {
                            for (name, price, description) in chunk {
                                // Create item card (original structure restored)
                                row.spawn((
                                    Node {
                                        width: Val::Px(180.0),
                                        height: Val::Px(180.0),
                                        flex_direction: FlexDirection::Column,
                                        border: UiRect::all(Val::Px(2.0)),
                                        padding: UiRect::all(Val::Px(8.0)),
                                        margin: UiRect::all(Val::Px(5.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.98, 0.96, 0.94)),
                                    BorderColor(Color::srgb(0.6, 0.4, 0.2)),
                                )).with_children(|card| {
                                    // Item image placeholder (sprite area)
                                    card.spawn((
                                        Node {
                                            width: Val::Percent(100.0),
                                            height: Val::Px(80.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            margin: UiRect::bottom(Val::Px(5.0)),
                                            border: UiRect::all(Val::Px(1.0)),
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgb(0.9, 0.9, 0.85)),
                                        BorderColor(Color::srgb(0.7, 0.7, 0.6)),
                                    )).with_children(|img_container| {
                                        // Placeholder sprite (64x64)
                                        img_container.spawn((
                                            Node {
                                                width: Val::Px(64.0),
                                                height: Val::Px(64.0),
                                                ..default()
                                            },
                                            BackgroundColor(Color::srgb(0.8, 0.8, 0.7)),
                                        )).with_children(|placeholder| {
                                            placeholder.spawn((
                                                Text::new("🖼"),
                                                TextFont { font_size: 32.0, ..default() },
                                                TextColor(Color::srgb(0.5, 0.5, 0.4)),
                                            ));
                                        });
                                    });
                                    
                                    // Item name
                                    card.spawn((
                                        Text::new(*name),
                                        TextFont {
                                            font_size: 12.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.2, 0.1, 0.0)),
                                        Node {
                                            margin: UiRect::bottom(Val::Px(3.0)),
                                            ..default()
                                        },
                                    ));
                                    
                                    // Price and utility info
                                    card.spawn((
                                        Node {
                                            width: Val::Percent(100.0),
                                            flex_direction: FlexDirection::Row,
                                            justify_content: JustifyContent::SpaceBetween,
                                            margin: UiRect::bottom(Val::Px(5.0)),
                                            ..default()
                                        },
                                    )).with_children(|info| {
                                        info.spawn((
                                            Text::new(format!("${}", price)),
                                            TextFont {
                                                font_size: 10.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.1, 0.5, 0.1)), // Green price
                                        ));
                                        
                                        info.spawn((
                                            Text::new("Utility: ⭐⭐⭐"),
                                            TextFont {
                                                font_size: 8.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.6, 0.4, 0.0)), // Orange utility
                                        ));
                                    });
                                    
                                    // Purchase button
                                    card.spawn((
                                        Button,
                                        Node {
                                            width: Val::Percent(100.0),
                                            height: Val::Px(25.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgb(0.2, 0.6, 0.2)),
                                    )).with_children(|buy_btn| {
                                        buy_btn.spawn((
                                            Text::new("Purchase"),
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
                    }
                });
            });
        });
    });
    
    let setup_duration = setup_start.elapsed();
    info!("🟢 CATALOG SETUP: === CATALOG SETUP COMPLETED SUCCESSFULLY in {:?} ===", setup_duration);
}

// System to cleanup catalog UI on state exit (both Lunex and Bevy UI)
pub fn cleanup_lunex_catalog(
    mut commands: Commands,
    lunex_catalog_query: Query<Entity, With<LunexCatalogUI>>,
    bevy_catalog_query: Query<Entity, With<BevyCatalogUI>>,
    time: Res<Time>,
) {
    let timestamp = time.elapsed_secs();
    let lunex_count = lunex_catalog_query.iter().count();
    let bevy_count = bevy_catalog_query.iter().count();
    
    info!("🔴 CATALOG CLEANUP: === CLEANUP TRIGGERED AT {:.3}s ===", timestamp);
    info!("🔴 CATALOG CLEANUP: Cleaning up {} Lunex + {} Bevy catalog entities", lunex_count, bevy_count);
    
    // Clean up Lunex entities
    for entity in lunex_catalog_query.iter() {
        info!("🔴 CATALOG CLEANUP: Despawning Lunex entity: {:?}", entity);
        commands.entity(entity).despawn();
    }
    
    // Clean up Bevy entities  
    for entity in bevy_catalog_query.iter() {
        info!("🔴 CATALOG CLEANUP: Despawning Bevy entity: {:?}", entity);
        commands.entity(entity).despawn();
    }
    
    info!("🔴 CATALOG CLEANUP: Catalog cleanup completed");
}

// System to handle catalog navigation with diagnostics
pub fn handle_lunex_catalog_navigation(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut app_state: ResMut<NextState<crate::AppState>>,
    mut catalog_state: ResMut<crate::catalog::resources::CatalogState>,
) {
    if keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::KeyC) {
        info!("🟡 CATALOG NAV: Close key pressed, transitioning to Playing state");
        catalog_state.is_open = false;
        app_state.set(crate::AppState::Playing);
    }
}

// System to update catalog currency display
pub fn update_catalog_currency_display(
    currency: Res<crate::photo_mode::resources::CurrencyResource>,
    mut text_query: Query<&mut Text, With<CatalogCurrencyText>>,
) {
    if currency.is_changed() {
        for mut text in text_query.iter_mut() {
            **text = format!("Credits: {}", currency.0); // Tuple struct access
        }
    }
}

// System to monitor catalog health every 1000ms as requested
pub fn catalog_health_monitor(
    lunex_catalog_query: Query<Entity, With<LunexCatalogUI>>,
    bevy_catalog_query: Query<Entity, With<BevyCatalogUI>>,
    time: Res<Time>,
    mut last_check: Local<f32>,
) {
    let current_time = time.elapsed_secs();
    
    // Check every 1000ms (1 second) as requested
    if current_time - *last_check >= 1.0 {
        *last_check = current_time;
        
        let lunex_count = lunex_catalog_query.iter().count();
        let bevy_count = bevy_catalog_query.iter().count();
        let total_count = lunex_count + bevy_count;
        
        if total_count == 0 {
            error!("🚨 CATALOG HEALTH: NO catalog UI entities found at {:.3}s! Something despawned them!", current_time);
        } else {
            info!("✅ CATALOG HEALTH: {} total catalog entities healthy at {:.3}s (Lunex: {}, Bevy: {})", 
                  total_count, current_time, lunex_count, bevy_count);
        }
    }
}

// System to setup journal using Bevy UI (stable working implementation)
pub fn setup_lunex_journal_simple(mut commands: Commands) {
    info!("🔵 JOURNAL SETUP: === STARTING JOURNAL SETUP ===");
    info!("🔵 JOURNAL SETUP: Creating Bevy UI journal (stable implementation)");
    
    // Create ONLY the Bevy UI journal (remove conflicting Lunex UI)
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(100.0),
            top: Val::Px(50.0),
            width: Val::Px(700.0),
            height: Val::Px(500.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.95, 0.92, 0.88, 0.95)), // Aged paper color
        BorderColor(Color::srgb(0.7, 0.6, 0.5)), // Brown border
        Name::new("Bevy UI Journal"),
        BevyJournalUI,
    )).with_children(|journal| {
        // Title
        journal.spawn((
            Text::new("Bird Journal"),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::srgb(0.3, 0.2, 0.1)),
            Node {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
        ));
        
        // Subtitle
        journal.spawn((
            Text::new("Field Notes & Species Observations"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(0.4, 0.3, 0.2)),
            Node {
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            },
        ));
        
        // Content area (with component marker for updates)
        journal.spawn((
            Text::new("Welcome to your field journal!\n\nHere you can:\n• View discovered bird species\n• Read educational information\n• Track your observations\n• Access research missions\n\nControls:\n• Press J or Escape to close\n• Use number keys 1-4 to switch tabs\n\nDiagnostics:\n• Press F1 for journal diagnostics\n• Press F3 for comprehensive UI health check"),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(0.2, 0.1, 0.0)),
            Node {
                flex_grow: 1.0,
                ..default()
            },
            JournalContentText, // Marker component for content updates
        ));
        
        // Footer instructions
        journal.spawn((
            Text::new("Press J or Escape to close • Press 1-4 for tabs • F1/F3 for diagnostics"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgb(0.5, 0.4, 0.3)),
            Node {
                margin: UiRect::top(Val::Px(20.0)),
                align_self: AlignSelf::Center,
                ..default()
            },
        ));
    });
    
    info!("🔵 JOURNAL SETUP: Created professional Bevy UI journal interface");
    info!("🔵 JOURNAL SETUP: === JOURNAL SETUP COMPLETED SUCCESSFULLY ===");
}

// Debug system to track journal entities
pub fn debug_journal_entities(
    journal_query: Query<Entity, With<LunexJournalUI>>,
    root_query: Query<Entity, With<UiLayoutRoot>>,
    text_query: Query<Entity, With<Text2d>>,
) {
    let journal_count = journal_query.iter().count();
    let root_count = root_query.iter().count();
    let text_count = text_query.iter().count();
    
    info!("🔵 JOURNAL DEBUG: Journal entities: {}, Root entities: {}, Text entities: {}", 
          journal_count, root_count, text_count);
}

// System to handle journal navigation with tab support
pub fn handle_lunex_journal_navigation_simple(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut app_state: ResMut<NextState<crate::AppState>>,
    mut journal_state: ResMut<crate::journal::resources::JournalState>,
) {
    if keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::KeyJ) {
        info!("🔴 JOURNAL: Keyboard close detected, transitioning to Playing state");
        app_state.set(crate::AppState::Playing);
    } else if keyboard.just_pressed(KeyCode::Digit1) {
        info!("🔵 JOURNAL: Species tab selected");
        journal_state.current_tab = crate::journal::components::JournalTab::Species;
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        info!("🔵 JOURNAL: Photos tab selected");
        journal_state.current_tab = crate::journal::components::JournalTab::Photos;
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        info!("🔵 JOURNAL: Conservation tab selected");
        journal_state.current_tab = crate::journal::components::JournalTab::Conservation;
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        info!("🔵 JOURNAL: Research tab selected");
        journal_state.current_tab = crate::journal::components::JournalTab::Research;
    }
}

// System to update Bevy journal content based on current tab
pub fn update_bevy_journal_content(
    journal_state: Res<crate::journal::resources::JournalState>,
    mut text_query: Query<&mut Text, With<JournalContentText>>,
    discovered_species: Res<crate::journal::resources::DiscoveredSpecies>,
) {
    if journal_state.is_changed() {
        if let Ok(mut text) = text_query.single_mut() {
            let content = match journal_state.current_tab {
                crate::journal::components::JournalTab::Species => {
                    format!("Species Tab - Discovered Birds\n\n{} species discovered so far.\n\nSpecies found:\n{}\n\nControls:\n• Press 2 for Photos\n• Press 3 for Conservation\n• Press 4 for Research\n• Press J or Escape to close",
                        discovered_species.0.len(),
                        if discovered_species.0.is_empty() {
                            "No species discovered yet. Go outside and observe some birds!".to_string()
                        } else {
                            discovered_species.0.iter()
                                .map(|species| format!("• {:?}", species))
                                .collect::<Vec<_>>()
                                .join("\n")
                        })
                }
                crate::journal::components::JournalTab::Photos => {
                    "Photo Gallery\n\nYour captured bird photographs will be displayed here.\n\nFeatures:\n• Sort by date taken\n• Sort by bird species\n• View photo metadata\n• Export favorite shots\n• Photo quality ratings\n\nTake photos by pressing P to enter photo mode, then Space to capture.\n\nControls:\n• Press 1 for Species\n• Press 3 for Conservation\n• Press 4 for Research".to_string()
                }
                crate::journal::components::JournalTab::Conservation => {
                    "Conservation Status\n\nTrack conservation efforts and bird population data.\n\nInformation includes:\n• Species protection status\n• Local population trends\n• Habitat preservation efforts\n• Conservation success stories\n• How you can help protect birds\n\nMany bird species are facing challenges from habitat loss and climate change.\n\nControls:\n• Press 1 for Species\n• Press 2 for Photos\n• Press 4 for Research".to_string()
                }
                crate::journal::components::JournalTab::Research => {
                    "Research Missions\n\nParticipate in citizen science projects and help real bird research.\n\nActive missions:\n• Migration pattern tracking\n• Feeding behavior studies\n• Population surveys for local species\n• Climate impact research\n• Breeding success monitoring\n\nYour observations contribute to scientific knowledge!\n\nControls:\n• Press 1 for Species\n• Press 2 for Photos\n• Press 3 for Conservation".to_string()
                }
                _ => "Select a tab (1-4) to view content".to_string(),
            };
            **text = content;
        }
    }
}

// ====== LUNEX MAIN MENU SYSTEMS ======

#[derive(Component)]
pub struct LunexMainMenuUI;

#[derive(Component)]
pub struct LunexMenuButton {
    pub action: MainMenuAction,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MainMenuAction {
    NewGame,
    LoadGame, 
    Settings,
    Quit,
}

// System to setup Lunex main menu on state entry
pub fn setup_lunex_main_menu(mut commands: Commands) {
    info!("Setting up Lunex Main Menu UI");
    
    // Create Lunex UI root for main menu
    commands.spawn((
        UiLayoutRoot::new_2d(),
        UiFetchFromCamera::<0>,
        Name::new("Lunex Main Menu Root"),
    )).with_children(|ui| {
        // Main menu container - centered
        ui.spawn((
            UiLayout::window().pos(Rl((300.0, 200.0))).size((Rl(400.0), Rl(400.0))).pack(),
            UiColor::new(vec![
                (UiBase::id(), Color::srgba(0.1, 0.1, 0.1, 0.9)),
            ]),
            Name::new("Lunex Main Menu Container"),
            LunexMainMenuUI,
            LunexMigrationMarker,
        )).with_children(|menu| {
            // Title
            menu.spawn((
                Text::new("Avian Haven"),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.6, 0.2)), // Gold title
            ));
            
            // Menu buttons
            let buttons = [
                ("New Game", MainMenuAction::NewGame),
                ("Load Game", MainMenuAction::LoadGame),
                ("Settings", MainMenuAction::Settings),
                ("Quit", MainMenuAction::Quit),
            ];
            
            for (i, (label, action)) in buttons.iter().enumerate() {
                menu.spawn((
                    UiLayout::window().pos(Rl((50.0, 100.0 + i as f32 * 60.0))).size((Rl(300.0), Rl(50.0))).pack(),
                    UiColor::new(vec![
                        (UiBase::id(), Color::srgb(0.4, 0.3, 0.2)),
                        (UiHover::id(), Color::srgb(0.5, 0.4, 0.3)),
                    ]),
                    UiHover::new().forward_speed(15.0).backward_speed(8.0),
                    LunexMenuButton { action: *action },
                    Name::new(format!("Menu Button: {}", label)),
                )).with_children(|button| {
                    button.spawn((
                        Text2d::new(*label),
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

// System to cleanup Lunex main menu on state exit
pub fn cleanup_lunex_main_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<LunexMainMenuUI>>,
) {
    info!("Cleaning up Lunex Main Menu UI");
    for entity in menu_query.iter() {
        commands.entity(entity).despawn();
    }
}

// System to handle main menu button clicks (keyboard and mouse)
pub fn handle_lunex_main_menu_clicks(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut cursor_moved_events: EventReader<bevy::window::CursorMoved>,
    menu_button_query: Query<(&LunexMenuButton, &UiLayout), With<LunexMenuButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut app_state: ResMut<NextState<crate::AppState>>,
    mut cursor_position: Local<Option<Vec2>>,
) {
    // Handle keyboard navigation
    if keyboard.just_pressed(KeyCode::KeyN) || keyboard.just_pressed(KeyCode::Space) {
        info!("Main menu: New Game (keyboard)");
        app_state.set(crate::AppState::Playing);
        return;
    } else if keyboard.just_pressed(KeyCode::KeyS) {
        info!("Main menu: Settings (keyboard)");
        app_state.set(crate::AppState::Settings);
        return;
    } else if keyboard.just_pressed(KeyCode::KeyL) {
        info!("Main menu: Load Game (keyboard)");
        app_state.set(crate::AppState::LoadGame);
        return;
    } else if keyboard.just_pressed(KeyCode::KeyQ) || keyboard.just_pressed(KeyCode::Escape) {
        info!("Main menu: Quit (keyboard)");
        std::process::exit(0);
    }
    
    // Track cursor position
    for event in cursor_moved_events.read() {
        *cursor_position = Some(event.position);
    }
    
    // Handle mouse clicks
    if mouse.just_pressed(MouseButton::Left) && cursor_position.is_some() {
        let cursor_pos = cursor_position.unwrap();
        
        // Check if cursor is within any button bounds
        for (menu_button, layout) in menu_button_query.iter() {
            // Get button bounds (approximate - this is a simplified approach)
            // For the main menu, buttons are at:
            // New Game: (350, 300) to (650, 350)
            // Load Game: (350, 360) to (650, 410)  
            // Settings: (350, 420) to (650, 470)
            // Quit: (350, 480) to (650, 530)
            
            let button_bounds = match menu_button.action {
                MainMenuAction::NewGame => (350.0..650.0, 300.0..350.0),
                MainMenuAction::LoadGame => (350.0..650.0, 360.0..410.0),
                MainMenuAction::Settings => (350.0..650.0, 420.0..470.0),
                MainMenuAction::Quit => (350.0..650.0, 480.0..530.0),
            };
            
            // Check if cursor is within button bounds
            if button_bounds.0.contains(&cursor_pos.x) && button_bounds.1.contains(&cursor_pos.y) {
                match menu_button.action {
                    MainMenuAction::NewGame => {
                        info!("Main menu: New Game (mouse)");
                        app_state.set(crate::AppState::Playing);
                    }
                    MainMenuAction::LoadGame => {
                        info!("Main menu: Load Game (mouse)");
                        app_state.set(crate::AppState::LoadGame);
                    }
                    MainMenuAction::Settings => {
                        info!("Main menu: Settings (mouse)");
                        app_state.set(crate::AppState::Settings);
                    }
                    MainMenuAction::Quit => {
                        info!("Main menu: Quit (mouse)");
                        std::process::exit(0);
                    }
                }
                return; // Only handle one button at a time
            }
        }
    }
}