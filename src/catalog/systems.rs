use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::catalog::{components::*, resources::*};
use crate::bird_ai::components::{SmartObject, ProvidesUtility};
use crate::despawn::SafeDespawn;

// Helper function to get filename for object sprites
pub fn object_filename(item_type: &PlaceableItemType) -> String {
    match item_type {
        PlaceableItemType::CardboardBox => "cardboard_box".to_string(),
        PlaceableItemType::CushionRed => "cushion_red".to_string(),
        PlaceableItemType::CushionBlue => "cushion_blue".to_string(),
        PlaceableItemType::WoodenPerch => "wooden_perch".to_string(),
        PlaceableItemType::FancyPerch => "fancy_perch".to_string(),
        PlaceableItemType::BasicBirdSeed => "basic_seed".to_string(),
        PlaceableItemType::PremiumSeed => "premium_seed".to_string(),
        PlaceableItemType::SuetCake => "suet_cake".to_string(),
        PlaceableItemType::NectarFeeder => "nectar_feeder".to_string(),
        PlaceableItemType::FruitDispenser => "fruit_dispenser".to_string(),
        PlaceableItemType::BasicBirdbath => "basic_birdbath".to_string(),
        PlaceableItemType::FountainBirdbath => "fountain_birdbath".to_string(),
        PlaceableItemType::StreamFeature => "stream_feature".to_string(),
        PlaceableItemType::GardenGnome => "garden_gnome".to_string(),
        PlaceableItemType::WindChime => "wind_chime".to_string(),
        PlaceableItemType::FlowerPot => "flower_pot".to_string(),
        PlaceableItemType::BirdHouse => "bird_house".to_string(),
        PlaceableItemType::NestingBox => "nesting_box".to_string(),
        PlaceableItemType::MirrorToy => "mirror_toy".to_string(),
        PlaceableItemType::BellToy => "bell_toy".to_string(),
        PlaceableItemType::SwingSeat => "swing_seat".to_string(),
    }
}

pub fn setup_catalog_items(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
) {
    // Setup catalog UI (initially hidden)
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            left: Val::Percent(0.0),
            top: Val::Percent(0.0),
            display: Display::None,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        CatalogUI,
    )).with_children(|parent| {
        // Catalog window
        parent.spawn((
            Node {
                width: Val::Percent(85.0),
                height: Val::Percent(85.0),
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.95, 0.92, 0.88)),
            BorderColor(Color::srgb(0.6, 0.4, 0.2)),
            CatalogContainer,
        )).with_children(|catalog| {
            // Title bar
            catalog.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(60.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::horizontal(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.7, 0.5, 0.3)),
            )).with_children(|title| {
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
                    Text::new("Credits: 0"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.9, 0.1)),
                ));
            });
            
            // Content area
            catalog.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
            )).with_children(|content| {
                // Category sidebar
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
                    
                    // Category buttons
                    let categories = [
                        (ItemCategory::Comfort, "Comfort"),
                        (ItemCategory::Food, "Food"),
                        (ItemCategory::Water, "Water"),
                        (ItemCategory::Decorative, "Decorative"),
                        (ItemCategory::Special, "Special"),
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
                            CategoryButton { category },
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
                
                // Items grid area
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
                    ItemsGrid,
                ));
            });
        });
    });
}


pub fn handle_catalog_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut catalog_state: ResMut<CatalogState>,
    mut placed_objects: ResMut<PlacedObjects>,
    mut catalog_query: Query<&mut Node, With<CatalogUI>>,
) {
    // Toggle catalog with C key
    if keyboard.just_pressed(KeyCode::KeyC) {
        catalog_state.is_open = !catalog_state.is_open;
        
        for mut node in catalog_query.iter_mut() {
            node.display = if catalog_state.is_open {
                Display::Flex
            } else {
                Display::None
            };
        }
        
        // Exit placement mode when closing catalog
        if !catalog_state.is_open {
            placed_objects.placement_mode = false;
        }
    }
    
    // Exit placement mode with Escape
    if keyboard.just_pressed(KeyCode::Escape) {
        placed_objects.placement_mode = false;
        catalog_state.selected_item = None;
    }
}

