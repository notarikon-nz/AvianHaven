use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{AppState, resources::{GameConfig, BirdCount, SpawnBirdEvent}};
use crate::bird_ai::components::{BirdAI, BirdState, Blackboard, InternalState, SocialBirdTraits, SocialRelationships, ForagingTraits, CacheData, ForagingState};
use crate::animation::components::AnimatedBird;
use crate::feeder::FeederType;
use crate::environment::resources::{TimeState, WeatherState, SeasonalState};
use crate::environment::components::Season;
use crate::journal::resources::BirdEducationData;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BirdCount>()
            .add_event::<SpawnBirdEvent>()
            .add_systems(Startup, spawn_initial_birds)
            .add_systems(
                Update,
                (
                    handle_spawn_events,
                    bird_movement,
                    update_wander_timer,
                    environmental_bird_spawning_system,
                ).run_if(in_state(AppState::Playing)),
            );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum BirdSpecies {
    // Tier 1 - Common backyard birds (20 species)
    Cardinal,
    BlueJay,
    Robin,
    Sparrow,
    Chickadee,
    Goldfinch,
    NorthernMockingbird,
    RedWingedBlackbird,
    CommonGrackle,
    BrownThrasher,
    CedarWaxwing,
    WhiteBreastedNuthatch,
    TuftedTitmouse,
    CarolinaWren,
    HouseFinch,
    EuropeanStarling,
    MourningDove,
    CommonCrow,
    BlueGrayGnatcatcher,
    YellowWarbler,
    
    // Tier 2 - Uncommon (15 species)
    DownyWoodpecker,
    HairyWoodpecker,
    PileatedWoodpecker,
    RedHeadedWoodpecker,
    RubyThroatedHummingbird,
    PurpleFinch,
    IndianaBunting,
    RoseBreastedGrosbeak,
    WoodThrush,
    Catbird,
    ScarletTanager,
    BaltimoreOriole,
    WinterWren,
    BrownCreeper,
    YellowBelledSapsucker,
    
    // Tier 3 - Rare (10 species)
    RedTailedHawk,
    CoopersHawk,
    GreatHornedOwl,
    BarredOwl,
    EasternBluebird,
    BelttedKingfisher,
    GrandSlamAmerican,
    PaintedBunting,
    CeruleanWarbler,
    HoodedWarbler,
    
    // Tier 4 - Legendary (5 species)
    BaldEagle,
    PeregrineFalcon,
    ProthonotaryWarbler,
    KentuckyWarbler,
    GoldenWingedWarbler,
}

impl BirdSpecies {
    fn random() -> Self {
        Self::random_with_rarity(1.0)
    }
    
    pub fn random_with_rarity(rarity_boost: f32) -> Self {
        let mut rng = rand::rng();
        let roll = rng.random::<f32>();
        
        // Tier probabilities (modified by rarity_boost)
        let tier2_chance = 0.15 * rarity_boost; // 15% for uncommon
        let tier3_chance = 0.05 * rarity_boost; // 5% for rare  
        let tier4_chance = 0.01 * rarity_boost; // 1% for legendary
        
        if roll < tier4_chance {
            // Tier 4 - Legendary
            match rng.random_range(0..5) {
                0 => Self::BaldEagle,
                1 => Self::PeregrineFalcon,
                2 => Self::ProthonotaryWarbler,
                3 => Self::KentuckyWarbler,
                _ => Self::GoldenWingedWarbler,
            }
        } else if roll < tier4_chance + tier3_chance {
            // Tier 3 - Rare
            match rng.random_range(0..10) {
                0 => Self::RedTailedHawk,
                1 => Self::CoopersHawk,
                2 => Self::GreatHornedOwl,
                3 => Self::BarredOwl,
                4 => Self::EasternBluebird,
                5 => Self::BelttedKingfisher,
                6 => Self::GrandSlamAmerican,
                7 => Self::PaintedBunting,
                8 => Self::CeruleanWarbler,
                _ => Self::HoodedWarbler,
            }
        } else if roll < tier4_chance + tier3_chance + tier2_chance {
            // Tier 2 - Uncommon
            match rng.random_range(0..15) {
                0 => Self::DownyWoodpecker,
                1 => Self::HairyWoodpecker,
                2 => Self::PileatedWoodpecker,
                3 => Self::RedHeadedWoodpecker,
                4 => Self::RubyThroatedHummingbird,
                5 => Self::PurpleFinch,
                6 => Self::IndianaBunting,
                7 => Self::RoseBreastedGrosbeak,
                8 => Self::WoodThrush,
                9 => Self::Catbird,
                10 => Self::ScarletTanager,
                11 => Self::BaltimoreOriole,
                12 => Self::WinterWren,
                13 => Self::BrownCreeper,
                _ => Self::YellowBelledSapsucker,
            }
        } else {
            // Tier 1 - Common
            match rng.random_range(0..20) {
                0 => Self::Cardinal,
                1 => Self::BlueJay,
                2 => Self::Robin,
                3 => Self::Sparrow,
                4 => Self::Chickadee,
                5 => Self::Goldfinch,
                6 => Self::NorthernMockingbird,
                7 => Self::RedWingedBlackbird,
                8 => Self::CommonGrackle,
                9 => Self::BrownThrasher,
                10 => Self::CedarWaxwing,
                11 => Self::WhiteBreastedNuthatch,
                12 => Self::TuftedTitmouse,
                13 => Self::CarolinaWren,
                14 => Self::HouseFinch,
                15 => Self::EuropeanStarling,
                16 => Self::MourningDove,
                17 => Self::CommonCrow,
                18 => Self::BlueGrayGnatcatcher,
                _ => Self::YellowWarbler,
            }
        }
    }
    
