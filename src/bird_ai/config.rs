use bevy::prelude::*;
use bevy::reflect::TypePath;
use serde::{Deserialize, Serialize};
use crate::bird_ai::components::{BirdAction, BirdState};

#[derive(Asset, TypePath, Resource, Debug, Clone, Deserialize, Serialize)]
pub struct BehaviorTreeConfig {
    pub rules: Vec<BehaviorRule>,
    pub default_behavior: String,
    pub thresholds: BehaviorThresholds,
    pub time_periods: TimePeriods,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BehaviorRule {
    pub name: String,
    pub priority: u32,
    pub conditions: Vec<BehaviorCondition>,
    pub result: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum BehaviorCondition {
    WeatherFear { threshold: f32 },
    WeatherShelterUrgency { threshold: f32 },
    InternalStateAbove { state: String, threshold: f32 },
    InternalStateBelow { state: String, threshold: f32 },
    TimeRange { start: f32, end: f32 },
    ActionAvailable(String),
    ActionNotAvailable(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BehaviorThresholds {
    pub weather_fear_base: f32,
    pub critical_hunger: f32,
    pub critical_thirst: f32,
    pub low_energy: f32,
    pub moderate_hunger_hover: f32,
    pub moderate_hunger_eat: f32,
    pub foraging_hunger: f32,
    pub cache_low_hunger: f32,
    pub cache_high_energy: f32,
    pub moderate_thirst: f32,
    pub territorial_stress: f32,
    pub social_need_court: f32,
    pub social_need_flock: f32,
    pub social_need_follow: f32,
    pub fear_limit_follow: f32,
    pub high_energy_nest: f32,
    pub high_energy_play: f32,
    pub exploration_energy: f32,
    pub exploration_fear_limit: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TimePeriods {
    pub roosting_start: f32,
    pub roosting_end: f32,
    pub dawn_chorus_start: f32,
    pub dawn_chorus_end: f32,
    pub prime_morning_start: f32,
    pub prime_morning_end: f32,
    pub prime_evening_start: f32,
    pub prime_evening_end: f32,
}

impl BehaviorTreeConfig {
    pub fn load_from_asset(asset_server: &AssetServer) -> Handle<BehaviorTreeConfig> {
        asset_server.load("data/behavior_tree.ron")
    }
    
    pub fn get_bird_state_from_string(&self, state_str: &str) -> BirdState {
        match state_str {
            "Wandering" => BirdState::Wandering,
            "MovingToTarget" => BirdState::MovingToTarget,
            "Eating" => BirdState::Eating,
            "Drinking" => BirdState::Drinking,
            "Bathing" => BirdState::Bathing,
            "Fleeing" => BirdState::Fleeing,
            "Resting" => BirdState::Resting,
            "Playing" => BirdState::Playing,
            "Exploring" => BirdState::Exploring,
            "Nesting" => BirdState::Nesting,
            "Roosting" => BirdState::Roosting,
            "Sheltering" => BirdState::Sheltering,
            "Courting" => BirdState::Courting,
            "Following" => BirdState::Following,
            "Territorial" => BirdState::Territorial,
            "Flocking" => BirdState::Flocking,
            "Foraging" => BirdState::Foraging,
            "Caching" => BirdState::Caching,
            "Retrieving" => BirdState::Retrieving,
            "HoverFeeding" => BirdState::HoverFeeding,
            _ => BirdState::Wandering,
        }
    }
    
    pub fn get_bird_action_from_string(&self, action_str: &str) -> Option<BirdAction> {
        match action_str {
            "Eat" => Some(BirdAction::Eat),
            "Drink" => Some(BirdAction::Drink),
            "Bathe" => Some(BirdAction::Bathe),
            "Perch" => Some(BirdAction::Perch),
            "Play" => Some(BirdAction::Play),
            "Explore" => Some(BirdAction::Explore),
            "Nest" => Some(BirdAction::Nest),
            "Roost" => Some(BirdAction::Roost),
            "Shelter" => Some(BirdAction::Shelter),
            "Court" => Some(BirdAction::Court),
            "Follow" => Some(BirdAction::Follow),
            "Challenge" => Some(BirdAction::Challenge),
            "Flock" => Some(BirdAction::Flock),
            "Forage" => Some(BirdAction::Forage),
            "Cache" => Some(BirdAction::Cache),
            "Retrieve" => Some(BirdAction::Retrieve),
            "HoverFeed" => Some(BirdAction::HoverFeed),
            _ => None,
        }
    }
}