pub fn handle_category_buttons(
    mut interaction_query: Query<
        (&Interaction, &CategoryButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut catalog_state: ResMut<CatalogState>,
) {
    for (interaction, category_button, mut bg_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                catalog_state.selected_category = category_button.category;
                *bg_color = Color::srgb(0.6, 0.5, 0.4).into(); // Pressed color
            }
            Interaction::Hovered => {
                *bg_color = Color::srgb(0.8, 0.75, 0.7).into(); // Hover color
            }
            Interaction::None => {
                *bg_color = if catalog_state.selected_category == category_button.category {
                    Color::srgb(0.6, 0.5, 0.4).into() // Selected color
                } else {
                    Color::srgb(0.75, 0.7, 0.65).into() // Default color
                };
            }
        }
    }
}

pub fn handle_purchase_buttons(
    mut interaction_query: Query<
        (&Interaction, &PurchaseButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut purchase_events: EventWriter<PurchaseItemEvent>,
    inventory: Res<PlayerInventory>,
) {
    for (interaction, purchase_button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            let price = purchase_button.item_type.price();
            if inventory.currency >= price {
                purchase_events.write(PurchaseItemEvent {
                    item_type: purchase_button.item_type.clone(),
                });
            }
        }
    }
}

pub fn handle_place_buttons(
    mut interaction_query: Query<
        (&Interaction, &PlaceButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut catalog_state: ResMut<CatalogState>,
    inventory: Res<PlayerInventory>,
) {
    for (interaction, place_button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            let owned_count = inventory.owned_items.get(&place_button.item_type).unwrap_or(&0);
            if *owned_count > 0 {
                catalog_state.selected_item = Some(place_button.item_type.clone());
            }
        }
    }
}

pub fn handle_purchase_events(
    mut purchase_events: EventReader<PurchaseItemEvent>,
    mut inventory: ResMut<PlayerInventory>,
    mut notifications: EventWriter<crate::notifications::resources::ShowNotificationEvent>,
    mut catalog_state: ResMut<CatalogState>,
) {
    for event in purchase_events.read() {
        let price = event.item_type.price();
        
        if inventory.currency >= price {
            inventory.currency -= price;
            *inventory.owned_items.entry(event.item_type.clone()).or_insert(0) += 1;
            
            // Trigger UI update
            catalog_state.set_changed();
            
            notifications.write(crate::notifications::resources::ShowNotificationEvent {
                notification: crate::notifications::components::NotificationType::Currency {
                    amount: price,
                    reason: format!("Bought {}", event.item_type.name()),
                },
            });
        } else {
            notifications.write(crate::notifications::resources::ShowNotificationEvent {
                notification: crate::notifications::components::NotificationType::Warning {
                    message: format!("Need {} coins to buy {}", price, event.item_type.name()),
                },
            });
        }
    }
}

pub fn handle_place_object_events(
    mut commands: Commands,
    mut place_events: EventReader<PlaceObjectEvent>,
    mut placed_objects: ResMut<PlacedObjects>,
    mut inventory: ResMut<PlayerInventory>,
    asset_server: Res<AssetServer>,
) {
    for event in place_events.read() {
        // Check if player owns this item
        if let Some(count) = inventory.owned_items.get_mut(&event.item_type) {
            if *count > 0 {
                *count -= 1;
                
                // Get item properties
                let item_size = event.item_type.physical_size();
                let actions = event.item_type.provides_actions();
                
                // Spawn the object in the world with physics and smart object components
                let mut entity_commands = commands.spawn((
                    Sprite {
                        image: asset_server.load(&format!("objects/{}.png", 
                            object_filename(&event.item_type))),
                        ..default()
                    },
                    Transform::from_translation(event.position),
                    RigidBody::Fixed,
                    Collider::cuboid(item_size.x / 2.0, item_size.y / 2.0),
                    Sensor, // Allow birds to overlap but detect collisions
                    PlaceableObject {
                        item_type: event.item_type.clone(),
                        placement_cost: event.item_type.price(),
                    },
                    SmartObject, // Mark as discoverable by bird AI
                ));
                
                // Add ProvidesUtility components for each action this item supports
                let base_utility = event.item_type.base_utility();
                let interaction_range = event.item_type.interaction_range();
                
                // For items that provide multiple actions, we need to spawn multiple entities
                // or use the first action as primary (simpler approach)
                if let Some(primary_action) = actions.first() {
                    entity_commands.insert(ProvidesUtility {
                        action: *primary_action,
                        base_utility,
                        range: interaction_range,
                    });
                }
                
                let entity = entity_commands.id();
                
                // For items with multiple actions, spawn additional utility providers at the same location
                for action in actions.iter().skip(1) {
                    commands.spawn((
                        // Invisible utility provider at same location
                        Transform::from_translation(event.position),
                        SmartObject,
                        ProvidesUtility {
                            action: *action,
                            base_utility: base_utility * 0.8, // Slightly lower utility for secondary actions
                            range: interaction_range,
                        },
                    ));
                }
                
                placed_objects.objects.insert(entity, event.item_type.clone());
            }
        }
    }
}

