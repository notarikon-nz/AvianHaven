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
                    handle_popout_menu,
                    update_popout_menu_visibility,
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

#[derive(Component)]
struct PopOutMenu;

#[derive(Component)]
struct PopOutMenuButton;

#[derive(Component)]
struct PopOutMenuExpanded(bool);

#[derive(Component)]
struct MenuIconButton {
    pub action: MenuAction,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MenuAction {
    Catalog,
    Journal,
    Photography,
    Settings,
}

#[derive(Component)]
struct CurrencyCounter;

#[derive(Component)]
struct MenuItemsContainer;

fn setup_ui(mut commands: Commands) {
    // Pop-out menu in top left
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(20.0),
            width: Val::Px(48.0),
            height: Val::Px(48.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        PopOutMenu,
        PopOutMenuExpanded(false),
    )).with_children(|parent| {
        // Main menu button (hamburger icon placeholder)
        parent.spawn((
            Button,
            PopOutMenuButton,
            Node {
                width: Val::Px(48.0),
                height: Val::Px(48.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.3, 0.3, 0.4)),
            BorderColor(Color::srgb(0.2, 0.2, 0.3)),
        )).with_children(|button| {
            button.spawn((
                Text::new("☰"), // Hamburger menu placeholder
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
        
        // Menu items container (initially hidden)
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(60.0),
                top: Val::Px(0.0),
                width: Val::Px(240.0), // 4 * 48px + gaps
                height: Val::Px(48.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                ..default()
            },
            Visibility::Hidden, // Start hidden
            MenuItemsContainer,
            Name::new("Menu Items Container"),
        )).with_children(|menu_container| {
            let menu_items = [
                ("📷", MenuAction::Photography), // Camera icon placeholder
                ("📖", MenuAction::Journal),     // Book icon placeholder
                ("🛍️", MenuAction::Catalog),    // Shop icon placeholder
                ("⚙️", MenuAction::Settings),   // Settings icon placeholder
            ];
            
            for (icon, action) in menu_items {
                menu_container.spawn((
                    Button,
                    MenuIconButton { action },
                    Node {
                        width: Val::Px(48.0),
                        height: Val::Px(48.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.4, 0.4, 0.5)),
                    BorderColor(Color::srgb(0.3, 0.3, 0.4)),
                )).with_children(|button| {
                    button.spawn((
                        Text::new(icon),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            }
        });
    });
    
    // Currency counter in bottom left
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            bottom: Val::Px(20.0),
            width: Val::Auto,
            height: Val::Auto,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(8.0),
            padding: UiRect::all(Val::Px(10.0)),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
        BorderColor(Color::srgb(0.8, 0.6, 0.2)),
        CurrencyCounter,
    )).with_children(|parent| {
        // Currency icon placeholder (16x16)
        parent.spawn((
            Text::new("💰"), // Coin icon placeholder
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.6, 0.2)),
        ));
        
        // Currency amount text
        parent.spawn((
            Text::new("0"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.6, 0.2)),
            CurrencyText,
        ));
    });

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

// bottom left currency indicator
fn update_currency_ui(
    currency: Res<CurrencyResource>,
    mut text_query: Query<&mut Text, With<CurrencyText>>,
) {
    if currency.is_changed() {
        for mut text in text_query.iter_mut() {
            **text = format!("{}", currency.0);
        }
    }
}

// System to handle pop-out menu interactions
fn handle_popout_menu(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PopOutMenuButton>, Without<MenuIconButton>),
    >,
    mut menu_query: Query<&mut PopOutMenuExpanded, With<PopOutMenu>>,
    mut menu_icon_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuIconButton),
        (Changed<Interaction>, With<MenuIconButton>, Without<PopOutMenuButton>),
    >,
    mut app_state: ResMut<NextState<crate::AppState>>,
    mut catalog_state: ResMut<crate::catalog::resources::CatalogState>,
) {
    // Handle main menu button clicks
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.2, 0.2, 0.3).into();
                if let Ok(mut expanded) = menu_query.single_mut() {
                    expanded.0 = !expanded.0;
                }
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.4, 0.4, 0.5).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.3, 0.3, 0.4).into();
            }
        }
    }
    
    // Handle menu icon button clicks
    for (interaction, mut color, menu_button) in menu_icon_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.3, 0.3, 0.4).into();
                
                // Close the menu first
                if let Ok(mut expanded) = menu_query.single_mut() {
                    expanded.0 = false;
                }
                
                // Handle the action
                match menu_button.action {
                    MenuAction::Photography => {
                        // Toggle photo mode - this would need photo mode integration
                        info!("Photography mode toggle requested");
                    }
                    MenuAction::Journal => {
                        app_state.set(crate::AppState::Journal);
                    }
                    MenuAction::Catalog => {
                        catalog_state.is_open = !catalog_state.is_open;
                    }
                    MenuAction::Settings => {
                        app_state.set(crate::AppState::Settings);
                    }
                }
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.5, 0.5, 0.6).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.4, 0.4, 0.5).into();
            }
        }
    }
}

// System to update pop-out menu visibility  
fn update_popout_menu_visibility(
    menu_query: Query<&PopOutMenuExpanded, (With<PopOutMenu>, Changed<PopOutMenuExpanded>)>,
    mut menu_items_query: Query<&mut Visibility, With<MenuItemsContainer>>,
) {
    for expanded in menu_query.iter() {
        // Update visibility of menu items container
        if let Ok(mut visibility) = menu_items_query.single_mut() {
            *visibility = if expanded.0 { Visibility::Visible } else { Visibility::Hidden };
        }
    }
}

fn cleanup_gameplay_ui(
    mut commands: Commands,
    ui_query: Query<Entity, Or<(
        With<SpawnButton>, 
        With<BirdCounterText>, 
        With<EnvironmentText>, 
        With<CurrencyText>,
        With<PopOutMenu>,
        With<CurrencyCounter>
    )>>,
) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn();
    }
}