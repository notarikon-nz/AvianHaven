// src/user_interface/styles.rs
use bevy::prelude::*;

#[derive(Resource)]
pub struct ButtonStyle {
    pub width: Val,
    pub height: Val,
    pub normal_color: Color,
    pub hovered_color: Color,
    pub pressed_color: Color,
    pub border_color: Color,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            width: Val::Px(100.0),
            height: Val::Px(100.0),
            normal_color: Color::srgb(0.15, 0.15, 0.15),
            hovered_color: Color::srgb(0.25, 0.25, 0.25),
            pressed_color: Color::srgb(0.35, 0.35, 0.35),
            border_color: Color::srgb(0.5, 0.5, 0.5),
        }
    }
}

#[derive(Resource)]
pub struct PanelStyle {
    pub background_color: Color,
    pub border_color: Color,
}

impl Default for PanelStyle {
    fn default() -> Self {
        Self {
            background_color: Color::srgba(0.1, 0.1, 0.1, 0.9),
            border_color: Color::srgb(0.3, 0.3, 0.3),
        }
    }
}