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

// Simplified system to handle slider interactions via button clicks
pub fn slider_interaction_system(
    mut interaction_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<SliderTrack>)>,
    mut slider_query: Query<&mut SliderWidget>,
    mut handle_query: Query<&mut Node, With<SliderHandle>>,
    mut text_query: Query<&mut Text, With<SliderValueText>>,
    mut slider_events: EventWriter<SliderValueChanged>,
    children_query: Query<&Children>,
) {
    for (track_entity, interaction) in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            // Find all sliders and match by track association
            for mut slider in &mut slider_query {
                // Simple click implementation: increment by 10%
                let old_percentage = slider.get_percentage();
                let new_percentage = if old_percentage >= 100 { 0 } else { (old_percentage + 10).min(100) };
                
                slider.set_from_percentage(new_percentage);
                
                // Update all handle positions
                for mut handle_node in &mut handle_query {
                    let handle_percentage = (slider.current_value - slider.min_value) / (slider.max_value - slider.min_value);
                    handle_node.left = Val::Percent(handle_percentage * 100.0);
                }
                
                // Update all value texts
                for mut text in &mut text_query {
                    **text = format!("{}%", slider.get_percentage());
                }
                
                // Send event for first slider (simplified)
                slider_events.write(SliderValueChanged {
                    slider_entity: track_entity, // Using track entity as identifier
                    new_value: slider.current_value,
                    percentage: slider.get_percentage(),
                });
                
                break; // Only handle one slider per click
            }
        }
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