use bevy::prelude::*;
use crate::bird::BirdSpecies;

#[derive(Component)]
pub struct FlockMember {
    pub flock_id: Option<Entity>,
    pub social_distance: f32,      // Preferred distance from other birds
    pub flocking_strength: f32,    // How strongly this bird follows flocking rules
}

impl Default for FlockMember {
    fn default() -> Self {
        Self {
            flock_id: None,
            social_distance: 30.0,
            flocking_strength: 0.7,
        }
    }
}

#[derive(Component)]
pub struct Flock {
    pub species: BirdSpecies,
    pub members: Vec<Entity>,
    pub center: Vec2,
    pub max_size: usize,
}

impl Flock {
    pub fn new(species: BirdSpecies) -> Self {
        Self {
            species,
            members: Vec::new(),
            center: Vec2::ZERO,
            max_size: species.max_flock_size(),
        }
    }
}

#[derive(Component)]
pub struct Territory {
    pub center: Vec2,
    pub radius: f32,
    pub species: BirdSpecies,
    pub aggression_level: f32,    // How aggressively this bird defends territory
}

impl Territory {
    pub fn new(center: Vec2, species: BirdSpecies) -> Self {
        Self {
            center,
            radius: species.territory_radius(),
            species,
            aggression_level: species.aggression_level(),
        }
    }
}

#[derive(Component)]
pub struct PredatorAvoidance {
    pub detection_radius: f32,
    pub panic_threshold: f32,
    pub escape_speed_multiplier: f32,
}

impl Default for PredatorAvoidance {
    fn default() -> Self {
        Self {
            detection_radius: 200.0,
            panic_threshold: 0.3,
            escape_speed_multiplier: 2.0,
        }
    }
}

// Extensions to BirdSpecies for flocking behavior
impl BirdSpecies {
    pub fn max_flock_size(&self) -> usize {
        match self {
            // Highly social species
            Self::EuropeanStarling | Self::CommonGrackle | Self::CommonCrow => 12,
            Self::Goldfinch | Self::HouseFinch => 8,
            Self::Sparrow | Self::Chickadee => 6,
            
            // Moderately social
            Self::RedWingedBlackbird | Self::CedarWaxwing => 4,
            Self::Robin | Self::MourningDove => 3,
            
            // Less social or territorial
            Self::Cardinal | Self::BlueJay => 2,
            Self::WhiteBreastedNuthatch | Self::TuftedTitmouse => 2,
            
            // Generally solitary
            _ => 1,
        }
    }
    
    pub fn territory_radius(&self) -> f32 {
        match self {
            // Highly territorial
            Self::BlueJay | Self::NorthernMockingbird => 150.0,
            Self::Cardinal | Self::Robin => 120.0,
            
            // Moderately territorial
            Self::RedWingedBlackbird | Self::CarolinaWren => 100.0,
            Self::WhiteBreastedNuthatch => 80.0,
            
            // Less territorial (social species)
            Self::Sparrow | Self::Chickadee | Self::Goldfinch => 50.0,
            Self::EuropeanStarling | Self::CommonGrackle => 30.0,
            
            _ => 60.0, // Default
        }
    }
    
    pub fn aggression_level(&self) -> f32 {
        match self {
            // Very aggressive
            Self::BlueJay | Self::NorthernMockingbird => 0.9,
            Self::RedWingedBlackbird => 0.8,
            
            // Moderately aggressive
            Self::Cardinal | Self::Robin => 0.6,
            Self::CommonCrow => 0.7,
            
            // Peaceful
            Self::Chickadee | Self::Goldfinch | Self::CedarWaxwing => 0.2,
            Self::MourningDove => 0.1,
            
            _ => 0.4, // Default moderate
        }
    }
    
    pub fn social_compatibility(&self, other: &BirdSpecies) -> f32 {
        // Same species - highly compatible
        if self == other {
            return 1.0;
        }
        
        match (self, other) {
            // Species that often feed together
            (Self::Chickadee, Self::WhiteBreastedNuthatch) | 
            (Self::WhiteBreastedNuthatch, Self::Chickadee) => 0.8,
            
            (Self::Goldfinch, Self::HouseFinch) | 
            (Self::HouseFinch, Self::Goldfinch) => 0.9,
            
            (Self::Sparrow, Self::HouseFinch) | 
            (Self::HouseFinch, Self::Sparrow) => 0.7,
            
            // Mixed flocks that are common
            (Self::Chickadee, Self::TuftedTitmouse) | 
            (Self::TuftedTitmouse, Self::Chickadee) => 0.8,
            
            // Species that avoid each other
            (Self::BlueJay, _) | (_, Self::BlueJay) => 0.3, // Blue jays often scare others
            (Self::CommonCrow, _) | (_, Self::CommonCrow) => 0.2, // Crows intimidate smaller birds
            
            // Small birds are generally wary of larger birds
            _ => {
                let size_diff = (self.size_category() as i32 - other.size_category() as i32).abs();
                match size_diff {
                    0 => 0.6, // Same size - neutral
                    1 => 0.4, // One size difference - cautious
                    _ => 0.2, // Large size difference - avoidance
                }
            }
        }
    }
    
    fn size_category(&self) -> u8 {
        match self {
            // Small birds (1)
            Self::Chickadee | Self::TuftedTitmouse | Self::WhiteBreastedNuthatch |
            Self::Goldfinch | Self::BlueGrayGnatcatcher => 1,
            
            // Medium-small birds (2)
            Self::Sparrow | Self::HouseFinch | Self::CarolinaWren | Self::YellowWarbler => 2,
            
            // Medium birds (3)
            Self::Cardinal | Self::Robin | Self::CedarWaxwing | Self::BrownThrasher => 3,
            
            // Large birds (4)
            Self::BlueJay | Self::CommonGrackle | Self::RedWingedBlackbird | 
            Self::NorthernMockingbird | Self::MourningDove => 4,
            
            // Very large birds (5)
            Self::CommonCrow | Self::EuropeanStarling => 5,
        }
    }
}