    pub fn rarity_tier(&self) -> u8 {
        match self {
            // Tier 1 - Common
            Self::Cardinal | Self::BlueJay | Self::Robin | Self::Sparrow | Self::Chickadee |
            Self::Goldfinch | Self::NorthernMockingbird | Self::RedWingedBlackbird | 
            Self::CommonGrackle | Self::BrownThrasher | Self::CedarWaxwing | 
            Self::WhiteBreastedNuthatch | Self::TuftedTitmouse | Self::CarolinaWren |
            Self::HouseFinch | Self::EuropeanStarling | Self::MourningDove | Self::CommonCrow |
            Self::BlueGrayGnatcatcher | Self::YellowWarbler => 1,
            
            // Tier 2 - Uncommon
            Self::DownyWoodpecker | Self::HairyWoodpecker | Self::PileatedWoodpecker |
            Self::RedHeadedWoodpecker | Self::RubyThroatedHummingbird | Self::PurpleFinch |
            Self::IndianaBunting | Self::RoseBreastedGrosbeak | Self::WoodThrush |
            Self::Catbird | Self::ScarletTanager | Self::BaltimoreOriole | Self::WinterWren |
            Self::BrownCreeper | Self::YellowBelledSapsucker => 2,
            
            // Tier 3 - Rare
            Self::RedTailedHawk | Self::CoopersHawk | Self::GreatHornedOwl | Self::BarredOwl |
            Self::EasternBluebird | Self::BelttedKingfisher | Self::GrandSlamAmerican |
            Self::PaintedBunting | Self::CeruleanWarbler | Self::HoodedWarbler => 3,
            
            // Tier 4 - Legendary
            Self::BaldEagle | Self::PeregrineFalcon | Self::ProthonotaryWarbler |
            Self::KentuckyWarbler | Self::GoldenWingedWarbler => 4,
        }
    }
    
    pub fn spawn_probability(&self) -> f32 {
        match self.rarity_tier() {
            1 => 1.0,    // Common - normal spawn rate
            2 => 0.3,    // Uncommon - 30% spawn rate
            3 => 0.1,    // Rare - 10% spawn rate
            4 => 0.02,   // Legendary - 2% spawn rate
            _ => 0.0,
        }
    }
    
    pub fn feeding_preferences(&self) -> Vec<FeederType> {
        match self {
            // Seed eaters
            Self::Cardinal | Self::BlueJay | Self::Sparrow | Self::Chickadee | Self::Goldfinch |
            Self::HouseFinch | Self::PurpleFinch | Self::IndianaBunting | Self::RoseBreastedGrosbeak => {
                vec![FeederType::Seed, FeederType::Ground]
            },
            
            // Suet specialists (woodpeckers)
            Self::DownyWoodpecker | Self::HairyWoodpecker | Self::PileatedWoodpecker | 
            Self::RedHeadedWoodpecker | Self::YellowBelledSapsucker => {
                vec![FeederType::Suet]
            },
            
            // Nectar specialists
            Self::RubyThroatedHummingbird => {
                vec![FeederType::Nectar]
            },
            
            // Fruit eaters
            Self::BrownThrasher | Self::WoodThrush | Self::Catbird | Self::ScarletTanager |
            Self::BaltimoreOriole | Self::PaintedBunting => {
                vec![FeederType::Fruit, FeederType::Ground]
            },
            
            // Ground foragers
            Self::Robin | Self::CommonGrackle | Self::EuropeanStarling | Self::CommonCrow |
            Self::MourningDove | Self::EasternBluebird => {
                vec![FeederType::Ground]
            },
            
            // Insectivores (rarely use feeders)
            Self::WhiteBreastedNuthatch | Self::TuftedTitmouse | Self::CarolinaWren |
            Self::BrownCreeper | Self::WinterWren | Self::BlueGrayGnatcatcher | Self::YellowWarbler |
            Self::CeruleanWarbler | Self::HoodedWarbler | Self::ProthonotaryWarbler |
            Self::KentuckyWarbler | Self::GoldenWingedWarbler => {
                vec![FeederType::Suet, FeederType::Seed]
            },
            
            // Mixed feeders
            Self::NorthernMockingbird | Self::RedWingedBlackbird | Self::CedarWaxwing => {
                vec![FeederType::Fruit, FeederType::Seed, FeederType::Ground]
            },
            
            // Raptors (no feeders - attracted by bird activity)
            Self::RedTailedHawk | Self::CoopersHawk | Self::GreatHornedOwl | Self::BarredOwl |
            Self::BaldEagle | Self::PeregrineFalcon => {
                vec![] // No direct feeding
            },
            
            // Water specialists  
            Self::BelttedKingfisher => {
                vec![] // Requires water features
            },
            
            // Game birds
            Self::GrandSlamAmerican => {
                vec![FeederType::Ground, FeederType::Seed]
            },
        }
    }

