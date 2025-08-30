use bevy::prelude::*;
use crate::animation::components::BirdSpecies;

#[derive(Component)]
pub struct PhotoTarget;

#[derive(Component)]
pub struct ViewfinderUI;

#[derive(Component)]
pub struct ScoreToast;

#[derive(Event)]
pub struct PhotoTakenEvent {
    pub score: u32,
    pub species: Option<BirdSpecies>,
    pub image_handle: Handle<Image>,
}

#[derive(Debug)]
pub struct PhotoScore {
    pub species_score: u32,
    pub centering_score: u32,
    pub clarity_score: u32,
    pub total_score: u32,
}