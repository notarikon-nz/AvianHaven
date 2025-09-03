// Reusable UI Widgets
use bevy::prelude::*;

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