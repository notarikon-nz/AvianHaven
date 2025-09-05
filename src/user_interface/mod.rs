// src/user_interface/mod.rs
use bevy::prelude::*;

pub mod builder;
pub mod styles;
pub mod resources;
pub mod slider;
pub mod dropdown;

use styles::*;
use slider::SliderPlugin;
use dropdown::DropdownPlugin;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ButtonStyle>()
            .init_resource::<PanelStyle>()
            .add_plugins((SliderPlugin, DropdownPlugin));
    }
}
