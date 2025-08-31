use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

use components::*;
use resources::*;
use systems::*;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TimeState>()
            .init_resource::<WeatherState>()
            .init_resource::<SeasonalState>()
            .add_event::<WeatherChangeEvent>()
            .add_event::<TimeChangeEvent>()
            .add_systems(Startup, setup_environment)
            .add_systems(Update, (
                time_progression_system,
                weather_system,
                seasonal_migration_system,
                environment_effect_system,
            ).run_if(in_state(crate::AppState::Playing)));
    }
}