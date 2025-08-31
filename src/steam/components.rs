use bevy::prelude::*;

#[derive(Component)]
pub struct SteamUser {
    pub steam_id: u64,
    pub display_name: String,
}

#[derive(Component)]
pub struct WorkshopItem {
    pub workshop_id: u64,
    pub item_type: WorkshopItemType,
    pub author: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub enum WorkshopItemType {
    CustomBird {
        species_name: String,
        behavior_data: String,
    },
    CustomFeeder {
        feeder_name: String,
        stats: FeederWorkshopStats,
    },
    Habitat {
        theme_name: String,
        assets: Vec<String>,
    },
}

#[derive(Debug, Clone)]
pub struct FeederWorkshopStats {
    pub capacity: f32,
    pub attraction_radius: f32,
    pub supported_food_types: Vec<String>,
}