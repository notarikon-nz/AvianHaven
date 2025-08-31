use bevy::prelude::*;

pub mod components;
pub mod systems;

use components::*;
use systems::*;

pub struct WeatherEffectsPlugin;

impl Plugin for WeatherEffectsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WeatherEffectsState>()
            .add_systems(Startup, setup_weather_particles)
            .add_systems(Update, (
                weather_particle_system,
                rain_particle_movement,
                snow_particle_movement,
                environmental_particle_system,
                environmental_particle_movement,
                interactive_particle_system,
                interactive_particle_movement,
                particle_cleanup_system,
            ).run_if(in_state(crate::AppState::Playing)));
    }
}