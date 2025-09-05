// Reusable UI Widgets
use bevy::prelude::*;

// Common UI Colors
pub struct UIColors;

impl UIColors {
    pub const BACKGROUND: Color = Color::srgb(0.9, 0.9, 0.9);
    pub const BACKGROUND_HOVER: Color = Color::srgb(0.8, 0.8, 0.8);
    pub const BACKGROUND_PRESSED: Color = Color::srgb(0.7, 0.7, 0.7);
    pub const TEXT_PRIMARY: Color = Color::srgb(0.3, 0.2, 0.1);
    pub const TEXT_SECONDARY: Color = Color::srgb(0.5, 0.3, 0.2);
    pub const ACCENT_ON: Color = Color::srgb(0.2, 0.6, 0.2);
    pub const ACCENT_OFF: Color = Color::srgb(0.6, 0.2, 0.2);
}

#[derive(Component)]
pub struct SliderWidget {
    pub min_value: f32,
    pub max_value: f32,
    pub current_value: f32,
    pub step: f32,
    pub show_percentage: bool,
    pub is_dragging: bool, 
}

#[derive(Component)]
pub struct SliderTrack;

#[derive(Component)]
pub struct SliderHandle;

#[derive(Component)]
pub struct SliderValueText;

#[derive(Event)]
pub struct SliderValueChanged {
    pub slider_entity: Entity,
    pub old_value: f32,
    pub new_value: f32,
    pub percentage: u32,
}

impl SliderWidget {
    pub fn new(min: f32, max: f32, initial: f32) -> Self {
        Self {
            min_value: min,
            max_value: max,
            current_value: initial.clamp(min, max),
            step: 0.01,
            show_percentage: true,
            is_dragging: false,
        }
    }
    
    pub fn with_step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }
    
    pub fn without_percentage(mut self) -> Self {
        self.show_percentage = false;
        self
    }
    
    pub fn get_percentage(&self) -> u32 {
        ((self.current_value - self.min_value) / (self.max_value - self.min_value) * 100.0).round() as u32
    }
    
    pub fn set_from_percentage(&mut self, percentage: u32) {
        let normalized = (percentage as f32 / 100.0).clamp(0.0, 1.0);
        self.current_value = self.min_value + normalized * (self.max_value - self.min_value);
    }
}



