use bevy::prelude::*;
use crate::bird::BirdSpecies;
use serde::{Serialize, Deserialize};

#[derive(Component)]
pub struct JournalMenu;

#[derive(Component)]
pub struct JournalTabButton {
    pub tab: JournalTab,
}

#[derive(Component)]
pub struct JournalTabContent {
    pub tab: JournalTab,
}

#[derive(Component)]
pub struct SpeciesButton(pub BirdSpecies);

#[derive(Component)]
pub struct SpeciesCard {
    pub species: BirdSpecies,
}

#[derive(Component)]
pub struct DetailPanel;

#[derive(Component)]
pub struct CloseButton;

#[derive(Component)]
pub struct JournalBackground;

#[derive(Component)]
pub struct TabIndicator;

#[derive(Component)]
pub struct FieldNotePage;

#[derive(Component)]
pub struct ConservationStatusBadge;

#[derive(Component)]
pub struct MigrationMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JournalTab {
    Species,
    Photos,
    Conservation,
    Migration,
    Research,
    Achievements,
}

#[derive(Component)]
pub struct ResearchMissionCard {
    pub mission_id: u32,
}

#[derive(Component)]
pub struct MissionProgressBar;

#[derive(Component)]
pub struct ResearchDataVisualization;
