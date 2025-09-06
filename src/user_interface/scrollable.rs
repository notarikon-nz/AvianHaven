use bevy::prelude::*;
use bevy::ui::FocusPolicy;

/// Scrollable container component
#[derive(Component, Debug)]
pub struct ScrollableContainer {
    /// Current scroll position (0.0 = top, 1.0 = bottom)
    pub scroll_position: f32,
    /// Content height in pixels
    pub content_height: f32,
    /// Visible area height in pixels
    pub viewport_height: f32,
    /// Scroll sensitivity for mouse wheel
    pub scroll_speed: f32,
    /// Whether scrolling is enabled
    pub enabled: bool,
}

impl Default for ScrollableContainer {
    fn default() -> Self {
        Self {
            scroll_position: 0.0,
            content_height: 0.0,
            viewport_height: 0.0,
            scroll_speed: 50.0,
            enabled: true,
        }
    }
}

/// Scrollbar component
#[derive(Component)]
pub struct Scrollbar;

/// Scrollbar track component
#[derive(Component)]
pub struct ScrollbarTrack;

/// Scrollbar thumb (the draggable part)
#[derive(Component)]
pub struct ScrollbarThumb {
    pub dragging: bool,
    pub drag_start_y: f32,
    pub drag_start_scroll: f32,
}

impl Default for ScrollbarThumb {
    fn default() -> Self {
        Self {
            dragging: false,
            drag_start_y: 0.0,
            drag_start_scroll: 0.0,
        }
    }
}

/// Content area that gets scrolled
#[derive(Component)]
pub struct ScrollableContent;

/// Event for scroll changes
#[derive(Event)]
pub struct ScrollEvent {
    pub entity: Entity,
    pub delta: f32,
}

/// Builder for creating scrollable containers
pub struct ScrollableBuilder<'w, 's, 'a> {
    commands: &'a mut Commands<'w, 's>,
    container_size: Vec2,
    content_height: f32,
    scroll_speed: f32,
}

impl<'w, 's, 'a> ScrollableBuilder<'w, 's, 'a> {
    pub fn new(commands: &'a mut Commands<'w, 's>) -> Self {
        Self {
            commands,
            container_size: Vec2::new(400.0, 300.0),
            content_height: 600.0,
            scroll_speed: 50.0,
        }
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.container_size = size;
        self
    }

    pub fn with_content_height(mut self, height: f32) -> Self {
        self.content_height = height;
        self
    }

    pub fn with_scroll_speed(mut self, speed: f32) -> Self {
        self.scroll_speed = speed;
        self
    }

    pub fn spawn(self) -> (Entity, Entity) {
        let scrollbar_width = 16.0;
        let content_width = self.container_size.x - scrollbar_width - 4.0; // 4px gap

        // Create scrollbar track
        let track_entity = self.commands.spawn((
            Node {
                width: Val::Px(scrollbar_width),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.8, 0.8, 0.8)),
            BorderRadius::all(Val::Px(8.0)),
            ScrollbarTrack,
        )).id();

        // Create scrollbar thumb
        let thumb_entity = self.commands.spawn((
            Node {
                width: Val::Px(scrollbar_width - 4.0),
                height: Val::Px(40.0), // Initial size, will be updated
                position_type: PositionType::Absolute,
                right: Val::Px(2.0),
                top: Val::Px(2.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
            BorderRadius::all(Val::Px(6.0)),
            Button,
            ScrollbarThumb::default(),
        )).id();

        // Create scrollable content area
        let content_entity = self.commands.spawn((
            Node {
                width: Val::Px(content_width),
                height: Val::Px(self.content_height),
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                overflow: Overflow::clip(),
                ..default()
            },
            ScrollableContent,
            FocusPolicy::Pass, // Allow interaction events to pass through to children
        )).id();

        // Create main container
        let container_entity = self.commands.spawn((
            Node {
                width: Val::Px(self.container_size.x),
                height: Val::Px(self.container_size.y),
                position_type: PositionType::Relative,
                overflow: Overflow::clip(),
                ..default()
            },
            ScrollableContainer {
                content_height: self.content_height,
                viewport_height: self.container_size.y,
                scroll_speed: self.scroll_speed,
                ..default()
            },
            Interaction::default(),
            FocusPolicy::Pass, // Allow interaction events to pass through to children
        )).id();

        // Set up hierarchy
        self.commands.entity(container_entity)
            .add_children(&[content_entity, track_entity]);
        self.commands.entity(track_entity)
            .add_child(thumb_entity);

        (container_entity, content_entity)
    }
}