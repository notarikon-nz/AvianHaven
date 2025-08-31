// src/audio/components.rs
use bevy::prelude::*;
use crate::audio::resources::*;

#[derive(Component)]
pub struct PositionalAudioSource {
    pub attenuation: f32,
}

impl Default for PositionalAudioSource {
    fn default() -> Self {
        Self {
            attenuation: 300.0, // Default maximum hearing distance
        }
    }
}

#[derive(Component)]
pub struct AudioSinkComponent {
    pub sink_handle: Handle<bevy::audio::AudioSource>,
    pub command: Option<AudioCommand>,
}
