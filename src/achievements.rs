use bevy::prelude::*;
use std::collections::HashSet;
use crate::bird::BirdSpecies;
use crate::photo_mode::components::PhotoTakenEvent;
use crate::notifications::{resources::ShowNotificationEvent, components::NotificationType};

pub struct AchievementPlugin;

impl Plugin for AchievementPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AchievementProgress>()
            .add_event::<AchievementUnlockedEvent>()
            .add_systems(Update, (
                photo_achievement_system,
                species_achievement_system,
                currency_achievement_system,
                achievement_notification_system,
            ).run_if(in_state(crate::AppState::Playing)));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Achievement {
    // Photography achievements
    FirstPhoto,
    PhotoMaster,          // Take 100 photos
    ActionShot,           // Photo with behavior_score >= 50
    MultiSpeciesShot,     // Photo with multiple birds
    
    // Species collection achievements  
    FirstSpecies,
    CommonCollector,      // Discover 10 species
    Ornithologist,        // Discover all 20 Tier 1 species
    
    // Currency achievements
    Wealthy,              // Accumulate 1000 currency
    Millionaire,          // Accumulate 10000 currency
    
    // Feeder achievements
    FeederMaintainer,     // Upgrade first feeder
    FeederExpert,         // Have 3 level-2+ feeders
}

impl Achievement {
    pub fn name(&self) -> &'static str {
        match self {
            Self::FirstPhoto => "First Snapshot",
            Self::PhotoMaster => "Photo Master",
            Self::ActionShot => "Action Shot",
            Self::MultiSpeciesShot => "Flock Photographer",
            Self::FirstSpecies => "First Discovery",
            Self::CommonCollector => "Common Collector",
            Self::Ornithologist => "Ornithologist",
            Self::Wealthy => "Wealthy",
            Self::Millionaire => "Millionaire",
            Self::FeederMaintainer => "Feeder Maintainer",
            Self::FeederExpert => "Feeder Expert",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            Self::FirstPhoto => "Take your first photo",
            Self::PhotoMaster => "Take 100 photos",
            Self::ActionShot => "Capture a bird feeding, drinking, or bathing",
            Self::MultiSpeciesShot => "Photograph multiple birds in one shot",
            Self::FirstSpecies => "Discover your first bird species",
            Self::CommonCollector => "Discover 10 different species",
            Self::Ornithologist => "Discover all 20 common bird species",
            Self::Wealthy => "Accumulate 1,000 currency",
            Self::Millionaire => "Accumulate 10,000 currency",
            Self::FeederMaintainer => "Upgrade your first feeder",
            Self::FeederExpert => "Have 3 feeders at level 2 or higher",
        }
    }
    
    pub fn currency_reward(&self) -> u32 {
        match self {
            Self::FirstPhoto | Self::FirstSpecies => 25,
            Self::ActionShot | Self::MultiSpeciesShot => 50,
            Self::FeederMaintainer => 75,
            Self::CommonCollector | Self::FeederExpert => 100,
            Self::PhotoMaster | Self::Ornithologist => 200,
            Self::Wealthy => 250,
            Self::Millionaire => 500,
        }
    }
}

fn achievement_notification_system(
    mut achievement_events: EventReader<AchievementUnlockedEvent>,
    mut currency: ResMut<crate::photo_mode::resources::CurrencyResource>,
    mut notification_events: EventWriter<ShowNotificationEvent>,
) {
    for event in achievement_events.read() {
        let reward = event.achievement.currency_reward();
        currency.0 += reward;
        
        // Send popup notification instead of console log
        notification_events.write(ShowNotificationEvent {
            notification: NotificationType::Achievement {
                title: event.achievement.name().to_string(),
                description: event.achievement.description().to_string(),
                currency_reward: reward,
            },
        });
    }
}

#[derive(Resource, Default)]
pub struct AchievementProgress {
    pub unlocked: HashSet<Achievement>,
    pub photos_taken: u32,
    pub species_discovered: u32,
    pub action_shots_taken: u32,
    pub multi_bird_shots: u32,
}

