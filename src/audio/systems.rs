// src/audio/systems.rs
use bevy::prelude::*;
use std::collections::VecDeque;
use crate::audio::{components::*, resources::*};
use crate::bird_ai::components::{BirdAI, BirdState};
use crate::animation::components::{AnimatedBird, BirdSpecies};

pub fn audio_setup_system(
    mut commands: Commands,
    mut audio_manager: ResMut<AudioManager>,
    audio_settings: Res<AudioSettings>,
) {
    info!("Initializing audio system with {} concurrent sound slots", audio_settings.max_concurrent_sounds);
    
    // Pre-populate sink pool
    for i in 0..audio_settings.max_concurrent_sounds {
        let sink_entity = commands.spawn((
            AudioSinkComponent {
                sink_handle: Handle::default(),
                command: None,
            },
            Name::new(format!("AudioSink_{}", i)),
        )).id();
        
        audio_manager.available_sinks.push_back(sink_entity);
    }
    
    info!("Audio system initialized with {} available sinks", audio_manager.available_sinks.len());
}

pub fn audio_event_system(
    mut commands: Commands,
    mut audio_events: EventReader<AudioPlayEvent>,
    mut audio_manager: ResMut<AudioManager>,
    mut sink_query: Query<&mut AudioSinkComponent>,
    audio_settings: Res<AudioSettings>,
    asset_server: Res<AssetServer>,
) {
    for event in audio_events.read() {
        // Check if we have available sinks
        let sink_entity = if let Some(entity) = audio_manager.available_sinks.pop_front() {
            entity
        } else {
            // No available sinks, steal the oldest one
            if let Some((&oldest_entity, _)) = audio_manager.in_use_sinks.iter().next() {
                audio_manager.in_use_sinks.remove(&oldest_entity);
                oldest_entity
            } else {
                warn!("No audio sinks available and none in use!");
                continue;
            }
        };
        
        // Get the actual audio source handle
        let audio_handle = match &event.source {
            crate::audio::resources::AudioSource::BirdVocalization(handle, _) => handle.clone(),
            crate::audio::resources::AudioSource::AmbientTrack(handle) => handle.clone(),
            crate::audio::resources::AudioSource::UiSound(handle) => handle.clone(),
        };
        
        // Update sink component
        if let Ok(mut sink_component) = sink_query.get_mut(sink_entity) {
            sink_component.command = Some(event.command.clone());
            
            // Play the audio based on command type
            match &event.command {
                AudioCommand::PlayGlobal => {
                    commands.entity(sink_entity).insert((
                        AudioPlayer::new(audio_handle),
                        PlaybackSettings::ONCE
                    ));
                }
                AudioCommand::PlayAt(position) => {
                    let (gain, panning) = calculate_positional_audio(*position, audio_manager.listener_position, 300.0);
                    commands.entity(sink_entity).insert((
                        AudioPlayer::new(audio_handle),
                        PlaybackSettings::ONCE
                    ));
                }
                AudioCommand::PlayFromEntity(_) => {
                    // Will be handled by update_positional_audio_system
                    commands.entity(sink_entity).insert((
                        AudioPlayer::new(audio_handle),
                        PlaybackSettings::ONCE
                    ));
                }
            }
            
            // Move to in-use collection
            audio_manager.in_use_sinks.insert(sink_entity, event.command.clone());
            
            debug!("Started playing audio on sink {:?}", sink_entity);
        }
    }
}