pub fn update_catalog_ui(
    mut commands: Commands,
    catalog_state: Res<CatalogState>,
    inventory: Res<PlayerInventory>,
    asset_server: Res<AssetServer>,
    mut items_grid_query: Query<Entity, With<ItemsGrid>>,
    currency_text_query: Query<Entity, (With<Text>, Without<ItemsGrid>)>,
    mut text_query: Query<&mut Text>,
) {
    if catalog_state.is_changed() {
        // Update currency display
        for text_entity in currency_text_query.iter() {
            if let Ok(mut text) = text_query.get_mut(text_entity) {
                text.0 = format!("Credits: {}", inventory.currency);
                break; // Only update the first currency text found
            }
        }
        
        // Clear and rebuild items grid
        if let Ok(grid_entity) = items_grid_query.single_mut() {
            // Despawn all existing item cards
            if let Ok(mut entity_commands) = commands.get_entity(grid_entity) {
                entity_commands.safe_despawn();
            }
            // Respawn the grid entity 
            let new_grid = commands.spawn((
                Node {
                    width: Val::Percent(80.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(15.0)),
                    overflow: Overflow::scroll_y(),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.92, 0.88, 0.84)),
                ItemsGrid,
            )).id();
            
            // Add new items based on selected category
            let items = catalog_state.selected_category.items();
            let items_per_row = 3;
            
            commands.entity(new_grid).with_children(|grid| {
                // Create rows
                for chunk in items.chunks(items_per_row) {
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
                        for item_type in chunk {
                            let owned_count = inventory.owned_items.get(item_type).unwrap_or(&0);
                            let price = item_type.price();
                            let can_afford = inventory.currency >= price;
                            let has_items = *owned_count > 0;
                            
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
                                BorderColor(if has_items {
                                    Color::srgb(0.2, 0.6, 0.2) // Green border if owned
                                } else {
                                    Color::srgb(0.6, 0.4, 0.2)
                                }),
                                ItemCard { item_type: item_type.clone() },
                            )).with_children(|card| {
                                // Item image placeholder
                                card.spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(80.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        margin: UiRect::bottom(Val::Px(5.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.9, 0.9, 0.85)),
                                )).with_children(|img_container| {
                                    img_container.spawn((
                                        ImageNode::new(asset_server.load(&format!("objects/{}.png", 
                                            object_filename(&item_type)))),
                                        Node {
                                            width: Val::Px(64.0),
                                            height: Val::Px(64.0),
                                            ..default()
                                        },
                                    ));
                                });
                                
                                // Item name
                                card.spawn((
                                    Text::new(item_type.name()),
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
                                
                                // Price and owned count
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
                                        TextColor(if can_afford { Color::srgb(0.2, 0.5, 0.2) } else { Color::srgb(0.7, 0.2, 0.2) }),
                                    ));
                                    
                                    if *owned_count > 0 {
                                        info.spawn((
                                            Text::new(format!("Owned: {}", owned_count)),
                                            TextFont {
                                                font_size: 10.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.2, 0.4, 0.6)),
                                        ));
                                    }
                                });
                                
                                // Buttons row
                                card.spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceBetween,
                                        column_gap: Val::Px(5.0),
                                        ..default()
                                    },
                                )).with_children(|buttons| {
                                    // Purchase button
                                    let purchase_disabled = !can_afford;
                                    buttons.spawn((
                                        Button,
                                        Node {
                                            width: Val::Percent(48.0),
                                            height: Val::Px(25.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BackgroundColor(if purchase_disabled {
                                            Color::srgb(0.6, 0.6, 0.6)
                                        } else {
                                            Color::srgb(0.3, 0.7, 0.3)
                                        }),
                                        PurchaseButton { item_type: item_type.clone() },
                                    )).with_children(|btn| {
                                        btn.spawn((
                                            Text::new("Buy"),
                                            TextFont {
                                                font_size: 10.0,
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                        ));
                                    });
                                    
                                    // Place button
                                    let place_disabled = !has_items;
                                    buttons.spawn((
                                        Button,
                                        Node {
                                            width: Val::Percent(48.0),
                                            height: Val::Px(25.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BackgroundColor(if place_disabled {
                                            Color::srgb(0.6, 0.6, 0.6)
                                        } else {
                                            Color::srgb(0.2, 0.5, 0.8)
                                        }),
                                        PlaceButton { item_type: item_type.clone() },
                                    )).with_children(|btn| {
                                        btn.spawn((
                                            Text::new("Place"),
                                            TextFont {
                                                font_size: 10.0,
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                        ));
                                    });
                                });
                            });
                        }
                    });
                }
            });
        }
    }
}


