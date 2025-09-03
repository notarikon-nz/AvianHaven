// Tooltip System
use bevy::prelude::*;

pub struct TooltipPlugin;

impl Plugin for TooltipPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TooltipState>()
            .add_systems(Update, (
                tooltip_hover_system,
                tooltip_display_system,
                tooltip_cleanup_system,
            ));
    }
}

#[derive(Resource, Default)]
pub struct TooltipState {
    pub current_tooltip: Option<Entity>,
    pub hover_timer: Timer,
    pub tooltip_text: String,
    pub show_delay: f32,
}

impl TooltipState {
    pub fn new() -> Self {
        Self {
            current_tooltip: None,
            hover_timer: Timer::from_seconds(0.5, TimerMode::Once),
            tooltip_text: String::new(),
            show_delay: 0.5,
        }
    }
}

#[derive(Component)]
pub struct Tooltip {
    pub text: String,
}

#[derive(Component)]
pub struct TooltipDisplay;

#[derive(Component)]
pub struct Hoverable {
    pub tooltip_text: String,
}

impl Hoverable {
    pub fn new(text: &str) -> Self {
        Self {
            tooltip_text: text.to_string(),
        }
    }
}

// System to detect hover states and start tooltip timer
pub fn tooltip_hover_system(
    mut tooltip_state: ResMut<TooltipState>,
    hoverable_query: Query<(Entity, &Interaction, &Hoverable), Changed<Interaction>>,
    time: Res<Time>,
) {
    let mut new_hover = None;
    
    // Check for new hovers
    for (entity, interaction, hoverable) in &hoverable_query {
        match interaction {
            Interaction::Hovered => {
                new_hover = Some((entity, hoverable.tooltip_text.clone()));
                break;
            }
            Interaction::None => {
                // Reset if we stopped hovering this entity
                if tooltip_state.current_tooltip == Some(entity) {
                    tooltip_state.current_tooltip = None;
                    tooltip_state.hover_timer.reset();
                }
            }
            _ => {}
        }
    }
    
    // Handle hover state changes
    if let Some((entity, text)) = new_hover {
        if tooltip_state.current_tooltip != Some(entity) {
            tooltip_state.current_tooltip = Some(entity);
            tooltip_state.tooltip_text = text;
            tooltip_state.hover_timer.reset();
        }
    }
    
    // Tick the hover timer
    if tooltip_state.current_tooltip.is_some() {
        tooltip_state.hover_timer.tick(time.delta());
    }
}

// System to display tooltip when timer expires
pub fn tooltip_display_system(
    mut commands: Commands,
    tooltip_state: Res<TooltipState>,
    hoverable_query: Query<&GlobalTransform, With<Hoverable>>,
    existing_tooltip_query: Query<Entity, With<TooltipDisplay>>,
    windows: Query<&Window>,
) {
    // Remove existing tooltips first
    for tooltip_entity in &existing_tooltip_query {
        commands.entity(tooltip_entity).despawn();
    }
    
    // Show new tooltip if timer finished and we have a hovered entity
    if let Some(hovered_entity) = tooltip_state.current_tooltip {
        if tooltip_state.hover_timer.finished() {
            if let Ok(window) = windows.single() {
                if let Some(cursor_pos) = window.cursor_position() {
                    // Spawn tooltip near cursor position
                    commands.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(cursor_pos.x + 10.0),
                            top: Val::Px(cursor_pos.y - 30.0),
                            padding: UiRect::all(Val::Px(8.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(1.0, 1.0, 1.0)), // Solid white background
                        BorderRadius::all(Val::Px(4.0)), // 4px rounded corners
                        ZIndex(1000), // Ensure tooltip appears above everything
                        TooltipDisplay,
                    )).with_children(|tooltip| {
                        tooltip.spawn((
                            Text::new(&tooltip_state.tooltip_text),
                            TextFont {
                                font_size: 12.0, // 12px black text
                                ..default()
                            },
                            TextColor(Color::BLACK),
                        ));
                    });
                }
            }
        }
    }
}

// System to clean up tooltips when no longer needed
pub fn tooltip_cleanup_system(
    mut commands: Commands,
    tooltip_state: Res<TooltipState>,
    tooltip_query: Query<Entity, With<TooltipDisplay>>,
) {
    // Remove tooltips if nothing is being hovered
    if tooltip_state.current_tooltip.is_none() {
        for tooltip_entity in &tooltip_query {
            commands.entity(tooltip_entity).despawn();
        }
    }
}