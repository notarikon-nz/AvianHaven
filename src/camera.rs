use bevy::prelude::*;

use crate::AppState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CameraState>()
            .add_systems(
                Update,
                (
                    camera_pan_system,
                    // ensure_camera_viewport,
                ).run_if(in_state(AppState::Playing))
            );
    }
}

#[derive(Resource, Default)]
struct CameraState {
    is_dragging: bool,
    last_mouse_position: Vec2,
}

fn camera_pan_system(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut cursor_events: EventReader<CursorMoved>,
    mut camera_state: ResMut<CameraState>,
    window: Query<&Window>,
) {
    let Ok(window) = window.single() else { return };
    let Ok(mut camera_transform) = camera_query.single_mut() else { return };

    // Handle mouse button presses
    if mouse_button.just_pressed(MouseButton::Left) {
        if let Some(cursor_position) = window.cursor_position() {
            camera_state.is_dragging = true;
            camera_state.last_mouse_position = cursor_position;
        }
    }

    if mouse_button.just_released(MouseButton::Left) {
        camera_state.is_dragging = false;
    }

    // Handle dragging
    if camera_state.is_dragging {
        for event in cursor_events.read() {
            let delta = event.position - camera_state.last_mouse_position;
            
            // Invert delta for natural camera movement
            camera_transform.translation.x -= delta.x;
            camera_transform.translation.y += delta.y; // Y is flipped in screen coordinates
            
            camera_state.last_mouse_position = event.position;
        }
    }
}