use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

use resources::*;
use systems::*;
use crate::{AppState};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TextureAtlasCache>()
            .add_systems(Startup, setup_animation_assets)
            .add_systems(FixedUpdate, (
                animation_state_system,
                update_sprite_on_state_change_system,
                advance_animation_frames_system,
            ).chain().run_if(in_state(AppState::Playing)))
            .add_systems(Update, sprite_flip_system.run_if(in_state(AppState::Playing)));
    }
}