    fn color(&self) -> Color {
        match self {
            // Tier 1 - Common species
            Self::Cardinal => Color::srgb(0.8, 0.2, 0.2),
            Self::BlueJay => Color::srgb(0.2, 0.4, 0.8),
            Self::Robin => Color::srgb(0.6, 0.3, 0.1),
            Self::Sparrow => Color::srgb(0.5, 0.4, 0.3),
            Self::Chickadee => Color::srgb(0.3, 0.3, 0.3),
            Self::Goldfinch => Color::srgb(0.9, 0.8, 0.2),
            Self::NorthernMockingbird => Color::srgb(0.7, 0.7, 0.7),
            Self::RedWingedBlackbird => Color::srgb(0.1, 0.1, 0.1),
            Self::CommonGrackle => Color::srgb(0.2, 0.1, 0.3),
            Self::BrownThrasher => Color::srgb(0.6, 0.4, 0.2),
            Self::CedarWaxwing => Color::srgb(0.7, 0.6, 0.4),
            Self::WhiteBreastedNuthatch => Color::srgb(0.8, 0.8, 0.9),
            Self::TuftedTitmouse => Color::srgb(0.6, 0.6, 0.6),
            Self::CarolinaWren => Color::srgb(0.5, 0.3, 0.2),
            Self::HouseFinch => Color::srgb(0.7, 0.4, 0.4),
            Self::EuropeanStarling => Color::srgb(0.3, 0.3, 0.4),
            Self::MourningDove => Color::srgb(0.5, 0.4, 0.4),
            Self::CommonCrow => Color::srgb(0.1, 0.1, 0.1),
            Self::BlueGrayGnatcatcher => Color::srgb(0.4, 0.5, 0.6),
            Self::YellowWarbler => Color::srgb(0.8, 0.8, 0.3),
            
            // Tier 2 - Uncommon species
            Self::DownyWoodpecker => Color::srgb(0.9, 0.9, 0.9),
            Self::HairyWoodpecker => Color::srgb(0.8, 0.8, 0.8),
            Self::PileatedWoodpecker => Color::srgb(0.2, 0.2, 0.2),
            Self::RedHeadedWoodpecker => Color::srgb(0.8, 0.1, 0.1),
            Self::RubyThroatedHummingbird => Color::srgb(0.1, 0.7, 0.1),
            Self::PurpleFinch => Color::srgb(0.6, 0.3, 0.6),
            Self::IndianaBunting => Color::srgb(0.2, 0.3, 0.8),
            Self::RoseBreastedGrosbeak => Color::srgb(0.8, 0.2, 0.4),
            Self::WoodThrush => Color::srgb(0.6, 0.4, 0.3),
            Self::Catbird => Color::srgb(0.4, 0.4, 0.4),
            Self::ScarletTanager => Color::srgb(0.9, 0.1, 0.1),
            Self::BaltimoreOriole => Color::srgb(0.9, 0.5, 0.1),
            Self::WinterWren => Color::srgb(0.4, 0.3, 0.2),
            Self::BrownCreeper => Color::srgb(0.5, 0.4, 0.3),
            Self::YellowBelledSapsucker => Color::srgb(0.8, 0.8, 0.4),
            
            // Tier 3 - Rare species
            Self::RedTailedHawk => Color::srgb(0.6, 0.4, 0.3),
            Self::CoopersHawk => Color::srgb(0.5, 0.5, 0.5),
            Self::GreatHornedOwl => Color::srgb(0.4, 0.3, 0.2),
            Self::BarredOwl => Color::srgb(0.5, 0.4, 0.3),
            Self::EasternBluebird => Color::srgb(0.2, 0.4, 0.9),
            Self::BelttedKingfisher => Color::srgb(0.3, 0.5, 0.7),
            Self::GrandSlamAmerican => Color::srgb(0.4, 0.3, 0.2),
            Self::PaintedBunting => Color::srgb(0.8, 0.3, 0.6),
            Self::CeruleanWarbler => Color::srgb(0.2, 0.6, 0.9),
            Self::HoodedWarbler => Color::srgb(0.8, 0.8, 0.2),
            
            // Tier 4 - Legendary species
            Self::BaldEagle => Color::srgb(0.3, 0.2, 0.1),
            Self::PeregrineFalcon => Color::srgb(0.4, 0.4, 0.5),
            Self::ProthonotaryWarbler => Color::srgb(0.9, 0.7, 0.1),
            Self::KentuckyWarbler => Color::srgb(0.7, 0.8, 0.2),
            Self::GoldenWingedWarbler => Color::srgb(0.8, 0.7, 0.3),
        }
    }
    
    /// Returns preferred feeder types for this species, in order of preference
    pub fn preferred_feeders(&self) -> Vec<FeederType> {
        self.feeding_preferences()
    }
    
    /// Returns utility multiplier for a given feeder type (0.0-2.0)
    pub fn feeder_utility_modifier(&self, feeder_type: FeederType) -> f32 {
        let preferences = self.preferred_feeders();
        if let Some(index) = preferences.iter().position(|&ft| ft as u8 == feeder_type as u8) {
            match index {
                0 => 1.5,  // Most preferred
                1 => 1.2,  // Second preference
                2 => 1.0,  // Third preference
                _ => 0.8,  // Lower preference
            }
        } else {
            0.3  // Not preferred, but will still use if desperate
        }
    }
    
    /// Returns true if this species is typically a year-round resident
    pub fn is_year_round_resident(&self) -> bool {
        match self {
            Self::Cardinal | Self::BlueJay | Self::Sparrow | Self::Chickadee |
            Self::WhiteBreastedNuthatch | Self::TuftedTitmouse | Self::CarolinaWren |
            Self::DownyWoodpecker | Self::HairyWoodpecker | Self::RedHeadedWoodpecker |
            Self::CommonCrow | Self::NorthernMockingbird | Self::RedTailedHawk |
            Self::GreatHornedOwl | Self::BelttedKingfisher => true,
            _ => false,
        }
    }
    
    /// Returns true if this species typically breeds in summer in this region
    pub fn is_summer_breeder(&self) -> bool {
        match self {
            Self::RubyThroatedHummingbird | Self::BaltimoreOriole | Self::ScarletTanager |
            Self::RoseBreastedGrosbeak | Self::ProthonotaryWarbler | Self::KentuckyWarbler |
            Self::GoldenWingedWarbler | Self::EasternBluebird | Self::WoodThrush => true,
            _ => self.is_year_round_resident(),
        }
    }
    
    /// Returns true if this species is present during winter
    pub fn is_winter_resident(&self) -> bool {
        match self {
            Self::CommonGrackle | Self::EuropeanStarling | Self::MourningDove |
            Self::HouseFinch | Self::PurpleFinch | Self::CedarWaxwing |
            Self::YellowBelledSapsucker => true,
            _ => self.is_year_round_resident(),
        }
    }
    