pub fn handle_object_placement(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut placed_objects: ResMut<PlacedObjects>,
    catalog_state: Res<CatalogState>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    asset_server: Res<AssetServer>,
    mut place_events: EventWriter<PlaceObjectEvent>,
) {
    let Ok(window) = windows.single() else { return };
    let Ok((camera, camera_transform)) = camera_query.single() else { return };
    
    // Handle placement mode
    if placed_objects.placement_mode {
        if let Some(cursor_position) = window.cursor_position() {
            // Convert screen position to world position 
            // For now, just use cursor position directly as world position
            let world_position = Vec2::new(cursor_position.x - 400.0, 300.0 - cursor_position.y); // Basic screen to world conversion
            
            // Update ghost position
            if let Some(ghost_entity) = placed_objects.ghost_entity {
                if let Ok(mut ghost_transform) = commands.get_entity(ghost_entity) {
                    ghost_transform.insert(Transform::from_translation(world_position.extend(1.0)));
                }
            }
            
            // Place object on left click
            if mouse_button.just_pressed(MouseButton::Left) {
                if let Some(item_type) = &catalog_state.selected_item {
                    place_events.write(PlaceObjectEvent {
                        item_type: item_type.clone(),
                        position: world_position.extend(1.0),
                    });
                    
                    // Exit placement mode
                    placed_objects.placement_mode = false;
                    if let Some(ghost) = placed_objects.ghost_entity.take() {
                        commands.entity(ghost).safe_despawn();
                    }
                }
            }
        }
        
        // Cancel placement on right click
        if mouse_button.just_pressed(MouseButton::Right) {
            placed_objects.placement_mode = false;
            if let Some(ghost) = placed_objects.ghost_entity.take() {
                commands.entity(ghost).safe_despawn();
            }
        }
    }
}

pub fn start_placement_mode(
    mut commands: Commands,
    mut placed_objects: ResMut<PlacedObjects>,
    catalog_state: Res<CatalogState>,
    asset_server: Res<AssetServer>,
) {
    if let Some(item_type) = &catalog_state.selected_item {
        if !placed_objects.placement_mode {
            placed_objects.placement_mode = true;
            
            // Create ghost object
            let ghost = commands.spawn((
                Sprite {
                    image: asset_server.load(&format!("objects/{}.png", 
                        object_filename(item_type))),
                    color: Color::srgba(1.0, 1.0, 1.0, 0.5),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 10.0),
                PlacementGhost,
            )).id();
            
            placed_objects.ghost_entity = Some(ghost);
        }
    }
}

pub fn cleanup_catalog_ui(
    mut commands: Commands,
    catalog_ui_query: Query<Entity, With<crate::catalog::components::CatalogUI>>,
) {
    for entity in catalog_ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Simplified catalog visibility management without content rebuilding
pub fn update_catalog_visibility(
    catalog_state: Res<CatalogState>,
    mut catalog_query: Query<&mut Node, With<CatalogUI>>,
) {
    if catalog_state.is_changed() {
        for mut node in catalog_query.iter_mut() {
            node.display = if catalog_state.is_open {
                Display::Flex
            } else {
                Display::None
            };
        }
    }
}