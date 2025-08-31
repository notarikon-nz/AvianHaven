use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

use components::*;
use resources::*;
use systems::*;

pub struct SteamPlugin;

impl Plugin for SteamPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SteamState>()
    .init_resource::<SteamStats>()
            .init_resource::<SteamAchievements>()
            .add_event::<SteamAchievementEvent>()
            .add_event::<SteamStatsEvent>()
            .add_systems(Startup, initialize_steam_systems)
            .add_systems(Update, (
                steam_achievement_sync_system,
                steam_stats_tracking_system,
                steam_workshop_system,
            ).run_if(resource_exists::<SteamState>));
    }
}