// src/audio/systems.rs
use bevy::prelude::*;
use bevy::audio::PlaybackSettings;
use crate::audio::{components::*, resources::*};
use crate::bird_ai::components::{BirdAI, BirdState};
use crate::animation::components::AnimatedBird;
use crate::bird::BirdSpecies;
use crate::feeder::Feeder;

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
                        PlaybackSettings::ONCE.with_spatial(true)
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
    camera_query: Query<&Transform, (With<Camera2d>, Without<BirdAI>)>,
    bird_query: Query<(Entity, &Transform, &BirdState, &AnimatedBird), (With<BirdAI>, Changed<BirdState>)>,
    all_birds_query: Query<(&Transform, &AnimatedBird), With<BirdAI>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    let Ok(camera_transform) = camera_query.get_single() else {
        return;
    };
    let camera_pos = camera_transform.translation.truncate();
    
    for (entity, transform, bird_state, animated_bird) in &bird_query {
        let bird_pos = transform.translation.truncate();
        let distance = camera_pos.distance(bird_pos);
        
        // Only play sounds for birds within audio range
        if distance > 400.0 {
            continue;
        }
        
        // Enhanced vocalization triggers based on behavior and social context
        let (should_vocalize, call_type) = determine_vocalization(bird_state, animated_bird.species, &all_birds_query, bird_pos);
        
        if should_vocalize {
            let sound_path = get_species_sound_path(animated_bird.species, call_type);
            let audio_handle = asset_server.load(sound_path);
            
            // Calculate positional audio with species-specific range
            let max_range = get_species_audio_range(animated_bird.species);
            let (gain, _panning) = calculate_positional_audio(bird_pos, camera_pos, max_range);
            
            // Species-specific volume adjustment
            let volume_modifier = get_species_volume(animated_bird.species);
            
            commands.spawn((
                AudioPlayer::new(audio_handle),
                PlaybackSettings::ONCE
                    // Volume disabled for Bevy 0.16.1 compatibility
                    .with_spatial(true),
                Transform::from_translation(transform.translation),
                PositionalAudioSource {
                    source_entity: entity,
                    max_distance: max_range,
                    volume_curve: AudioVolumeCurve::InverseSquare,
                },
            ));
            
            debug!("Bird {:?} {:?} vocalizing at distance {:.1} with gain {:.2}", 
                   animated_bird.species, call_type, distance, gain);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CallType {
    Song,        // Territorial or mating call
    Alarm,       // Warning call
    Contact,     // Social communication
    Feeding,     // Feeding excitement
    Territorial, // Aggressive display
}

fn determine_vocalization(
    bird_state: &BirdState, 
    species: BirdSpecies, 
    all_birds: &Query<(&Transform, &AnimatedBird), With<BirdAI>>,
    bird_pos: Vec2
) -> (bool, CallType) {
    match bird_state {
        BirdState::Eating => {
            // Check for nearby birds of same species to trigger contact calls
            let nearby_same_species = all_birds.iter()
                .filter(|(transform, bird)| {
                    bird.species == species && 
                    bird_pos.distance(transform.translation.truncate()) < 100.0
                })
                .count();
            
            if nearby_same_species > 1 {
                (true, CallType::Contact) // Social feeding calls
            } else {
                (rand::random::<f32>() < 0.3, CallType::Feeding) // Occasional feeding sounds
            }
        },
        BirdState::MovingToTarget => {
            // Song birds often call while moving
            match species {
                BirdSpecies::Robin | BirdSpecies::Cardinal | BirdSpecies::BaltimoreOriole => {
                    (rand::random::<f32>() < 0.2, CallType::Song)
                },
                _ => (false, CallType::Contact)
            }
        },
        BirdState::Fleeing => {
            // Alarm calls when fleeing
            (rand::random::<f32>() < 0.8, CallType::Alarm)
        },
        BirdState::Resting => {
            // Territorial calls during rest periods
            if species.aggression_level() > 0.5 {
                (rand::random::<f32>() < 0.1, CallType::Territorial)
            } else {
                (rand::random::<f32>() < 0.05, CallType::Song)
            }
        },
        _ => (false, CallType::Contact)
    }
}

fn get_species_sound_path(species: BirdSpecies, call_type: CallType) -> &'static str {
    match (species, call_type) {
        // Cardinals - distinctive calls
        (BirdSpecies::Cardinal, CallType::Song) => "audio/cardinal_song.ogg",
        (BirdSpecies::Cardinal, CallType::Alarm) => "audio/cardinal_alarm.ogg",
        (BirdSpecies::Cardinal, _) => "audio/cardinal_call.ogg",
        
        // Blue Jays - loud and varied
        (BirdSpecies::BlueJay, CallType::Alarm) => "audio/bluejay_scream.ogg",
        (BirdSpecies::BlueJay, CallType::Territorial) => "audio/bluejay_aggressive.ogg",
        (BirdSpecies::BlueJay, _) => "audio/bluejay_call.ogg",
        
        // Robins - melodic songs
        (BirdSpecies::Robin, CallType::Song) => "audio/robin_song.ogg",
        (BirdSpecies::Robin, CallType::Alarm) => "audio/robin_alarm.ogg",
        (BirdSpecies::Robin, _) => "audio/robin_chirp.ogg",
        
        // Chickadees - social calls
        (BirdSpecies::Chickadee, CallType::Contact) => "audio/chickadee_social.ogg",
        (BirdSpecies::Chickadee, CallType::Alarm) => "audio/chickadee_alarm.ogg",
        (BirdSpecies::Chickadee, _) => "audio/chickadee_call.ogg",
        
        // Woodpeckers - drumming and calls
        (BirdSpecies::DownyWoodpecker, CallType::Territorial) => "audio/downy_drum.ogg",
        (BirdSpecies::HairyWoodpecker, CallType::Territorial) => "audio/hairy_drum.ogg",
        (BirdSpecies::PileatedWoodpecker, CallType::Territorial) => "audio/pileated_drum.ogg",
        (BirdSpecies::RedHeadedWoodpecker, CallType::Territorial) => "audio/redheaded_drum.ogg",
        (woodpecker, _) if matches!(woodpecker, 
            BirdSpecies::DownyWoodpecker | BirdSpecies::HairyWoodpecker | 
            BirdSpecies::PileatedWoodpecker | BirdSpecies::RedHeadedWoodpecker |
            BirdSpecies::YellowBelledSapsucker
        ) => "audio/woodpecker_call.ogg",
        
        // Raptors - distinctive screeches
        (BirdSpecies::RedTailedHawk, _) => "audio/hawk_screech.ogg",
        (BirdSpecies::CoopersHawk, _) => "audio/cooper_call.ogg",
        (BirdSpecies::PeregrineFalcon, _) => "audio/falcon_cry.ogg",
        (BirdSpecies::BaldEagle, _) => "audio/eagle_call.ogg",
        
        // Owls - hoots and calls
        (BirdSpecies::GreatHornedOwl, _) => "audio/owl_hoot.ogg",
        (BirdSpecies::BarredOwl, _) => "audio/barred_owl.ogg",
        
        // Hummingbirds - buzzing
        (BirdSpecies::RubyThroatedHummingbird, _) => "audio/hummingbird_buzz.ogg",
        
        // Other songbirds
        (BirdSpecies::BaltimoreOriole, CallType::Song) => "audio/oriole_song.ogg",
        (BirdSpecies::ScarletTanager, CallType::Song) => "audio/tanager_song.ogg",
        (BirdSpecies::EasternBluebird, CallType::Song) => "audio/bluebird_warble.ogg",
        
        // Generic fallbacks based on rarity
        (species, _) => match species.rarity_tier() {
            1 => "audio/common_chirp.ogg",
            2 => "audio/uncommon_song.ogg",
            3 => "audio/rare_call.ogg", 
            4 => "audio/legendary_song.ogg",
            _ => "audio/default_chirp.ogg",
        }
    }
}

fn get_species_audio_range(species: BirdSpecies) -> f32 {
    match species {
        // Large raptors - calls carry very far
        BirdSpecies::BaldEagle | BirdSpecies::RedTailedHawk => 800.0,
        BirdSpecies::GreatHornedOwl | BirdSpecies::BarredOwl => 600.0,
        BirdSpecies::CoopersHawk | BirdSpecies::PeregrineFalcon => 500.0,
        
        // Loud species
        BirdSpecies::BlueJay | BirdSpecies::CommonCrow | BirdSpecies::NorthernMockingbird => 450.0,
        BirdSpecies::PileatedWoodpecker => 400.0,
        
        // Medium-range species
        BirdSpecies::Cardinal | BirdSpecies::Robin | BirdSpecies::BaltimoreOriole => 300.0,
        BirdSpecies::ScarletTanager | BirdSpecies::WoodThrush => 250.0,
        
        // Quieter species
        BirdSpecies::Chickadee | BirdSpecies::TuftedTitmouse | BirdSpecies::WhiteBreastedNuthatch => 200.0,
        BirdSpecies::Goldfinch | BirdSpecies::HouseFinch | BirdSpecies::PurpleFinch => 150.0,
        
        // Very quiet species
        BirdSpecies::WinterWren | BirdSpecies::BrownCreeper => 100.0,
        BirdSpecies::RubyThroatedHummingbird => 80.0,
        
        _ => 300.0, // Default range
    }
}

fn get_species_volume(species: BirdSpecies) -> f32 {
    match species {
        // Very loud species
        BirdSpecies::BaldEagle | BirdSpecies::RedTailedHawk => 0.8,
        BirdSpecies::BlueJay | BirdSpecies::CommonCrow | BirdSpecies::GreatHornedOwl => 0.7,
        BirdSpecies::PileatedWoodpecker | BirdSpecies::NorthernMockingbird => 0.6,
        
        // Moderately loud
        BirdSpecies::Cardinal | BirdSpecies::Robin | BirdSpecies::BaltimoreOriole => 0.5,
        BirdSpecies::CoopersHawk | BirdSpecies::PeregrineFalcon => 0.5,
        
        // Quiet species
        BirdSpecies::Chickadee | BirdSpecies::Goldfinch | BirdSpecies::HouseFinch => 0.3,
        BirdSpecies::WinterWren | BirdSpecies::BrownCreeper => 0.2,
        BirdSpecies::RubyThroatedHummingbird => 0.15,
        
        _ => 0.4, // Default volume
    }
}

// Helper function for positional audio calculations
fn calculate_positional_audio(source_pos: Vec2, listener_pos: Vec2, attenuation: f32) -> (f32, f32) {
    let distance = source_pos.distance(listener_pos);
    let gain = (1.0 - (distance / attenuation)).max(0.0).min(1.0);
    
    // Enhanced stereo panning with distance consideration
    let relative_pos = source_pos - listener_pos;
    let panning = (relative_pos.x / attenuation).clamp(-1.0, 1.0);
    
    // Apply inverse square law for more realistic audio falloff
    let realistic_gain = 1.0 / (1.0 + distance * distance / (attenuation * attenuation));
    let final_gain = (gain * 0.3 + realistic_gain * 0.7).min(1.0);
    
    (final_gain, panning)
}

pub fn ambient_feeder_audio_system(
    mut commands: Commands,
    camera_query: Query<&Transform, (With<Camera2d>, Without<BirdAI>)>,
    feeder_query: Query<(&Transform, &Feeder)>,
    bird_query: Query<&Transform, With<BirdAI>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    let Ok(camera_transform) = camera_query.get_single() else {
        return;
    };
    let camera_pos = camera_transform.translation.truncate();
    
    for (feeder_transform, feeder) in &feeder_query {
        let feeder_pos = feeder_transform.translation.truncate();
        let distance = camera_pos.distance(feeder_pos);
        
        // Only create ambient sounds for feeders within range
        if distance > 300.0 {
            continue;
        }
        
        // Count nearby birds at this feeder
        let birds_near_feeder = bird_query.iter()
            .filter(|transform| {
                feeder_pos.distance(transform.translation.truncate()) < 100.0
            })
            .count();
        
        // Play ambient feeder sounds based on activity level
        if birds_near_feeder > 0 && time.elapsed_secs() % 8.0 < 0.1 {
            let sound_path = match birds_near_feeder {
                1..=2 => "audio/feeder_light_activity.ogg",
                3..=5 => "audio/feeder_moderate_activity.ogg",
                _ => "audio/feeder_busy_activity.ogg",
            };
            
            let audio_handle = asset_server.load(sound_path);
            let (_gain, _) = calculate_positional_audio(feeder_pos, camera_pos, 300.0);
            
            commands.spawn((
                AudioPlayer::new(audio_handle),
                PlaybackSettings::ONCE.with_spatial(true),
                Transform::from_translation(feeder_transform.translation),
                PositionalAudioSource {
                    source_entity: Entity::PLACEHOLDER,
                    max_distance: 300.0,
                    volume_curve: AudioVolumeCurve::Linear,
                },
            ));
        }
    }
}
