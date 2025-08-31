use bevy::prelude::*;
use std::collections::HashMap;
use crate::bird::BirdSpecies;
use crate::animation::components::AnimationData;
use crate::bird_ai::components::BirdState;

#[derive(Resource, Default)]
pub struct TextureAtlasCache {
    pub atlases: HashMap<(BirdSpecies, BirdState), AnimationData>,
}