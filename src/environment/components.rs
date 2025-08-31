use bevy::prelude::*;

#[derive(Component)]
pub struct EnvironmentEntity;

#[derive(Component)]
pub struct WeatherEffectEntity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Season {
    Spring,
    Summer,
    Fall,
    Winter,
}

impl Season {
    pub fn next(&self) -> Self {
        match self {
            Self::Spring => Self::Summer,
            Self::Summer => Self::Fall,
            Self::Fall => Self::Winter,
            Self::Winter => Self::Spring,
        }
    }
    
    pub fn bird_activity_modifier(&self) -> f32 {
        match self {
            Self::Spring => 1.3,  // High bird activity during breeding season
            Self::Summer => 1.1,  // Slightly elevated activity
            Self::Fall => 1.2,   // Migration season activity
            Self::Winter => 0.7, // Reduced activity, fewer species
        }
    }
    
    pub fn migration_factor(&self) -> f32 {
        match self {
            Self::Spring => 0.8, // Some migrants arriving
            Self::Summer => 0.3, // Most birds are residents
            Self::Fall => 0.9,   // Peak migration season
            Self::Winter => 0.5, // Some migrants, mostly residents
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Weather {
    Clear,
    Cloudy,
    Rainy,
    Snowy,
    Windy,
}

impl Weather {
    pub fn bird_activity_modifier(&self) -> f32 {
        match self {
            Self::Clear => 1.0,     // Normal activity
            Self::Cloudy => 0.9,    // Slightly reduced
            Self::Rainy => 0.4,     // Birds seek shelter
            Self::Snowy => 0.3,     // Minimal activity
            Self::Windy => 0.6,     // Reduced flying, more ground feeding
        }
    }
    
    pub fn feeder_preference_modifier(&self, feeder_type: &crate::feeder::FeederType) -> f32 {
        use crate::feeder::FeederType;
        match (self, feeder_type) {
            (Self::Rainy | Self::Snowy, FeederType::Ground) => 0.2, // Avoid ground feeding in bad weather
            (Self::Rainy | Self::Snowy, _) => 1.2, // Prefer covered feeders
            (Self::Windy, FeederType::Nectar) => 0.6, // Harder to feed from hanging feeders
            (Self::Clear, _) => 1.0,
            _ => 0.8,
        }
    }
    
    pub fn background_color(&self) -> Color {
        match self {
            Self::Clear => Color::srgb(0.7, 0.9, 1.0),     // Bright blue
            Self::Cloudy => Color::srgb(0.6, 0.6, 0.7),    // Gray
            Self::Rainy => Color::srgb(0.4, 0.4, 0.5),     // Dark gray
            Self::Snowy => Color::srgb(0.9, 0.9, 0.95),    // Light gray/white
            Self::Windy => Color::srgb(0.6, 0.7, 0.8),     // Dusty blue
        }
    }
}