use bevy::prelude::*;
use std::collections::HashSet;
use crate::animation::components::BirdSpecies;

#[derive(Resource)]
pub struct PhotoModeSettings {
    pub is_active: bool,
    pub toggle_key: KeyCode,
    pub capture_key: KeyCode,
}

impl Default for PhotoModeSettings {
    fn default() -> Self {
        Self {
            is_active: false,
            toggle_key: KeyCode::KeyP,
            capture_key: KeyCode::Space,
        }
    }
}

#[derive(Resource, Default)]
pub struct CurrencyResource(pub u32);

#[derive(Resource, Default)]
pub struct DiscoveredSpecies {
    pub species: HashSet<BirdSpecies>,
}

impl DiscoveredSpecies {
    pub fn discover(&mut self, species: BirdSpecies) -> bool {
        self.species.insert(species)
    }
    
    pub fn is_discovered(&self, species: &BirdSpecies) -> bool {
        self.species.contains(species)
    }
}