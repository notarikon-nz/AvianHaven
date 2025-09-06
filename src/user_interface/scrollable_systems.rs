use bevy::prelude::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use super::scrollable::*;

/// System for handling mouse wheel scrolling
pub fn mouse_wheel_scroll_system(
    mut scroll_events: EventReader<MouseWheel>,
    mut containers: Query<&mut ScrollableContainer>,
    mut content_query: Query<&mut Node, With<ScrollableContent>>,
    interaction_query: Query<&Interaction, With<ScrollableContainer>>,
) {
    for scroll_event in scroll_events.read() {
        // Check if mouse is over a scrollable container
        for mut container in containers.iter_mut() {
            if !container.enabled {
                continue;
            }

            let scroll_delta = match scroll_event.unit {
                MouseScrollUnit::Line => scroll_event.y * container.scroll_speed,
                MouseScrollUnit::Pixel => scroll_event.y,
            };

            // Calculate new scroll position
            let max_scroll = (container.content_height - container.viewport_height).max(0.0);
            let current_scroll_px = container.scroll_position * max_scroll;
            let new_scroll_px = (current_scroll_px - scroll_delta).clamp(0.0, max_scroll);
            container.scroll_position = if max_scroll > 0.0 {
                new_scroll_px / max_scroll
            } else {
                0.0
            };

            // Update content position
            if let Ok(mut content_node) = content_query.single_mut() {
                content_node.top = Val::Px(-new_scroll_px);
            }
        }
    }
}

/// System for handling scrollbar thumb dragging
pub fn scrollbar_drag_system(
    mut thumb_query: Query<(&mut ScrollbarThumb, &Interaction, &GlobalTransform), Changed<Interaction>>,
    mut containers: Query<&mut ScrollableContainer>,
    mut content_query: Query<&mut Node, With<ScrollableContent>>,
    cursor_position: Res<CursorPosition>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    // Handle thumb interaction start/end
    for (mut thumb, interaction, _transform) in thumb_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                thumb.dragging = true;
                if let Some(cursor_pos) = cursor_position.0 {
                    thumb.drag_start_y = cursor_pos.y;
                    if let Ok(container) = containers.single() {
                        thumb.drag_start_scroll = container.scroll_position;
                    }
                }
            }
            Interaction::None => {
                if !mouse_input.pressed(MouseButton::Left) {
                    thumb.dragging = false;
                }
            }
            _ => {}
        }
    }

    // Handle dragging
    if let Some(cursor_pos) = cursor_position.0 {
        for (mut thumb, _interaction, _transform) in thumb_query.iter_mut() {
            if thumb.dragging && mouse_input.pressed(MouseButton::Left) {
                if let Ok(mut container) = containers.single_mut() {
                    let drag_delta = cursor_pos.y - thumb.drag_start_y;
                    let scroll_range = container.viewport_height - 40.0; // Subtract thumb height
                    let scroll_delta = drag_delta / scroll_range;
                    
                    container.scroll_position = (thumb.drag_start_scroll + scroll_delta).clamp(0.0, 1.0);

                    // Update content position
                    let max_scroll = (container.content_height - container.viewport_height).max(0.0);
                    let scroll_px = container.scroll_position * max_scroll;
                    
                    if let Ok(mut content_node) = content_query.single_mut() {
                        content_node.top = Val::Px(-scroll_px);
                    }
                }
            }
        }
    }
}

/// System for updating scrollbar thumb position and size
pub fn update_scrollbar_system(
    containers: Query<&ScrollableContainer, Changed<ScrollableContainer>>,
    mut thumb_query: Query<&mut Node, With<ScrollbarThumb>>,
) {
    for container in containers.iter() {
        if let Ok(mut thumb_node) = thumb_query.single_mut() {
            // Calculate thumb size based on content ratio
            let visible_ratio = if container.content_height > 0.0 {
                (container.viewport_height / container.content_height).min(1.0)
            } else {
                1.0
            };
            
            let thumb_height = (container.viewport_height * visible_ratio).max(20.0);
            thumb_node.height = Val::Px(thumb_height);

            // Calculate thumb position
            let track_height = container.viewport_height - 4.0; // Account for padding
            let thumb_travel = track_height - thumb_height;
            let thumb_pos = container.scroll_position * thumb_travel + 2.0; // Add padding
            thumb_node.top = Val::Px(thumb_pos);
        }
    }
}

/// Resource for tracking cursor position
#[derive(Resource, Default)]
pub struct CursorPosition(pub Option<Vec2>);

/// System for tracking cursor position
pub fn cursor_position_system(
    mut cursor_position: ResMut<CursorPosition>,
    mut cursor_events: EventReader<CursorMoved>,
) {
    for event in cursor_events.read() {
        cursor_position.0 = Some(event.position);
    }
}