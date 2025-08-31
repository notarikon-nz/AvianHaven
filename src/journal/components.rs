// src/journal/components.rs
use bevy::prelude::*;
use crate::animation::components::BirdSpecies;

#[derive(Component)]
pub struct JournalMenu;

#[derive(Component)]
pub struct SpeciesButton(pub BirdSpecies);

#[derive(Component)]
pub struct DetailPanel;

#[derive(Component)]
pub struct CloseButton;
