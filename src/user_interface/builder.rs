use bevy::prelude::*;
use crate::user_interface::styles::{ButtonStyle, PanelStyle};
use crate::feeder::{FeederType};

pub fn ui_text(
    parent: &mut ChildSpawner,
    text: &str,
    font_size: f32,
    color: Color,
) -> Entity {
    parent.spawn((
        Text::new(text),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(color),
    )).id()
}

pub fn ui_button(
    commands: &mut Commands,
    button_style: &ButtonStyle,
    on_click: impl Bundle,
) -> Entity {
    commands.spawn((
        Button,
        Node {
            width: button_style.width,
            height: button_style.height,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(button_style.normal_color),
        BorderColor(button_style.border_color),
        on_click,
    )).id()
}

pub fn ui_icon(
    commands: &mut Commands,
    texture: Handle<Image>,
    size: Val,
) -> Entity {
    commands.spawn((
        ImageNode::new(texture),
        Node {
            width: size,
            height: size,
            ..default()
        },
    )).id()
}

pub fn ui_panel(
    commands: &mut Commands,
    panel_style: &PanelStyle,
) -> Entity {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        BackgroundColor(panel_style.background_color),
        BorderColor(panel_style.border_color),
    )).id()
}

/*
// For Steam Workshop content
pub fn ui_workshop_card(
    commands: &mut Commands,
    title: &str,
    author: &str,
    rating: f32,
) -> Entity {
    // Workshop content card with rating stars
}

// For achievement notifications
pub fn ui_achievement_toast(
    commands: &mut Commands,
    achievement_name: &str,
    icon: Handle<Image>,
) -> Entity {
    // Sliding achievement notification
}

// For seasonal content
pub fn ui_seasonal_banner(
    commands: &mut Commands,
    season: &str,
    expires_in: &str,
) -> Entity {
    // Time-limited content banner
}

// For feeder management
pub fn ui_feeder_slot(
    commands: &mut Commands,
    feeder_type: Option<FeederType>,
    capacity: f32,
) -> Entity {
    // Drag-drop feeder placement UI
}

// For bird behavior stats
pub fn ui_stat_bar(
    commands: &mut Commands,
    label: &str,
    value: f32,
    max_value: f32,
    color: Color,
) -> Entity {
    // Progress bar for hunger/thirst/energy
}

// For educational content
pub fn ui_fact_card(
    commands: &mut Commands,
    fact: &str,
    source: &str,
) -> Entity {
    // Educational fact display with citation
}

// For Steam integration
pub fn ui_trading_card(
    commands: &mut Commands,
    card_image: Handle<Image>,
    rarity: &str,
) -> Entity {
    // Steam trading card display
}

*/