pub fn update_positional_audio_system(
    mut audio_manager: ResMut<AudioManager>,
    sink_query: Query<(Entity, &AudioSinkComponent)>,
    mut audio_query: Query<&mut PlaybackSettings>,
    transform_query: Query<&Transform, Without<AudioSinkComponent>>,
    camera_query: Query<&Transform, (With<Camera2d>, Without<AudioSinkComponent>)>,
    audio_settings: Res<AudioSettings>,
    time: Res<Time>,
) {
    // Update listener position to camera position
    if let Ok(camera_transform) = camera_query.single() {
        audio_manager.listener_position = camera_transform.translation.truncate();
    }
    
    // Update all positional audio sources
    for (sink_entity, sink_component) in sink_query.iter() {
        if let Some(command) = &sink_component.command {
            match command {
                AudioCommand::PlayFromEntity(entity) => {
                    if let Ok(transform) = transform_query.get(*entity) {
                        let position = transform.translation.truncate();
                        let (gain, _panning) = calculate_positional_audio(
                            position, 
                            audio_manager.listener_position, 
                            300.0
                        );
                        
                        // Update volume based on distance
                        if let Ok(mut playback_settings) = audio_query.get_mut(sink_entity) {
                            // Volume control disabled for compatibility with Bevy 0.16.1
                        }
                    }
                }
                AudioCommand::PlayAt(position) => {
                    let (gain, _panning) = calculate_positional_audio(*position, audio_manager.listener_position, 300.0);
                    
                    if let Ok(mut playback_settings) = audio_query.get_mut(sink_entity) {
                        // Volume control disabled for compatibility with Bevy 0.16.1
                    }
                }
                AudioCommand::PlayGlobal => {
                    // No positional updates needed for global sounds
                }
            }
        }
    }
}

pub fn audio_cleanup_system(
    mut commands: Commands,
    mut audio_manager: ResMut<AudioManager>,
    sink_query: Query<(Entity, &AudioSinkComponent, Option<&AudioPlayer>)>,
) {
    let mut completed_sinks = Vec::new();
    
    for (sink_entity, _sink_component, audio_player) in sink_query.iter() {
        // Check if audio has finished playing
        let is_finished = audio_player.map(|player| {
            // In a real implementation, we'd check if the AudioPlayer has finished
            // For now, we'll use a simple heuristic or timer
            false // Placeholder - would need proper sink state checking
        }).unwrap_or(true);
        
        if is_finished && audio_manager.in_use_sinks.contains_key(&sink_entity) {
            completed_sinks.push(sink_entity);
        }
    }
    
    // Clean up completed sinks
    for sink_entity in completed_sinks {
        audio_manager.in_use_sinks.remove(&sink_entity);
        audio_manager.available_sinks.push_back(sink_entity);
        
        // Remove audio components to stop playback
        if let Ok(mut entity_commands) = commands.get_entity(sink_entity) {
            entity_commands.remove::<AudioPlayer>();
            entity_commands.remove::<PlaybackSettings>();
        }
        
        debug!("Cleaned up completed audio sink {:?}", sink_entity);
    }
}

pub fn bird_vocalization_system(
    mut commands: Commands,
    bird_query: Query<(Entity, &BirdState, &AnimatedBird), (With<BirdAI>, Changed<BirdState>)>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    for (entity, bird_state, animated_bird) in bird_query.iter() {
        // Birds vocalize when they change to certain states
        let should_vocalize = matches!(bird_state, 
            BirdState::MovingToTarget | BirdState::Eating | BirdState::Fleeing
        );
        
        if should_vocalize {
            // Load appropriate sound for species
            let sound_path = match animated_bird.species {
                BirdSpecies::Cardinal => "audio/cardinal_call.ogg",
                BirdSpecies::BlueJay => "audio/bluejay_call.ogg",
                BirdSpecies::Sparrow => "audio/sparrow_chirp.ogg",
            };
            
            let audio_handle = asset_server.load(sound_path);
            
            commands.trigger(AudioPlayEvent {
                source: crate::audio::resources::AudioSource::BirdVocalization(audio_handle, animated_bird.species),
                command: AudioCommand::PlayFromEntity(entity),
            });
            
            debug!("Bird {:?} vocalizing in state {:?}", animated_bird.species, bird_state);
        }
    }
}

// Helper function for positional audio calculations
fn calculate_positional_audio(source_pos: Vec2, listener_pos: Vec2, attenuation: f32) -> (f32, f32) {
    let distance = source_pos.distance(listener_pos);
    let gain = (1.0 - (distance / attenuation)).max(0.0).min(1.0);
    
    // Simple stereo panning based on left/right position
    let relative_pos = source_pos - listener_pos;
    let panning = (relative_pos.x / attenuation).clamp(-1.0, 1.0);
    
    (gain, panning)
}
