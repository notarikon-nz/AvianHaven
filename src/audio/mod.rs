// src/audio/mod.rs
use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

use resources::*;
use systems::*;
use crate::AppState;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AudioSettings>()
            .init_resource::<AudioManager>()
            .add_event::<AudioPlayEvent>()
            .add_systems(Startup, audio_setup_system)
            .add_systems(Update, (
                audio_event_system,
                update_positional_audio_system,
                audio_cleanup_system,
                bird_vocalization_system,
            ).run_if(in_state(AppState::Playing)));
    }
}

// Helper functions for common audio operations
pub fn play_ambient(commands: &mut Commands, handle: Handle<bevy::audio::AudioSource>) {
    commands.trigger(AudioPlayEvent {
        source: crate::audio::resources::AudioSource::AmbientTrack(handle),
        command: AudioCommand::PlayGlobal,
    });
}

pub fn play_bird_sound(commands: &mut Commands, entity: Entity, species: crate::animation::components::BirdSpecies, handle: Handle<bevy::audio::AudioSource>) {
    commands.trigger(AudioPlayEvent {
        source: crate::audio::resources::AudioSource::BirdVocalization(handle, species),
        command: AudioCommand::PlayFromEntity(entity),
    });
}

pub fn play_ui_sound(commands: &mut Commands, handle: Handle<bevy::audio::AudioSource>) {
    commands.trigger(AudioPlayEvent {
        source: crate::audio::resources::AudioSource::UiSound(handle),
        command: AudioCommand::PlayGlobal,
    });
}