    /// Returns seasonal feeding preference multiplier for a feeder type
    pub fn seasonal_feeding_modifier(&self, feeder_type: FeederType, season: crate::environment::components::Season) -> f32 {
        use crate::environment::components::Season;
        use crate::feeder::FeederType;
        
        match season {
            Season::Spring => {
                match (self, feeder_type) {
                    // Spring nesting season - higher protein needs (suet preference)
                    (Self::Robin | Self::Sparrow | Self::Chickadee, FeederType::Suet) => 1.3,
                    // Early migrants need high energy
                    (Self::RedWingedBlackbird | Self::MourningDove, FeederType::Ground) => 1.4,
                    // Hummingbirds return - nectar critical
                    (Self::RubyThroatedHummingbird, FeederType::Nectar) => 2.0,
                    _ => 1.0,
                }
            },
            Season::Summer => {
                match (self, feeder_type) {
                    // Hot weather - increased fruit/water preference
                    (Self::BrownThrasher | Self::Catbird | Self::BaltimoreOriole, FeederType::Fruit) => 1.3,
                    // Peak hummingbird season
                    (Self::RubyThroatedHummingbird, FeederType::Nectar) => 1.5,
                    // Less seed feeding during abundant insect season
                    (_, FeederType::Seed) => 0.8,
                    _ => 1.0,
                }
            },
            Season::Fall => {
                match (self, feeder_type) {
                    // Migration fattening - prefer high-energy foods
                    (Self::WhiteBreastedNuthatch | Self::Chickadee | Self::TuftedTitmouse, FeederType::Suet) => 1.4,
                    // Seed stockpiling behavior
                    (Self::BlueJay | Self::CommonCrow, FeederType::Seed) => 1.3,
                    // Late fruits
                    (Self::CedarWaxwing, FeederType::Fruit) => 1.6,
                    _ => 1.0,
                }
            },
            Season::Winter => {
                match (self, feeder_type) {
                    // Winter survival - high calorie needs
                    (Self::Cardinal | Self::BlueJay | Self::Chickadee, FeederType::Suet) => 1.5,
                    (Self::Cardinal | Self::Goldfinch | Self::HouseFinch, FeederType::Seed) => 1.4,
                    // Ground feeding more difficult in winter
                    (_, FeederType::Ground) => 0.6,
                    _ => 1.0,
                }
            },
        }
    }
    
    /// Returns time-of-day feeding preference modifier
    pub fn time_based_feeding_modifier(&self, feeder_type: FeederType, hour: f32) -> f32 {
        use crate::feeder::FeederType;
        
        match self {
            // Early morning feeders (dawn chorus participants)
            Self::Robin | Self::Cardinal | Self::Sparrow => {
                if hour >= 5.0 && hour <= 8.0 {
                    1.3 // 30% boost during dawn hours
                } else if hour >= 18.0 && hour <= 20.0 {
                    1.2 // 20% boost during evening
                } else {
                    0.9 // Slightly less active during day
                }
            },
            
            // Hummingbirds need frequent feeding throughout the day
            Self::RubyThroatedHummingbird => {
                if matches!(feeder_type, FeederType::Nectar) {
                    if hour >= 6.0 && hour <= 19.0 {
                        1.0 + 0.3 * (hour - 12.5).abs() / 6.5 // Peak at dawn/dusk
                    } else {
                        0.3 // Less active at night
                    }
                } else {
                    1.0
                }
            },
            
            // Woodpeckers most active mid-morning
            Self::DownyWoodpecker | Self::HairyWoodpecker | Self::PileatedWoodpecker => {
                if hour >= 8.0 && hour <= 12.0 {
                    1.4 // Very active mid-morning
                } else if hour >= 14.0 && hour <= 17.0 {
                    1.2 // Active afternoon
                } else {
                    0.8
                }
            },
            
            // Evening feeders (gathering before roost)
            Self::CommonGrackle | Self::EuropeanStarling | Self::RedWingedBlackbird => {
                if hour >= 16.0 && hour <= 19.0 {
                    1.4 // Very active before roosting
                } else if hour >= 7.0 && hour <= 11.0 {
                    1.1 // Moderately active morning
                } else {
                    0.8
                }
            },
            
            // All-day feeders
            Self::BlueJay | Self::Chickadee | Self::WhiteBreastedNuthatch => {
                if hour >= 6.0 && hour <= 18.0 {
                    1.0 // Consistent activity
                } else {
                    0.5 // Reduced nighttime activity
                }
            },
            
            _ => 1.0, // Default no modifier
        }
    }
    
    /// Returns competitive feeding behavior traits
    pub fn feeding_aggression_level(&self) -> f32 {
        match self {
            // Highly aggressive at feeders
            Self::BlueJay | Self::CommonCrow | Self::PileatedWoodpecker => 0.9,
            Self::RedWingedBlackbird | Self::CommonGrackle => 0.8,
            
            // Moderately aggressive
            Self::Cardinal | Self::DownyWoodpecker | Self::HairyWoodpecker => 0.6,
            Self::NorthernMockingbird | Self::BrownThrasher => 0.6,
            
            // Mildly competitive
            Self::HouseFinch | Self::PurpleFinch | Self::Goldfinch => 0.4,
            Self::Chickadee | Self::TuftedTitmouse | Self::WhiteBreastedNuthatch => 0.4,
            
            // Generally peaceful
            Self::MourningDove | Self::Sparrow | Self::CedarWaxwing => 0.2,
            Self::RubyThroatedHummingbird => 0.1, // Too small to compete with most species
            
            // Non-competitive (raptors, insectivores)
            _ => 0.1,
        }
    }
    
