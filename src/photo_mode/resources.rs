use bevy::prelude::*;
use std::collections::HashSet;
use crate::bird::BirdSpecies;
use crate::photo_mode::components::PhotoScore;

#[derive(Resource)]
pub struct PhotoModeSettings {
    pub is_active: bool,
    pub toggle_key: KeyCode,
    pub capture_key: KeyCode,
    pub grid_toggle_key: KeyCode,
    pub settings_toggle_key: KeyCode,
    pub show_composition_grid: bool,
    pub show_camera_settings: bool,
    pub show_depth_preview: bool,
}

impl Default for PhotoModeSettings {
    fn default() -> Self {
        Self {
            is_active: false,
            toggle_key: KeyCode::KeyP,
            capture_key: KeyCode::Space,
            grid_toggle_key: KeyCode::KeyG,
            settings_toggle_key: KeyCode::KeyC,
            show_composition_grid: false,
            show_camera_settings: false,
            show_depth_preview: false,
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

#[derive(Clone)]
pub struct SavedPhoto {
    pub species: Option<BirdSpecies>,
    pub score: PhotoScore,
    pub image_handle: Handle<Image>,
    pub timestamp: f64, // Game time when photo was taken
}

#[derive(Resource, Default)]
pub struct PhotoCollection {
    pub photos: Vec<SavedPhoto>,
}

impl PhotoCollection {
    pub fn add_photo(&mut self, photo: SavedPhoto) {
        self.photos.push(photo);
        // Keep only the best 100 photos to prevent memory issues
        if self.photos.len() > 100 {
            self.photos.sort_by(|a, b| b.score.total_score.cmp(&a.score.total_score));
            self.photos.truncate(100);
        }
    }
    
    pub fn get_best_photos(&self, count: usize) -> Vec<&SavedPhoto> {
        let mut sorted_photos: Vec<&SavedPhoto> = self.photos.iter().collect();
        sorted_photos.sort_by(|a, b| b.score.total_score.cmp(&a.score.total_score));
        sorted_photos.into_iter().take(count).collect()
    }
    
    pub fn get_species_photos(&self, species: BirdSpecies) -> Vec<&SavedPhoto> {
        self.photos.iter()
            .filter(|photo| photo.species == Some(species))
            .collect()
    }
}