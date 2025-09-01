use bevy::prelude::*;

#[derive(Component)]
pub struct SaveMarker;

#[derive(Component)]
pub struct PersistentObject {
    pub save_id: String,
}