    /// Returns feeding style characteristics
    pub fn feeding_style_traits(&self) -> (f32, f32, f32) {
        // Returns: (feeding_duration, feeding_frequency, group_tolerance)
        match self {
            // Quick, frequent, solitary feeders
            Self::RubyThroatedHummingbird => (0.2, 0.9, 0.1),
            Self::WhiteBreastedNuthatch | Self::BrownCreeper => (0.3, 0.8, 0.2),
            
            // Medium duration, frequent, social feeders  
            Self::Chickadee | Self::TuftedTitmouse => (0.4, 0.7, 0.8),
            Self::Goldfinch | Self::HouseFinch | Self::PurpleFinch => (0.5, 0.6, 0.9),
            
            // Longer feeding sessions, social
            Self::Cardinal | Self::BlueJay => (0.7, 0.5, 0.6),
            Self::Sparrow | Self::MourningDove => (0.6, 0.6, 0.9),
            
            // Extended feeding, flock-oriented
            Self::CommonGrackle | Self::EuropeanStarling | Self::RedWingedBlackbird => (0.8, 0.4, 0.9),
            Self::CedarWaxwing => (0.9, 0.3, 0.9), // Highly social, long feeding
            
            // Variable based on food availability
            Self::Robin | Self::BrownThrasher | Self::NorthernMockingbird => (0.6, 0.5, 0.5),
            
            _ => (0.5, 0.5, 0.5), // Default moderate values
        }
    }
    
    /// Returns specialized feeding technique preferences
    pub fn feeding_technique_preference(&self, feeder_type: FeederType) -> f32 {
        use crate::feeder::FeederType;
        
        match (self, feeder_type) {
            // Hovering specialists
            (Self::RubyThroatedHummingbird, FeederType::Nectar) => 2.0, // Exclusive hover feeding
            
            // Clinging specialists (can feed upside down)
            (Self::WhiteBreastedNuthatch | Self::DownyWoodpecker | Self::HairyWoodpecker, FeederType::Suet) => 1.8,
            (Self::Chickadee | Self::TuftedTitmouse, FeederType::Seed) => 1.4, // Good at clinging to seed feeders
            
            // Ground foraging specialists
            (Self::Robin | Self::Sparrow | Self::MourningDove, FeederType::Ground) => 1.6,
            (Self::CommonCrow | Self::CommonGrackle | Self::EuropeanStarling, FeederType::Ground) => 1.5,
            
            // Platform/perch feeders
            (Self::Cardinal | Self::BlueJay, FeederType::Seed) => 1.3,
            (Self::Goldfinch | Self::HouseFinch | Self::PurpleFinch, FeederType::Seed) => 1.4,
            
            // Fruit handling specialists
            (Self::BrownThrasher | Self::Catbird | Self::BaltimoreOriole, FeederType::Fruit) => 1.5,
            (Self::CedarWaxwing | Self::ScarletTanager, FeederType::Fruit) => 1.6,
            
            _ => 1.0, // Standard feeding technique
        }
    }
}

