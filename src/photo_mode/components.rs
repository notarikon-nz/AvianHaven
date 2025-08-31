use bevy::prelude::*;
use crate::bird::BirdSpecies;

#[derive(Component)]
pub struct PhotoTarget;

#[derive(Component)]
pub struct ViewfinderUI;

#[derive(Component)]
pub struct ScoreToast;

#[derive(Event)]
pub struct PhotoTakenEvent {
    pub score: PhotoScore,
    pub species: Option<BirdSpecies>,
    pub image_handle: Handle<Image>,
}

#[derive(Debug, Clone)]
pub struct PhotoScore {
    pub species_score: u32,
    pub centering_score: u32,
    pub clarity_score: u32,
    pub behavior_score: u32,  // New: Score based on bird's current behavior
    pub timing_score: u32,    // New: Score based on photogenic moment timing
    pub rarity_bonus: u32,    // New: Bonus for rare behaviors or poses
    pub total_score: u32,
}