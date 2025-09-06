use bevy::prelude::*;
use crate::bird_ai::config::BehaviorTreeConfig;

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

#[derive(Resource)]
pub struct BehaviorTreeConfigResource {
    pub config: Option<BehaviorTreeConfig>,
    pub use_configurable: bool,
    pub handle: Option<Handle<BehaviorTreeConfig>>,
}

impl Default for BehaviorTreeConfigResource {
    fn default() -> Self {
        Self {
            config: None,
            use_configurable: false,
            handle: None,
        }
    }
}