// Working drag slider system with proper coordinate conversion
pub fn slider_interaction_system(
    mut track_query: Query<(Entity, &Interaction, &GlobalTransform, &Node), (With<SliderTrack>, Without<SliderHandle>)>,
    mut slider_query: Query<&mut SliderWidget>,
    mut handle_query: Query<&mut Node, (With<SliderHandle>, Without<SliderTrack>)>, 
    mut text_query: Query<&mut Text, With<SliderValueText>>,
    mut slider_events: EventWriter<SliderValueChanged>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut cursor_moved: EventReader<CursorMoved>,
    windows: Query<&Window>,
) {
    let cursor_position = cursor_moved.read().last().map(|e| e.position);
    
    // Mouse drag handling
    if mouse_button.pressed(MouseButton::Left) {
        if let Some(cursor_pos) = cursor_position {
            for (track_entity, interaction, track_transform, track_node) in &track_query {
                if matches!(interaction, Interaction::Pressed | Interaction::Hovered) {
                    // Get window for coordinate conversion
                    let Ok(window) = windows.single() else { continue };
                    
                    // Convert UI coordinates properly
                    let track_world_pos = track_transform.translation().truncate();
                    let track_size = Vec2::new(200.0, 24.0); // Simplified - use fixed size for now
                    
                    // Convert cursor position from screen to UI coordinates
                    let window_size = Vec2::new(window.width(), window.height());
                    let ui_cursor = Vec2::new(
                        cursor_pos.x - window_size.x / 2.0,
                        window_size.y / 2.0 - cursor_pos.y
                    );
                    
                    // Calculate relative position on track
                    let track_left = track_world_pos.x - track_size.x / 2.0;
                    let relative_x = (ui_cursor.x - track_left) / track_size.x;
                    let clamped_x = relative_x.clamp(0.0, 1.0);
                    
                    // Update slider value
                    for mut slider in &mut slider_query {
                        let old_value = slider.current_value;
                        let new_value = slider.min_value + clamped_x * (slider.max_value - slider.min_value);
                        
                        if (new_value - old_value).abs() > 0.001 {
                            slider.current_value = new_value;
                            
                            // Update handle position
                            for mut handle_node in &mut handle_query {
                                handle_node.left = Val::Percent(clamped_x * 100.0);
                            }
                            
                            // Update text
                            for mut text in &mut text_query {
                                **text = format!("{}%", slider.get_percentage());
                            }
                            
                            // Send event
                            slider_events.write(SliderValueChanged {
                                slider_entity: track_entity,
                                old_value,
                                new_value: slider.current_value,
                                percentage: slider.get_percentage(),
                            });
                        }
                        break;
                    }
                }
            }
        }
    }
    
    // Keyboard controls
    if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::ArrowRight) || 
       keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::ArrowDown) {
        
        let increment = if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
            0.01 // 1% with Shift
        } else {
            0.05 // 5% normal
        };
        
        let direction = if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::ArrowUp) {
            1.0
        } else {
            -1.0
        };
        
        for mut slider in &mut slider_query {
            let old_value = slider.current_value;
            let value_range = slider.max_value - slider.min_value;
            let new_value = (slider.current_value + direction * increment * value_range)
                .clamp(slider.min_value, slider.max_value);
            
            if (new_value - old_value).abs() > 0.001 {
                slider.current_value = new_value;
                
                let handle_percentage = (slider.current_value - slider.min_value) / (slider.max_value - slider.min_value);
                for mut handle_node in &mut handle_query {
                    handle_node.left = Val::Percent(handle_percentage * 100.0);
                }
                
                for mut text in &mut text_query {
                    **text = format!("{}%", slider.get_percentage());
                }
                
                slider_events.write(SliderValueChanged {
                    slider_entity: Entity::PLACEHOLDER,
                    old_value,
                    new_value: slider.current_value,
                    percentage: slider.get_percentage(),
                });
            }
        }
    }
}

// Cursor position resource for drag support
#[derive(Resource, Default)]
pub struct CursorPosition(pub Option<Vec2>);

pub fn update_cursor_position(
    mut cursor_position: ResMut<CursorPosition>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    for event in cursor_moved_events.read() {
        cursor_position.0 = Some(event.position);
    }
}

// ========== DROPDOWN WIDGET ==========

#[derive(Component)]
pub struct DropdownWidget<T: Send + Sync + Clone + 'static> {
    pub options: Vec<T>,
    pub selected_index: usize,
    pub is_open: bool,
}

impl<T: Send + Sync + Clone + 'static> DropdownWidget<T> {
    pub fn new(options: Vec<T>, selected_index: usize) -> Self {
        let max_index = options.len().saturating_sub(1);
        Self {
            options,
            selected_index: selected_index.clamp(0, max_index),
            is_open: false,
        }
    }
    
    pub fn selected(&self) -> Option<&T> {
        self.options.get(self.selected_index)
    }
    
    pub fn select(&mut self, index: usize) {
        if index < self.options.len() {
            self.selected_index = index;
        }
    }
    
    pub fn toggle_open(&mut self) {
        self.is_open = !self.is_open;
    }
}

#[derive(Component)]
pub struct DropdownButton;

#[derive(Component)]
pub struct DropdownOptions {
    pub dropdown_entity: Entity,
}

#[derive(Component)]
pub struct DropdownOption {
    pub dropdown_entity: Entity,
    pub option_index: usize,
}

#[derive(Event)]
pub struct DropdownValueChanged<T: Send + Sync + Clone + 'static> {
    pub dropdown_entity: Entity,
    pub selected_index: usize,
    pub selected_value: T,
}

// ========== TOGGLE BUTTON WIDGET ==========

