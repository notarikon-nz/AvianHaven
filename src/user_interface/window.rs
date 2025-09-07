use bevy::prelude::*;

// COMPONENTS

#[derive(Component, Debug, Clone, Reflect)]
pub struct WindowContainer {
    pub title: String,
    pub resizable: bool,
    pub movable: bool,
    pub closable: bool,
}

#[derive(Component, Debug, Clone, Reflect, Default)]
pub struct WindowState {
    pub is_open: bool,
    pub is_focused: bool,
    pub position: Vec2,
    pub size: Vec2,
}

#[derive(Component)]
pub struct WindowRoot;

#[derive(Component)]
pub struct WindowTitleBar;

#[derive(Component)]
pub struct WindowTitleText;

#[derive(Component)]
pub struct WindowCloseButton;

#[derive(Component)]
pub struct WindowContent;

// EVENTS

#[derive(Event, Debug, Clone)]
pub struct WindowOpenedEvent {
    pub entity: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct WindowClosedEvent {
    pub entity: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct WindowFocusedEvent {
    pub entity: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct WindowResizedEvent {
    pub entity: Entity,
    pub new_size: Vec2,
}

// SYSTEMS

pub fn window_open_close_system(
    mut commands: Commands,
    mut q_windows: Query<(Entity, &mut WindowState, &WindowContainer)>,
    mut ev_opened: EventWriter<WindowOpenedEvent>,
    mut ev_closed: EventWriter<WindowClosedEvent>,
) {
    for (entity, mut state, container) in &mut q_windows {
        if container.closable && !state.is_open {
            ev_closed.send(WindowClosedEvent { entity });
            commands.entity(entity).insert(Visibility::Hidden);
        } else if state.is_open {
            ev_opened.send(WindowOpenedEvent { entity });
            commands.entity(entity).insert(Visibility::Visible);
        }
    }
}

pub fn window_focus_system(
    mut q_windows: Query<(Entity, &Interaction, &mut WindowState), (With<WindowRoot>, Changed<Interaction>)>,
    mut ev_focused: EventWriter<WindowFocusedEvent>,
) {
    for (entity, interaction, mut state) in &mut q_windows {
        if *interaction == Interaction::Pressed {
            state.is_focused = true;
            ev_focused.send(WindowFocusedEvent { entity });
        }
    }
}

pub fn window_drag_system(
    mut q_windows: Query<(&mut Style, &mut WindowState, &WindowContainer, &Children), With<WindowRoot>>,
    q_titlebar: Query<&Interaction, With<WindowTitleBar>>,
    windows: Res<Windows>,
) {
    let primary = windows.get_primary().unwrap();

    for (mut style, mut state, container, children) in &mut q_windows {
        if container.movable {
            if let Some(&titlebar_entity) = children.iter().find(|&&c| q_titlebar.get(c).is_ok()) {
                if let Ok(interaction) = q_titlebar.get(titlebar_entity) {
                    if *interaction == Interaction::Pressed {
                        // Example: snap to cursor, refine with delta drag
                        let cursor = primary.cursor_position().unwrap_or_default();
                        state.position = cursor;
                        style.position_type = PositionType::Absolute;
                        style.left = Val::Px(cursor.x);
                        style.top = Val::Px(cursor.y);
                    }
                }
            }
        }
    }
}

// BUILDER

pub struct WindowBuilder<'w, 's, 'a> {
    commands: &'a mut Commands<'w, 's>,
    title: String,
    size: Vec2,
    resizable: bool,
    movable: bool,
    closable: bool,
}

impl<'w, 's, 'a> WindowBuilder<'w, 's, 'a> {
    pub fn new(commands: &'a mut Commands<'w, 's>, title: impl Into<String>) -> Self {
        Self {
            commands,
            title: title.into(),
            size: Vec2::new(400.0, 300.0),
            resizable: true,
            movable: true,
            closable: true,
        }
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }

    pub fn resizable(mut self, value: bool) -> Self {
        self.resizable = value;
        self
    }

    pub fn movable(mut self, value: bool) -> Self {
        self.movable = value;
        self
    }

    pub fn closable(mut self, value: bool) -> Self {
        self.closable = value;
        self
    }

    pub fn spawn(self) -> Entity {
        let window_entity = self.commands.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(self.size.x),
                    height: Val::Px(self.size.y),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..default()
            },
            WindowContainer {
                title: self.title.clone(),
                resizable: self.resizable,
                movable: self.movable,
                closable: self.closable,
            },
            WindowState {
                is_open: true,
                is_focused: false,
                position: Vec2::ZERO,
                size: self.size,
            },
            WindowRoot,
        )).id();

        // Title bar
        let titlebar = self.commands.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(30.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::GRAY.into(),
                ..default()
            },
            WindowTitleBar,
        )).id();

        // Title text
        let text = self.commands.spawn((
            TextBundle::from_section(
                self.title,
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                }
            ),
            WindowTitleText,
        )).id();

        // Close button
        let close_button = self.commands.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(20.0),
                    height: Val::Px(20.0),
                    ..default()
                },
                background_color: Color::RED.into(),
                ..default()
            },
            WindowCloseButton,
        )).id();

        // Content area
        let content = self.commands.spawn((
            NodeBundle {
                style: Style {
                    flex_grow: 1.0,
                    width: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..default()
            },
            WindowContent,
        )).id();

        self.commands.entity(titlebar).push_children(&[text, close_button]);
        self.commands.entity(window_entity).push_children(&[titlebar, content]);

        window_entity
    }
}

