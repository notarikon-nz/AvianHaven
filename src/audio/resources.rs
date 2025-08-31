// src/audio/resources.rs
use bevy::prelude::*;
use std::collections::{HashMap, VecDeque};
use crate::animation::components::BirdSpecies;

#[derive(Resource)]
pub struct AudioSettings {
    pub volume: f32,
    pub max_concurrent_sounds: usize,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            volume: 1.0,
            max_concurrent_sounds: 16,
        }
    }
}

#[derive(Resource)]
pub struct AudioManager {
    pub available_sinks: VecDeque<Entity>,
    pub in_use_sinks: HashMap<Entity, AudioCommand>,
    pub listener_position: Vec2,
}

impl Default for AudioManager {
    fn default() -> Self {
        Self {
            available_sinks: VecDeque::new(),
            in_use_sinks: HashMap::new(),
            listener_position: Vec2::ZERO,
        }
    }
}

#[derive(Event)]
pub struct AudioPlayEvent {
    pub source: AudioSource,
    pub command: AudioCommand,
}

#[derive(Clone, Debug)]
pub enum AudioCommand {
    PlayGlobal,
    PlayAt(Vec2),
    PlayFromEntity(Entity),
}

#[derive(Clone, Debug)]
pub enum AudioSource {
    BirdVocalization(Handle<bevy::audio::AudioSource>, BirdSpecies),
    AmbientTrack(Handle<bevy::audio::AudioSource>),
    UiSound(Handle<bevy::audio::AudioSource>),
}