use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

// use components::*;
use resources::*;
use systems::*;
//use crate::AppState;

pub struct SaveLoadPlugin;

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SaveManager>()
            .init_resource::<PlaytimeTracker>()
            .add_event::<SaveGameEvent>()
            .add_event::<LoadGameEvent>()
            .add_event::<SaveCompleteEvent>()
            .add_event::<LoadCompleteEvent>()
            .add_systems(Update, (
                save_game_system,
                load_game_system,
                auto_save_system,
                track_playtime_system,
            ));
    }
}