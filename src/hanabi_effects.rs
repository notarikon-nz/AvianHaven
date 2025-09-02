// Bevy Hanabi GPU-Accelerated Particle Effects - Phase 4
// Note: This is a framework setup for Hanabi integration
// Currently using placeholder system until full migration is completed
use bevy::prelude::*;
use crate::environment::components::{Weather, Season};
use crate::bird::BirdSpecies;

pub struct HanabiEffectsPlugin;

impl Plugin for HanabiEffectsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<HanabiState>()
            .add_systems(Startup, setup_hanabi_placeholder)
            .add_systems(Update, (
                hanabi_weather_system,
                hanabi_interactive_system,
            ).run_if(in_state(crate::AppState::Playing)));
    }
}

#[derive(Resource, Default)]
pub struct HanabiState {
    pub migration_complete: bool,
    pub effects_active: bool,
}

fn setup_hanabi_placeholder(
    mut hanabi_state: ResMut<HanabiState>,
) {
    info!("Setting up Hanabi placeholder system");
    info!("Note: Full Hanabi integration requires compatible versions");
    info!("Current system provides foundation for GPU particle effects");
    
    hanabi_state.effects_active = true;
    hanabi_state.migration_complete = false;
}

fn hanabi_weather_system(
    weather_state: Res<crate::environment::resources::WeatherState>,
    hanabi_state: Res<HanabiState>,
) {
    if !hanabi_state.effects_active {
        return;
    }

    // Placeholder for weather-based Hanabi particle effects
    match weather_state.current_weather {
        Weather::Rainy => {
            // Future: Spawn GPU-accelerated rain particles
            debug!("Would spawn Hanabi rain effects");
        },
        Weather::Snowy => {
            // Future: Spawn GPU-accelerated snow particles
            debug!("Would spawn Hanabi snow effects");
        },
        Weather::Windy => {
            // Future: Spawn windy atmospheric particles
            debug!("Would spawn Hanabi wind effects");
        },
        _ => {
            // Future: Spawn ambient atmospheric particles
            debug!("Would spawn Hanabi atmospheric effects");
        },
    }
}

fn hanabi_interactive_system(
    bird_query: Query<(&Transform, &crate::bird_ai::components::BirdState), With<crate::bird::Bird>>,
    hanabi_state: Res<HanabiState>,
    time: Res<Time>,
) {
    if !hanabi_state.effects_active {
        return;
    }

    for (bird_transform, bird_state) in &bird_query {
        match bird_state {
            crate::bird_ai::components::BirdState::Bathing => {
                // Future: Trigger GPU water splash effects
                if rand::random::<f32>() < 0.1 * time.delta().as_secs_f32() {
                    debug!("Would spawn Hanabi splash effect at {:?}", bird_transform.translation);
                }
            },
            crate::bird_ai::components::BirdState::Eating => {
                // Future: Trigger GPU seed scatter effects
                if rand::random::<f32>() < 0.05 * time.delta().as_secs_f32() {
                    debug!("Would spawn Hanabi seed scatter effect at {:?}", bird_transform.translation);
                }
            },
            _ => {}
        }
    }
}