impl AchievementProgress {
    pub fn unlock(&mut self, achievement: Achievement) -> bool {
        self.unlocked.insert(achievement)
    }
    
    pub fn is_unlocked(&self, achievement: &Achievement) -> bool {
        self.unlocked.contains(achievement)
    }
}

#[derive(Event)]
pub struct AchievementUnlockedEvent {
    pub achievement: Achievement,
}

// Achievement systems

fn photo_achievement_system(
    mut photo_events: EventReader<PhotoTakenEvent>,
    mut progress: ResMut<AchievementProgress>,
    mut achievement_events: EventWriter<AchievementUnlockedEvent>,
) {
    for event in photo_events.read() {
        progress.photos_taken += 1;
        
        // First photo achievement
        if progress.photos_taken == 1 && !progress.is_unlocked(&Achievement::FirstPhoto) {
            progress.unlock(Achievement::FirstPhoto);
            achievement_events.write(AchievementUnlockedEvent { achievement: Achievement::FirstPhoto });
        }
        
        // Photo master achievement
        if progress.photos_taken >= 100 && !progress.is_unlocked(&Achievement::PhotoMaster) {
            progress.unlock(Achievement::PhotoMaster);
            achievement_events.write(AchievementUnlockedEvent { achievement: Achievement::PhotoMaster });
        }
        
        // Action shot achievement
        if event.score.behavior_score >= 50 {
            progress.action_shots_taken += 1;
            if !progress.is_unlocked(&Achievement::ActionShot) {
                progress.unlock(Achievement::ActionShot);
                achievement_events.write(AchievementUnlockedEvent { achievement: Achievement::ActionShot });
            }
        }
        
        // Multi-bird achievement
        if event.score.rarity_bonus > 0 {
            progress.multi_bird_shots += 1;
            if !progress.is_unlocked(&Achievement::MultiSpeciesShot) {
                progress.unlock(Achievement::MultiSpeciesShot);
                achievement_events.write(AchievementUnlockedEvent { achievement: Achievement::MultiSpeciesShot });
            }
        }
    }
}

fn species_achievement_system(
    discovered_species: Res<crate::photo_mode::resources::DiscoveredSpecies>,
    mut progress: ResMut<AchievementProgress>,
    mut achievement_events: EventWriter<AchievementUnlockedEvent>,
) {
    if discovered_species.is_changed() {
        let species_count = discovered_species.species.len() as u32;
        progress.species_discovered = species_count;
        
        // First species
        if species_count >= 1 && !progress.is_unlocked(&Achievement::FirstSpecies) {
            progress.unlock(Achievement::FirstSpecies);
            achievement_events.write(AchievementUnlockedEvent { achievement: Achievement::FirstSpecies });
        }
        
        // Common collector
        if species_count >= 10 && !progress.is_unlocked(&Achievement::CommonCollector) {
            progress.unlock(Achievement::CommonCollector);
            achievement_events.write(AchievementUnlockedEvent { achievement: Achievement::CommonCollector });
        }
        
        // Ornithologist
        if species_count >= 20 && !progress.is_unlocked(&Achievement::Ornithologist) {
            progress.unlock(Achievement::Ornithologist);
            achievement_events.write(AchievementUnlockedEvent { achievement: Achievement::Ornithologist });
        }
    }
}

fn currency_achievement_system(
    currency: Res<crate::photo_mode::resources::CurrencyResource>,
    mut progress: ResMut<AchievementProgress>,
    mut achievement_events: EventWriter<AchievementUnlockedEvent>,
) {
    if currency.is_changed() {
        // Wealthy achievement
        if currency.0 >= 1000 && !progress.is_unlocked(&Achievement::Wealthy) {
            progress.unlock(Achievement::Wealthy);
            achievement_events.write(AchievementUnlockedEvent { achievement: Achievement::Wealthy });
        }
        
        // Millionaire achievement
        if currency.0 >= 10000 && !progress.is_unlocked(&Achievement::Millionaire) {
            progress.unlock(Achievement::Millionaire);
            achievement_events.write(AchievementUnlockedEvent { achievement: Achievement::Millionaire });
        }
    }
}