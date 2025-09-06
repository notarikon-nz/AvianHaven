// src/user_interface/mod.rs
use bevy::prelude::*;

pub mod builder;
pub mod styles;
pub mod resources;
pub mod slider;
pub mod dropdown;
pub mod toggle;
pub mod scrollable;
pub mod scrollable_systems;
pub mod tab_group;

use styles::*;
use slider::SliderPlugin;
use dropdown::DropdownPlugin;
use toggle::TogglePlugin;
use scrollable_systems::*;
use tab_group::TabPlugin;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ButtonStyle>()
            .init_resource::<PanelStyle>()
            .init_resource::<CursorPosition>()
            .add_event::<scrollable::ScrollEvent>()
            .add_plugins((SliderPlugin, DropdownPlugin, TogglePlugin, TabPlugin))
            .add_systems(Update, (
                cursor_position_system,
                mouse_wheel_scroll_system,
                scrollbar_drag_system,
                update_scrollbar_system,
            ));
    }
}
