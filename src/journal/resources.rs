use bevy::prelude::*;
use std::collections::HashSet;
use crate::animation::components::BirdSpecies;

#[derive(Resource, Default)]
pub struct DiscoveredSpecies(pub HashSet<BirdSpecies>);

#[derive(Resource, Default)]
pub struct JournalData {
    pub selected_species: Option<BirdSpecies>,
}

