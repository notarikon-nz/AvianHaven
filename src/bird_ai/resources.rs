use bevy::prelude::*;

#[derive(Resource)]
pub struct UtilityTimer(pub Timer);

impl Default for UtilityTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

#[derive(Resource)]
pub struct BehaviorTreeTimer(pub Timer);

impl Default for BehaviorTreeTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2.0, TimerMode::Repeating))
    }
}