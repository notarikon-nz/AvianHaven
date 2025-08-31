// src/user_interface/mod.rs
use bevy::prelude::*;

pub mod builder;
pub mod styles;
pub mod resources;

use styles::*;
use builder::*;
use resources::*;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ButtonStyle>()
            .init_resource::<PanelStyle>();
    }
}
