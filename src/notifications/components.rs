use bevy::prelude::*;

#[derive(Component)]
pub struct NotificationContainer;

#[derive(Component)]
pub struct Notification {
    pub lifetime: Timer,
    pub notification_type: NotificationType,
}

#[derive(Debug, Clone)]
pub enum NotificationType {
    Achievement {
        title: String,
        description: String,
        currency_reward: u32,
    },
    Currency {
        amount: u32,
        reason: String,
    },
    Warning {
        message: String,
    },
    Info {
        message: String,
    },
}

impl NotificationType {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Achievement { .. } => "🏆",
            Self::Currency { .. } => "💰",
            Self::Warning { .. } => "⚠️",
            Self::Info { .. } => "ℹ️",
        }
    }
    
    pub fn background_color(&self) -> Color {
        match self {
            Self::Achievement { .. } => Color::srgba(0.1, 0.4, 0.1, 0.9), // Green
            Self::Currency { .. } => Color::srgba(0.4, 0.3, 0.1, 0.9),    // Gold
            Self::Warning { .. } => Color::srgba(0.4, 0.2, 0.1, 0.9),     // Orange
            Self::Info { .. } => Color::srgba(0.1, 0.2, 0.4, 0.9),        // Blue
        }
    }
    
    pub fn border_color(&self) -> Color {
        match self {
            Self::Achievement { .. } => Color::srgb(0.2, 0.8, 0.2), // Bright green
            Self::Currency { .. } => Color::srgb(0.8, 0.6, 0.2),    // Bright gold
            Self::Warning { .. } => Color::srgb(0.8, 0.4, 0.2),     // Bright orange
            Self::Info { .. } => Color::srgb(0.2, 0.4, 0.8),        // Bright blue
        }
    }
}