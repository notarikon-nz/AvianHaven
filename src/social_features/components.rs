// Social Features - Phase 4
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crate::bird::BirdSpecies;
use crate::photo_mode::components::PhotoScore;

// Community Challenge System
#[derive(Resource, Default)]
pub struct CommunitySystem {
    pub active_challenges: Vec<Challenge>,
    pub player_stats: PlayerStats,
    pub leaderboards: Vec<Leaderboard>,
    pub shared_photos: Vec<SharedPhoto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Challenge {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub challenge_type: ChallengeType,
    pub difficulty: ChallengeDifficulty,
    pub start_date: String,
    pub end_date: String,
    pub rewards: ChallengeRewards,
    pub progress: ChallengeProgress,
    pub participants: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChallengeType {
    SpeciesPhoto {
        target_species: BirdSpecies,
        min_score: u32,
    },
    EnvironmentalPhoto {
        weather_condition: String,
        time_of_day: String,
    },
    TechnicalPhoto {
        required_lens: String,
        required_filter: String,
    },
    BehaviorCapture {
        target_behavior: String,
        min_duration: f32,
    },
    SeriesPhoto {
        theme: String,
        required_count: u32,
    },
    CollaborativeGoal {
        community_target: u32,
        current_progress: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChallengeDifficulty {
    Beginner,    // 100-300 reward
    Intermediate,// 300-600 reward
    Advanced,    // 600-1000 reward
    Expert,      // 1000+ reward
}

impl ChallengeDifficulty {
    pub fn base_reward(&self) -> u32 {
        match self {
            Self::Beginner => 200,
            Self::Intermediate => 450,
            Self::Advanced => 800,
            Self::Expert => 1200,
        }
    }
    
    pub fn color(&self) -> Color {
        match self {
            Self::Beginner => Color::srgb(0.2, 0.8, 0.2),
            Self::Intermediate => Color::srgb(0.2, 0.5, 0.8),
            Self::Advanced => Color::srgb(0.8, 0.5, 0.2),
            Self::Expert => Color::srgb(0.8, 0.2, 0.2),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeRewards {
    pub currency: u32,
    pub unlockable_lens: Option<String>,
    pub unlockable_filter: Option<String>,
    pub title: Option<String>,
    pub badge: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeProgress {
    pub completed: bool,
    pub current_value: u32,
    pub target_value: u32,
    pub best_submission: Option<u32>, // Photo score
}

// Leaderboard System
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Leaderboard {
    pub leaderboard_type: LeaderboardType,
    pub entries: Vec<LeaderboardEntry>,
    pub season: String,
    pub last_updated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LeaderboardType {
    TopPhotographers,    // Overall score
    SpeciesSpecialist(BirdSpecies),
    WeeklyChallenge,
    MonthlyChallenge,
    ConsecutiveDays,
    RareFinds,
    CommunityContributor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub rank: u32,
    pub player_name: String,
    pub score: u32,
    pub badge: Option<String>,
    pub recent_achievement: Option<String>,
}

// Photo Sharing System
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedPhoto {
    pub id: u32,
    pub author: String,
    pub species: Option<BirdSpecies>,
    pub score: PhotoScore,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub likes: u32,
    pub views: u32,
    pub shared_date: String,
    pub featured: bool,
    pub challenge_submission: Option<u32>, // Challenge ID
}

// Player Statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerStats {
    pub username: String,
    pub total_photos: u32,
    pub best_photo_score: u32,
    pub species_photographed: Vec<BirdSpecies>,
    pub challenges_completed: u32,
    pub current_streak: u32,
    pub longest_streak: u32,
    pub community_rank: u32,
    pub badges_earned: Vec<Badge>,
    pub titles_unlocked: Vec<PlayerTitle>,
    pub favorite_species: Option<BirdSpecies>,
    pub photography_level: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Badge {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub earned_date: String,
    pub rarity: BadgeRarity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BadgeRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerTitle {
    pub id: String,
    pub title: String,
    pub requirement: String,
    pub active: bool,
}

// Events
#[derive(Event)]
pub struct ChallengeCompletedEvent {
    pub challenge_id: u32,
    pub photo_score: u32,
}

#[derive(Event)]
pub struct PhotoSharedEvent {
    pub photo: SharedPhoto,
}

#[derive(Event)]
pub struct LeaderboardUpdateEvent;

#[derive(Event)]
pub struct BadgeEarnedEvent {
    pub badge: Badge,
}

// Components for UI
#[derive(Component)]
pub struct ChallengesUI;

#[derive(Component)]
pub struct LeaderboardUI;

#[derive(Component)]
pub struct PhotoGalleryUI;

#[derive(Component)]
pub struct CommunityHubUI;

// Challenge Generation
impl Challenge {
    pub fn generate_daily_challenges() -> Vec<Challenge> {
        vec![
            Challenge {
                id: 1,
                title: "Morning Songbird".to_string(),
                description: "Capture a bird singing during the golden hour".to_string(),
                challenge_type: ChallengeType::BehaviorCapture {
                    target_behavior: "Singing".to_string(),
                    min_duration: 3.0,
                },
                difficulty: ChallengeDifficulty::Beginner,
                start_date: "2025-01-01".to_string(),
                end_date: "2025-01-02".to_string(),
                rewards: ChallengeRewards {
                    currency: 200,
                    unlockable_lens: None,
                    unlockable_filter: Some("Warm".to_string()),
                    title: Some("Dawn Serenader".to_string()),
                    badge: None,
                },
                progress: ChallengeProgress {
                    completed: false,
                    current_value: 0,
                    target_value: 1,
                    best_submission: None,
                },
                participants: 0,
            },
            Challenge {
                id: 2,
                title: "Cardinal in the Snow".to_string(),
                description: "Photograph a Cardinal during winter weather".to_string(),
                challenge_type: ChallengeType::SpeciesPhoto {
                    target_species: BirdSpecies::Cardinal,
                    min_score: 600,
                },
                difficulty: ChallengeDifficulty::Intermediate,
                start_date: "2025-01-01".to_string(),
                end_date: "2025-01-07".to_string(),
                rewards: ChallengeRewards {
                    currency: 450,
                    unlockable_lens: Some("Telephoto".to_string()),
                    unlockable_filter: None,
                    title: Some("Winter Photographer".to_string()),
                    badge: Some("Snow Bird".to_string()),
                },
                progress: ChallengeProgress {
                    completed: false,
                    current_value: 0,
                    target_value: 1,
                    best_submission: None,
                },
                participants: 0,
            },
        ]
    }
    
    pub fn is_active(&self) -> bool {
        !self.progress.completed
    }
    
    pub fn check_completion(&mut self, photo_score: u32, species: Option<BirdSpecies>) -> bool {
        match &self.challenge_type {
            ChallengeType::SpeciesPhoto { target_species, min_score } => {
                if let Some(photo_species) = species {
                    if photo_species == *target_species && photo_score >= *min_score {
                        self.progress.completed = true;
                        self.progress.current_value = 1;
                        self.progress.best_submission = Some(photo_score);
                        return true;
                    }
                }
            },
            ChallengeType::BehaviorCapture { target_behavior: _, min_duration: _ } => {
                // For now, just accept any photo with good score
                if photo_score >= 600 {
                    self.progress.completed = true;
                    self.progress.current_value = 1;
                    self.progress.best_submission = Some(photo_score);
                    return true;
                }
            },
            _ => {
                // Simple completion for other challenge types
                if photo_score >= 500 {
                    self.progress.current_value += 1;
                    if self.progress.current_value >= self.progress.target_value {
                        self.progress.completed = true;
                        self.progress.best_submission = Some(photo_score);
                        return true;
                    }
                }
            }
        }
        false
    }
}