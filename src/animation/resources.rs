use bevy::prelude::*;
use std::collections::HashMap;
use crate::animation::components::{BirdSpecies, AnimationData};
use crate::bird_ai::components::BirdState;

#[derive(Resource, Default)]
pub struct TextureAtlasCache {
    pub atlases: HashMap<(BirdSpecies, BirdState), AnimationData>,
}