use bevy::prelude::*;
use super::components::NotificationType;
use std::collections::VecDeque;

#[derive(Resource, Default)]
pub struct NotificationQueue {
    pub notifications: VecDeque<NotificationType>,
    pub max_concurrent: usize,
    pub current_count: usize,
}

impl NotificationQueue {
    pub fn new() -> Self {
        Self {
            notifications: VecDeque::new(),
            max_concurrent: 3, // Show max 3 notifications at once
            current_count: 0,
        }
    }
    
    pub fn push(&mut self, notification: NotificationType) {
        self.notifications.push_back(notification);
    }
    
    pub fn pop(&mut self) -> Option<NotificationType> {
        if self.current_count < self.max_concurrent {
            self.current_count += 1;
            self.notifications.pop_front()
        } else {
            None
        }
    }
    
    pub fn notification_completed(&mut self) {
        if self.current_count > 0 {
            self.current_count -= 1;
        }
    }
}

#[derive(Event)]
pub struct ShowNotificationEvent {
    pub notification: NotificationType,
}