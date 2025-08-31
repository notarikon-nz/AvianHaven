// src/audio/components.rs
use bevy::prelude::*;
use crate::audio::resources::*;

#[derive(Component)]
pub struct PositionalAudioSource {
    pub source_entity: Entity,
    pub max_distance: f32,
    pub volume_curve: AudioVolumeCurve,
}

#[derive(Debug, Clone, Copy)]
pub enum AudioVolumeCurve {
    Linear,
    Exponential,
    InverseSquare,
}

#[derive(Component)]
pub struct AudioSinkComponent {
    pub sink_handle: Handle<bevy::audio::AudioSource>,
    pub command: Option<AudioCommand>,
}
