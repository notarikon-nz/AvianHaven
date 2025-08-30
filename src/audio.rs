use bevy::prelude::*;

use crate::{AppState, resources::SpawnBirdEvent};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AudioManager>()
            .add_systems(
                Update,
                handle_bird_spawn_audio.run_if(in_state(AppState::Playing))
            );
    }
}

#[derive(Resource, Default)]
pub struct AudioManager {
    pub bird_spawn_sound: Option<Handle<AudioSource>>,
}

fn handle_bird_spawn_audio(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnBirdEvent>,
    audio_manager: Res<AudioManager>,
) {
    for _ in spawn_events.read() {
        // Placeholder: In a real implementation, we'd load and play actual bird sounds
        // For now, we just demonstrate the audio system structure
        info!("Bird spawned - would play chirp sound here");
        
        // Example of how to play a sound when we have actual audio files:
        // if let Some(sound_handle) = &audio_manager.bird_spawn_sound {
        //     commands.spawn((
        //         AudioPlayer::new(sound_handle.clone()),
        //         PlaybackSettings::DESPAWN,
        //     ));
        // }
    }
}