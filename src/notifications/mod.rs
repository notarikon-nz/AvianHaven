use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

use resources::*;
use systems::*;

pub struct NotificationPlugin;

impl Plugin for NotificationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<NotificationQueue>()
            .add_event::<ShowNotificationEvent>()
            .add_systems(Startup, setup_notification_container)
            .add_systems(Update, (
                notification_spawner_system,
                notification_animation_system,
                notification_cleanup_system,
            ));
    }
}