#[derive(Component)]
pub struct Bird {
    pub species: BirdSpecies,
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
struct WanderTimer(Timer);

fn handle_spawn_events(
    mut commands: Commands,
    mut events: EventReader<SpawnBirdEvent>,
    mut bird_count: ResMut<BirdCount>,
) {
    for _ in events.read() {
        spawn_bird(&mut commands);
        bird_count.0 += 1;
    }
}

fn spawn_initial_birds(mut commands: Commands) {
    // Spawn 3-5 initial birds to populate the world
    for _ in 0..4 {
        spawn_bird(&mut commands);
    }
}

fn spawn_bird(commands: &mut Commands) {
    let species = BirdSpecies::random();
    let mut rng = rand::rng();
    
    let x = rng.random_range(-400.0..400.0);
    let y = rng.random_range(-300.0..300.0);
    
    // Spawn bird entity with basic components first
    let bird_entity = commands.spawn((
        Sprite {
            image: Handle::default(), // Will be set by animation system
            texture_atlas: None,      // Will be set by animation system
            ..default()
        },
        Transform::from_xyz(x, y, 1.0),
        RigidBody::Dynamic,
        Collider::ball(10.0),
        Bird { species },
        Velocity(Vec2::ZERO),
        WanderTimer(Timer::from_seconds(2.0, TimerMode::Repeating)),
        GravityScale(0.0),
        Damping { linear_damping: 2.0, angular_damping: 10.0 },
    )).id();
    
    // Add AI components in a separate bundle
    commands.entity(bird_entity).insert((
        BirdAI,
        BirdState::Wandering,
        Blackboard {
            internal: InternalState {
                hunger: rng.random_range(0.2..0.8),
                thirst: rng.random_range(0.2..0.8),
                energy: rng.random_range(0.5..1.0),
                social_need: rng.random_range(0.2..0.6),
                territorial_stress: rng.random_range(0.3..0.5),
                fear: 0.0,
            },
            ..default()
        },
    ));
    
    // Add social behavior components
    commands.entity(bird_entity).insert((
        SocialBirdTraits {
            dominance_level: rng.random_range(0.2..0.8),
            territorial_aggression: get_species_territorial_aggression(species).clamp(0.0, 1.0),
            social_tolerance: get_species_social_tolerance(species).clamp(0.0, 1.0),
            mating_receptivity: rng.random_range(0.3..0.7),
            flock_tendency: get_species_flock_tendency(species).clamp(0.0, 1.0),
        },
        SocialRelationships::default(),
    ));
    
    // Add foraging behavior components
    commands.entity(bird_entity).insert((
        ForagingTraits {
            foraging_style: get_species_foraging_style(species),
            ground_preference: get_species_ground_preference(species),
            cache_tendency: get_species_cache_tendency(species),
            search_pattern: get_species_search_pattern(species),
            hover_ability: get_species_hover_ability(species),
        },
        CacheData {
            cached_locations: Vec::new(),
            retrieval_memory: get_species_cache_memory(species),
            current_cache_count: 0,
            max_cache_capacity: get_species_max_cache(species),
        },
        ForagingState::default(),
    ));
    
    // Add animation components
    commands.entity(bird_entity).insert((
        AnimatedBird { species },
        crate::animation::components::AnimationController::default(),
        crate::animation::components::AnimationLibrary::default(),
    ));
}

fn bird_movement(
    mut bird_query: Query<(&mut Velocity, &mut Transform), With<Bird>>,
    feeder_query: Query<&Transform, (With<crate::feeder::Feeder>, Without<Bird>)>,
    config: Res<GameConfig>,
    time: Res<Time>,
) {
    for (mut velocity, mut transform) in bird_query.iter_mut() {
        let mut force = Vec2::ZERO;
        
        // Attraction to feeders
        for feeder_transform in feeder_query.iter() {
            let distance = transform.translation.truncate().distance(feeder_transform.translation.truncate());
            if distance < config.bird_attraction_radius {
                let direction = (feeder_transform.translation.truncate() - transform.translation.truncate()).normalize();
                let strength = (config.bird_attraction_radius - distance) / config.bird_attraction_radius;
                force += direction * strength * config.bird_attraction_force;
            }
        }
        
        // Simple obstacle avoidance (removed Rapier context usage)
        let mut avoidance_force = Vec2::ZERO;
        for other_feeder in feeder_query.iter() {
            let distance = transform.translation.truncate().distance(other_feeder.translation.truncate());
            if distance < config.bird_avoidance_radius && distance > 0.1 {
                let direction = (transform.translation.truncate() - other_feeder.translation.truncate()).normalize();
                avoidance_force += direction * (config.bird_avoidance_radius - distance) / config.bird_avoidance_radius;
            }
        }
        force += avoidance_force * config.bird_attraction_force;
        
        // Apply wandering velocity
        let target_velocity = velocity.0 + force * time.delta().as_secs_f32();
        let max_speed = config.bird_wander_speed;
        if target_velocity.length() > max_speed {
            velocity.0 = target_velocity.normalize() * max_speed;
        } else {
            velocity.0 = target_velocity;
        }
        
        // Update transform
        transform.translation += velocity.0.extend(0.0) * time.delta().as_secs_f32();
    }
}

fn update_wander_timer(
    mut bird_query: Query<(&mut Velocity, &mut WanderTimer), With<Bird>>,
    config: Res<GameConfig>,
    time: Res<Time>,
) {
    let mut rng = rand::rng();
    
    for (mut velocity, mut timer) in bird_query.iter_mut() {
        timer.0.tick(time.delta());
        
        if timer.0.just_finished() {
            let angle = rng.random_range(0.0..std::f32::consts::TAU);
            let speed = config.bird_wander_speed * rng.random_range(0.3..1.0);
            velocity.0 = Vec2::new(angle.cos(), angle.sin()) * speed;
        }
    }
}

pub fn environmental_bird_spawning_system(
    mut commands: Commands,
    bird_count: Res<BirdCount>,
    time_state: Res<TimeState>,
    weather_state: Res<WeatherState>,
    seasonal_state: Res<SeasonalState>,
    time: Res<Time>,
    bird_registry: Res<crate::bird_data::BirdDataRegistry>,
    education_data: Res<BirdEducationData>,
) {
    // Environmental spawning logic
    let season = time_state.get_season();
    let base_activity = season.bird_activity_modifier();
    let weather_activity = weather_state.current_weather.bird_activity_modifier();
    let time_activity = time_state.daylight_factor();
    let song_activity = time_state.song_period_activity(); // Dawn chorus and evening song multiplier
    
    let spawn_chance = base_activity * weather_activity * time_activity * song_activity * 0.001; // Base spawn rate per frame
    
    // Only spawn if we're under the bird limit and conditions are favorable
    if bird_count.0 < 15 && rand::rng().random::<f32>() < spawn_chance {
        spawn_seasonal_bird(&mut commands, &seasonal_state, &bird_registry, &education_data, season);
    }
}

fn spawn_seasonal_bird(
    commands: &mut Commands, 
    seasonal_state: &SeasonalState,
    bird_registry: &crate::bird_data::BirdDataRegistry,
    education_data: &BirdEducationData,
    season: Season,
) {
    let mut rng = rand::rng();
    
    // Select species based on seasonal availability and migration data
    let available_species: Vec<(BirdSpecies, f32)> = seasonal_state.available_species.iter()
        .map(|(species, _)| {
            let mut probability = bird_registry.get_spawn_probability(species, season);
            
            // Apply migration logic based on education data
            if let Some(migration_data) = education_data.migration_data.get(species) {
                probability *= get_migration_availability(*species, migration_data, season);
            }
            
            (*species, probability)
        })
        .filter(|(_, prob)| *prob > 0.0) // Only include species available this season
        .collect();
    
    if available_species.is_empty() {
        return; // No species available
    }
    
    // Weighted random selection
    let total_weight: f32 = available_species.iter().map(|(_, weight)| weight).sum();
    let mut random_weight = rng.random::<f32>() * total_weight;
    
    let mut selected_species = available_species[0].0;
    for (species, weight) in available_species {
        random_weight -= weight;
        if random_weight <= 0.0 {
            selected_species = species;
            break;
        }
    }
    
    // Spawn the selected bird
    spawn_specific_bird(commands, selected_species);
}

fn spawn_specific_bird(commands: &mut Commands, species: BirdSpecies) {
    let mut rng = rand::rng();
    
    let x = rng.random_range(-400.0..400.0);
    let y = rng.random_range(-300.0..300.0);
    
    commands.spawn((
        Sprite {
            image: Handle::default(), // Will be set by animation system
            texture_atlas: Some(TextureAtlas {
                layout: Handle::default(), // Will be set by animation system
                index: 0, // Will be updated by animation system
            }),
            ..default()
        },
        Transform::from_xyz(x, y, 1.0),
        RigidBody::Dynamic,
        Collider::ball(10.0),
        Bird { species },
        Velocity(Vec2::ZERO),
        WanderTimer(Timer::from_seconds(2.0, TimerMode::Repeating)),
        GravityScale(0.0),
        Damping { linear_damping: 2.0, angular_damping: 10.0 },
        // AI Components
        BirdAI,
        BirdState::Wandering,
        Blackboard {
            internal: InternalState {
                hunger: rng.random_range(0.2..0.8),
                thirst: rng.random_range(0.2..0.8),
                energy: rng.random_range(0.5..1.0),
                social_need: rng.random_range(0.2..0.6),
                territorial_stress: rng.random_range(0.3..0.5),                
                fear: 0.0,
            },
            world_knowledge: crate::bird_ai::components::WorldKnowledge::default(),
            current_target: None,
        },
        // Animation components
        AnimatedBird {
            species,
        },
        crate::animation::components::AnimationController::default(),
        crate::animation::components::AnimationLibrary::default(),
    ));
}

/// Determines migration availability based on education data and current season
fn get_migration_availability(
    species: BirdSpecies,
    migration_data: &crate::journal::resources::MigrationData,
    season: Season,
) -> f32 {
    // Non-migratory species are always available (but still subject to seasonal variations)
    if !migration_data.is_migratory {
        return 1.0;
    }
    
    // Parse migration timing to determine seasonal availability
    let timing = migration_data.migration_timing.to_lowercase();
    
    match season {
        Season::Spring => {
            if timing.contains("spring") || timing.contains("march") || 
               timing.contains("april") || timing.contains("may") {
                1.2 // Higher chance during migration season
            } else if species.is_year_round_resident() {
                1.0 // Year-round residents
            } else {
                0.3 // Low chance if not migration season
            }
        },
        Season::Summer => {
            // Most migrants should be in breeding range during summer
            if species.is_summer_breeder() {
                1.0
            } else if species.is_year_round_resident() {
                1.0
            } else {
                0.2 // Very low chance for winter-only species
            }
        },
        Season::Fall => {
            if timing.contains("fall") || timing.contains("september") || 
               timing.contains("october") || timing.contains("november") {
                1.2 // Higher chance during fall migration
            } else if species.is_year_round_resident() {
                1.0
            } else {
                0.4
            }
        },
        Season::Winter => {
            if species.is_winter_resident() || species.is_year_round_resident() {
                1.0
            } else {
                0.1 // Very low chance for summer migrants
            }
        }
    }
}

// Species-specific social trait functions
fn get_species_territorial_aggression(species: BirdSpecies) -> f32 {
    use BirdSpecies::*;
    match species {
        // High territorial aggression
        BlueJay | NorthernMockingbird | RedWingedBlackbird | Cardinal => 0.8,
        RedHeadedWoodpecker | BelttedKingfisher | RedTailedHawk | CoopersHawk => 0.9,
        PeregrineFalcon | BaldEagle => 0.95,
        
        // Medium territorial aggression
        Robin | ScarletTanager | BaltimoreOriole => 0.6,
        DownyWoodpecker | HairyWoodpecker | PileatedWoodpecker => 0.65,
        EasternBluebird | PaintedBunting => 0.55,
        
        // Low territorial aggression (more social/flock-oriented)
        Goldfinch | HouseFinch | PurpleFinch | CommonGrackle => 0.3,
        Chickadee | TuftedTitmouse | WhiteBreastedNuthatch => 0.35,
        CedarWaxwing | YellowWarbler | IndianaBunting => 0.4,
        
        // Very low (highly social)
        Sparrow | EuropeanStarling => 0.2,
        
        _ => 0.5, // Default medium aggression
    }
}

fn get_species_social_tolerance(species: BirdSpecies) -> f32 {
    use BirdSpecies::*;
    match species {
        // Highly social (high tolerance for others)
        Goldfinch | HouseFinch | CommonGrackle | EuropeanStarling => 0.9,
        Chickadee | TuftedTitmouse | CedarWaxwing => 0.85,
        Sparrow | PurpleFinch => 0.8,
        
        // Moderately social
        Robin | BlueJay | WhiteBreastedNuthatch | YellowWarbler => 0.6,
        DownyWoodpecker | HairyWoodpecker | CarolinaWren => 0.65,
        
        // Less social (lower tolerance)
        Cardinal | NorthernMockingbird | RedWingedBlackbird => 0.4,
        BrownThrasher | PaintedBunting | ScarletTanager => 0.45,
        
        // Solitary (very low tolerance)
        RedTailedHawk | CoopersHawk | BarredOwl | GreatHornedOwl => 0.2,
        BelttedKingfisher | PeregrineFalcon | BaldEagle => 0.1,
        
        _ => 0.5, // Default medium tolerance
    }
}

fn get_species_flock_tendency(species: BirdSpecies) -> f32 {
    use BirdSpecies::*;
    match species {
        // High flocking tendency (mixed species flocks)
        Goldfinch | HouseFinch | PurpleFinch | CommonGrackle => 0.9,
        Chickadee | TuftedTitmouse | WhiteBreastedNuthatch => 0.85,
        Sparrow | EuropeanStarling | CedarWaxwing => 0.8,
        
        // Moderate flocking (winter flocks, feeding aggregations)
        Robin | Cardinal | BlueJay | YellowWarbler => 0.6,
        DownyWoodpecker | HairyWoodpecker | CarolinaWren => 0.5,
        
        // Low flocking (mostly solitary or pairs)
        NorthernMockingbird | RedWingedBlackbird | BrownThrasher => 0.3,
        PaintedBunting | ScarletTanager | BaltimoreOriole => 0.4,
        
        // Minimal flocking (highly territorial/solitary)
        RedTailedHawk | CoopersHawk | BarredOwl | GreatHornedOwl => 0.1,
        BelttedKingfisher | PeregrineFalcon | BaldEagle => 0.05,
        
        _ => 0.5, // Default medium flocking
    }
}

// Species-specific foraging trait functions
fn get_species_foraging_style(species: BirdSpecies) -> crate::bird_ai::components::ForagingStyle {
    use BirdSpecies::*;
    use crate::bird_ai::components::ForagingStyle;
    match species {
        // Methodical foragers
        Robin | BrownThrasher | WoodThrush => ForagingStyle::Methodical,
        DownyWoodpecker | HairyWoodpecker | RedHeadedWoodpecker => ForagingStyle::Methodical,
        
        // Specialists
        RubyThroatedHummingbird | BelttedKingfisher => ForagingStyle::Specialist,
        RedTailedHawk | CoopersHawk | PeregrineFalcon | BaldEagle => ForagingStyle::Specialist,
        
        // Scatter foragers (highly mobile)
        Goldfinch | PurpleFinch | CedarWaxwing => ForagingStyle::Scatter,
        YellowWarbler | CeruleanWarbler | HoodedWarbler => ForagingStyle::Scatter,
        
        // Opportunistic (default for most species)
        _ => ForagingStyle::Opportunistic,
    }
}

fn get_species_ground_preference(species: BirdSpecies) -> f32 {
    use BirdSpecies::*;
    match species {
        // High ground preference (primarily ground feeders)
        Robin | BrownThrasher | WoodThrush => 0.9,
        MourningDove | CommonGrackle | RedWingedBlackbird => 0.8,
        Sparrow | EuropeanStarling | CommonCrow => 0.85,
        
        // Moderate ground feeding
        Cardinal | BlueJay | NorthernMockingbird => 0.6,
        TuftedTitmouse | CarolinaWren | WhiteBreastedNuthatch => 0.5,
        
        // Low ground preference (primarily arboreal)
        Chickadee | CedarWaxwing | BaltimoreOriole => 0.3,
        DownyWoodpecker | HairyWoodpecker | RedHeadedWoodpecker => 0.2,
        
        // Minimal ground feeding
        RubyThroatedHummingbird => 0.1,
        RedTailedHawk | CoopersHawk | PeregrineFalcon | BaldEagle => 0.05,
        
        _ => 0.4, // Default moderate ground feeding
    }
}

fn get_species_cache_tendency(species: BirdSpecies) -> f32 {
    use BirdSpecies::*;
    match species {
        // High caching tendency
        BlueJay | CommonCrow => 0.9,
        WhiteBreastedNuthatch | BrownCreeper => 0.85,
        
        // Moderate caching
        Chickadee | TuftedTitmouse => 0.6,
        DownyWoodpecker | HairyWoodpecker => 0.5,
        
        // Low caching
        Cardinal | HouseFinch | PurpleFinch => 0.3,
        Robin | BrownThrasher => 0.2,
        
        // No caching (primarily nectar feeders or hunters)
        RubyThroatedHummingbird => 0.0,
        RedTailedHawk | CoopersHawk | PeregrineFalcon | BaldEagle => 0.05,
        
        _ => 0.2, // Default low caching
    }
}

fn get_species_search_pattern(species: BirdSpecies) -> crate::bird_ai::components::SearchPattern {
    use BirdSpecies::*;
    use crate::bird_ai::components::SearchPattern;
    match species {
        // Grid searchers (systematic)
        Robin | BrownThrasher | WoodThrush => SearchPattern::Grid,
        DownyWoodpecker | HairyWoodpecker => SearchPattern::Grid,
        
        // Spiral searchers (expanding outward)
        BlueJay | CommonCrow | NorthernMockingbird => SearchPattern::Spiral,
        Cardinal | TuftedTitmouse => SearchPattern::Spiral,
        
        // Linear searchers (back and forth)
        RedWingedBlackbird | CommonGrackle => SearchPattern::Linear,
        BeltedKingfisher => SearchPattern::Linear,
        
        // Random foragers
        _ => SearchPattern::Random,
    }
}

fn get_species_hover_ability(species: BirdSpecies) -> f32 {
    use BirdSpecies::*;
    match species {
        // Excellent hoverers
        RubyThroatedHummingbird => 1.0,
        
        // Good hoverers
        BelttedKingfisher => 0.8,
        PeregrineFalcon => 0.7,
        
        // Moderate hovering (brief hover capability)
        Goldfinch | PurpleFinch | CedarWaxwing => 0.4,
        YellowWarbler | BaltimoreOriole => 0.3,
        
        // Limited hovering
        Chickadee | TuftedTitmouse => 0.2,
        
        // No hovering ability
        _ => 0.0,
    }
}

fn get_species_cache_memory(species: BirdSpecies) -> f32 {
    use BirdSpecies::*;
    match species {
        // Excellent memory
        BlueJay | CommonCrow => 0.95,
        WhiteBreastedNuthatch | BrownCreeper => 0.9,
        
        // Good memory
        Chickadee | TuftedTitmouse => 0.8,
        DownyWoodpecker | HairyWoodpecker => 0.75,
        
        // Moderate memory
        Cardinal | HouseFinch => 0.6,
        
        // Poor memory (don't cache much anyway)
        _ => 0.4,
    }
}

fn get_species_max_cache(species: BirdSpecies) -> u32 {
    use BirdSpecies::*;
    match species {
        // High cache capacity
        BlueJay | CommonCrow => 15,
        WhiteBreastedNuthatch | BrownCreeper => 12,
        
        // Moderate cache capacity
        Chickadee | TuftedTitmouse => 8,
        DownyWoodpecker | HairyWoodpecker => 6,
        
        // Low cache capacity
        Cardinal | HouseFinch => 3,
        
        // No caching
        RubyThroatedHummingbird | RedTailedHawk | CoopersHawk | PeregrineFalcon | BaldEagle => 0,
        
        _ => 2, // Default minimal caching
    }
}