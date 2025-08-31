use bevy::prelude::*;
use std::collections::HashMap;
use crate::bird_ai::components::BirdState;
use crate::bird::BirdSpecies;

#[derive(Component)]
pub struct AnimationController {
    pub timer: Timer,
    pub frames: usize,
    pub current_frame: usize,
    pub atlas_layout: Handle<TextureAtlasLayout>,
}

impl Default for AnimationController {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            frames: 1,
            current_frame: 0,
            atlas_layout: Handle::default(),
        }
    }
}

#[derive(Component, Default)]
pub struct AnimationLibrary {
    pub animations: HashMap<BirdState, AnimationData>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnimationData {
    pub texture_atlas_handle: Handle<TextureAtlasLayout>,
    pub texture_handle: Handle<Image>,
    pub frame_range: (usize, usize),
    pub fps: f32,
}

#[derive(Component)]
pub struct AnimationStateChange;

#[derive(Component, Clone, Copy)]
pub struct AnimatedBird {
    pub species: BirdSpecies,
}