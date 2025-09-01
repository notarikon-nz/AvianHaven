use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

use components::*;
use resources::*;
use systems::*;
use crate::AppState;

pub struct TutorialPlugin;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TutorialState>()
            .insert_resource(TutorialProgress::load_from_file())
            .add_event::<TutorialEvent>()
            .add_event::<TutorialStepCompleteEvent>()
            .add_systems(OnEnter(AppState::Playing), check_tutorial_needed)
            .add_systems(Update, (
                tutorial_step_system,
                tutorial_input_handler,
                tutorial_ui_update_system,
                tutorial_highlight_system,
                tutorial_completion_system,
                tutorial_button_system,
            ).run_if(in_state(AppState::Playing)));
    }
}