#[derive(Component)]
pub struct ToggleButton {
    pub is_on: bool,
    pub label: String,
}

impl ToggleButton {
    pub fn new(label: impl Into<String>, initial_state: bool) -> Self {
        Self {
            is_on: initial_state,
            label: label.into(),
        }
    }
    
    pub fn toggle(&mut self) {
        self.is_on = !self.is_on;
    }
}

#[derive(Event)]
pub struct ToggleButtonChanged {
    pub entity: Entity,
    pub is_on: bool,
}

// ========== SYSTEMS ==========

// Simplified dropdown system - we'll store dropdown entity directly in the button component
#[derive(Component)]
pub struct DropdownButtonRef {
    pub dropdown_entity: Entity,
}

pub fn dropdown_button_system(
    mut interaction_query: Query<(Entity, &Interaction, &DropdownButtonRef), (Changed<Interaction>, With<DropdownButton>)>,
    mut dropdown_query: Query<&mut DropdownWidget<String>>,
) {
    for (button_entity, interaction, dropdown_ref) in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            if let Ok(mut dropdown) = dropdown_query.get_mut(dropdown_ref.dropdown_entity) {
                dropdown.toggle_open();
            }
        }
    }
}

pub fn dropdown_option_system(
    mut interaction_query: Query<(Entity, &Interaction, &DropdownOption), (Changed<Interaction>, With<Button>)>,
    mut dropdown_query: Query<&mut DropdownWidget<String>>,
    mut dropdown_events: EventWriter<DropdownValueChanged<String>>,
    mut text_query: Query<&mut Text>,
    children_query: Query<&Children>,
) {
    for (option_entity, interaction, dropdown_option) in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            if let Ok(mut dropdown) = dropdown_query.get_mut(dropdown_option.dropdown_entity) {
                let old_index = dropdown.selected_index;
                dropdown.select(dropdown_option.option_index);
                dropdown.is_open = false; // Close dropdown after selection
                
                if old_index != dropdown.selected_index {
                    if let Some(selected_value) = dropdown.selected().cloned() {
                        // Update dropdown button text
                        if let Ok(dropdown_children) = children_query.get(dropdown_option.dropdown_entity) {
                            for dropdown_child in dropdown_children.iter() {
                                if let Ok(button_children) = children_query.get(dropdown_child) {
                                    for button_child in button_children.iter() {
                                        if let Ok(mut text) = text_query.get_mut(button_child) {
                                            if !text.contains("▼") { // Update the main text, not the arrow
                                                **text = selected_value.clone();
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        
                        dropdown_events.write(DropdownValueChanged {
                            dropdown_entity: dropdown_option.dropdown_entity,
                            selected_index: dropdown.selected_index,
                            selected_value,
                        });
                    }
                }
            }
        }
    }
}

pub fn toggle_button_system(
    mut interaction_query: Query<(Entity, &Interaction, &mut ToggleButton), (Changed<Interaction>, With<Button>)>,
    mut toggle_events: EventWriter<ToggleButtonChanged>,
) {
    for (entity, interaction, mut toggle) in &mut interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            toggle.toggle();
            toggle_events.write(ToggleButtonChanged {
                entity,
                is_on: toggle.is_on,
            });
        }
    }
}

pub fn dropdown_visibility_system(
    dropdown_query: Query<(Entity, &DropdownWidget<String>, &Children), Changed<DropdownWidget<String>>>,
    mut node_query: Query<&mut Node>,
    options_query: Query<&DropdownOptions>,
) {
    for (dropdown_entity, dropdown, children) in &dropdown_query {
        // Find the dropdown options container
        for child in children.iter() {
            if options_query.contains(child) {
                if let Ok(mut node) = node_query.get_mut(child) {
                    node.display = if dropdown.is_open {
                        Display::Flex
                    } else {
                        Display::None
                    };
                }
                break;
            }
        }
    }
}

// Note: Builder functions removed due to Bevy version compatibility issues.
// Settings now use simplified click-to-cycle approach for dropdowns and direct button interactions for toggles.