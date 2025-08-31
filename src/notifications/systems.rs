use bevy::prelude::*;
use super::{components::*, resources::*};
use std::time::Duration;

pub fn setup_notification_container(mut commands: Commands) {
    // Create a fixed container for notifications in top-right corner
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            right: Val::Px(20.0),
            width: Val::Px(350.0),
            height: Val::Auto,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            ..default()
        },
        NotificationContainer,
        Visibility::Inherited,
    ));
}

pub fn notification_spawner_system(
    mut commands: Commands,
    mut notification_events: EventReader<ShowNotificationEvent>,
    mut notification_queue: ResMut<NotificationQueue>,
    container_query: Query<Entity, With<NotificationContainer>>,
) {
    // Add new notifications to queue
    for event in notification_events.read() {
        notification_queue.push(event.notification.clone());
    }
    
    // Spawn notifications from queue if space available
    let Ok(container_entity) = container_query.single() else {
        return;
    };
    
    while let Some(notification_type) = notification_queue.pop() {
        spawn_notification(&mut commands, container_entity, notification_type);
    }
}

fn spawn_notification(
    commands: &mut Commands,
    container_entity: Entity,
    notification_type: NotificationType,
) {
    let notification_id = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Auto,
            padding: UiRect::all(Val::Px(15.0)),
            margin: UiRect::bottom(Val::Px(5.0)),
            border: UiRect::all(Val::Px(2.0)),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(10.0),
            ..default()
        },
        BackgroundColor(notification_type.background_color()),
        BorderColor(notification_type.border_color()),
        Notification {
            lifetime: Timer::new(Duration::from_secs(4), TimerMode::Once),
            notification_type: notification_type.clone(),
        },
        // Start with scale 0 for animation
        Transform::from_scale(Vec3::ZERO),
    )).id();

    commands.entity(notification_id).with_children(|parent| {
        // Icon
        parent.spawn((
            Text::new(notification_type.icon()),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
        
        // Content area
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                flex_grow: 1.0,
                row_gap: Val::Px(2.0),
                ..default()
            },
        )).with_children(|content| {
            match &notification_type {
                NotificationType::Achievement { title, description, currency_reward } => {
                    // Achievement title
                    content.spawn((
                        Text::new(format!("Achievement Unlocked: {}", title)),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                    
                    // Achievement description
                    content.spawn((
                        Text::new(description.clone()),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                    
                    // Currency reward
                    content.spawn((
                        Text::new(format!("Reward: {} currency", currency_reward)),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.6, 0.2)),
                    ));
                },
                NotificationType::Currency { amount, reason } => {
                    content.spawn((
                        Text::new(format!("Earned {} currency", amount)),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                    
                    content.spawn((
                        Text::new(reason.clone()),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                },
                NotificationType::Warning { message } => {
                    content.spawn((
                        Text::new(message.clone()),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                },
                NotificationType::Info { message } => {
                    content.spawn((
                        Text::new(message.clone()),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                },
            }
        });
    });

    // Add to container
    commands.entity(container_entity).add_child(notification_id);
}

pub fn notification_animation_system(
    mut notification_query: Query<(&mut Transform, &mut Notification), With<Notification>>,
    time: Res<Time>,
) {
    for (mut transform, mut notification) in &mut notification_query {
        notification.lifetime.tick(time.delta());
        
        let elapsed = notification.lifetime.elapsed().as_secs_f32();
        let duration = notification.lifetime.duration().as_secs_f32();
        
        if elapsed < 0.2 {
            // Scale in animation (first 0.2 seconds)
            let scale_progress = elapsed / 0.2;
            let scale = ease_out_back(scale_progress);
            transform.scale = Vec3::splat(scale);
        } else if elapsed > duration - 0.5 {
            // Scale out animation (last 0.5 seconds)
            let fade_progress = (elapsed - (duration - 0.5)) / 0.5;
            let scale = 1.0 - ease_in_cubic(fade_progress);
            transform.scale = Vec3::splat(scale.max(0.0));
        } else {
            // Fully visible
            transform.scale = Vec3::ONE;
        }
    }
}

pub fn notification_cleanup_system(
    mut commands: Commands,
    mut notification_queue: ResMut<NotificationQueue>,
    notification_query: Query<(Entity, &Notification), With<Notification>>,
) {
    for (entity, notification) in &notification_query {
        if notification.lifetime.finished() {
            commands.entity(entity).despawn();
            notification_queue.notification_completed();
        }
    }
}

// Easing functions for smooth animations
fn ease_out_back(t: f32) -> f32 {
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
}

fn ease_in_cubic(t: f32) -> f32 {
    t * t * t
}