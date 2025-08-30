use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component, Default)]
pub struct BirdAI;

#[derive(Component, Default)]
pub struct Blackboard {
    pub internal: InternalState,
    pub world_knowledge: WorldKnowledge,
    pub current_target: Option<Entity>,
}

#[derive(Default)]
pub struct InternalState {
    pub hunger: f32,
    pub thirst: f32,
    pub energy: f32,
    pub fear: f32,
}

#[derive(Default)]
pub struct WorldKnowledge {
    pub perceived_threat: Option<Vec2>,
    pub available_actions: HashMap<BirdAction, UtilityEntry>,
}

pub struct UtilityEntry {
    pub entity: Entity,
    pub score: f32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum BirdState {
    #[default]
    Wandering,
    MovingToTarget,
    Eating,
    Drinking,
    Bathing,
    Fleeing,
    Resting,
}

#[derive(Component)]
pub struct SmartObject;

#[derive(Component, Clone)]
pub struct ProvidesUtility {
    pub action: BirdAction,
    pub base_utility: f32,
    pub range: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BirdAction {
    Eat,
    Drink,
    Bathe,
    Perch,
}