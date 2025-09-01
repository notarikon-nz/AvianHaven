use bevy::prelude::*;
use crate::bird::BirdSpecies;
use crate::bird_ai::components::BirdAction;

#[derive(Component)]
pub struct CatalogUI;

#[derive(Component)]
pub struct CatalogItem {
    pub item_type: PlaceableItemType,
}

#[derive(Component)]
pub struct CategoryButton {
    pub category: crate::catalog::resources::ItemCategory,
}

#[derive(Component)]
pub struct ItemCard {
    pub item_type: PlaceableItemType,
}

#[derive(Component)]
pub struct PurchaseButton {
    pub item_type: PlaceableItemType,
}

#[derive(Component)]
pub struct PlaceButton {
    pub item_type: PlaceableItemType,
}

#[derive(Component)]
pub struct CatalogContainer;

#[derive(Component)]
pub struct ItemsGrid;

#[derive(Component)]
pub struct PlaceableObject {
    pub item_type: PlaceableItemType,
    pub placement_cost: u32,
}

#[derive(Component)]
pub struct PlacementGhost;

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum PlaceableItemType {
    // Comfort items (like in Neko Atsume)
    CardboardBox,
    CushionRed,
    CushionBlue,
    WoodenPerch,
    FancyPerch,
    
    // Food items
    BasicBirdSeed,
    PremiumSeed,
    SuetCake,
    NectarFeeder,
    FruitDispenser,
    
    // Water features
    BasicBirdbath,
    FountainBirdbath,
    StreamFeature,
    
    // Decorative items
    GardenGnome,
    WindChime,
    FlowerPot,
    BirdHouse,
    NestingBox,
    
    // Special attraction items
    MirrorToy,
    BellToy,
    SwingSeat,
}

impl PlaceableItemType {
    pub fn name(&self) -> &str {
        match self {
            Self::CardboardBox => "Cardboard Box",
            Self::CushionRed => "Red Cushion",
            Self::CushionBlue => "Blue Cushion", 
            Self::WoodenPerch => "Wooden Perch",
            Self::FancyPerch => "Fancy Perch",
            Self::BasicBirdSeed => "Basic Bird Seed",
            Self::PremiumSeed => "Premium Seed Mix",
            Self::SuetCake => "Suet Cake",
            Self::NectarFeeder => "Nectar Feeder",
            Self::FruitDispenser => "Fruit Dispenser",
            Self::BasicBirdbath => "Basic Birdbath",
            Self::FountainBirdbath => "Fountain Birdbath",
            Self::StreamFeature => "Stream Feature",
            Self::GardenGnome => "Garden Gnome",
            Self::WindChime => "Wind Chime",
            Self::FlowerPot => "Flower Pot",
            Self::BirdHouse => "Bird House",
            Self::NestingBox => "Nesting Box",
            Self::MirrorToy => "Mirror Toy",
            Self::BellToy => "Bell Toy",
            Self::SwingSeat => "Swing Seat",
        }
    }
    
    pub fn price(&self) -> u32 {
        match self {
            // Comfort items
            Self::CardboardBox => 10,
            Self::CushionRed => 25,
            Self::CushionBlue => 25,
            Self::WoodenPerch => 50,
            Self::FancyPerch => 150,
            
            // Food items
            Self::BasicBirdSeed => 30,
            Self::PremiumSeed => 75,
            Self::SuetCake => 40,
            Self::NectarFeeder => 120,
            Self::FruitDispenser => 90,
            
            // Water features
            Self::BasicBirdbath => 80,
            Self::FountainBirdbath => 200,
            Self::StreamFeature => 500,
            
            // Decorative items
            Self::GardenGnome => 60,
            Self::WindChime => 45,
            Self::FlowerPot => 35,
            Self::BirdHouse => 100,
            Self::NestingBox => 120,
            
            // Special items
            Self::MirrorToy => 85,
            Self::BellToy => 70,
            Self::SwingSeat => 180,
        }
    }
    
    pub fn description(&self) -> &str {
        match self {
            Self::CardboardBox => "A simple box that birds love to explore",
            Self::CushionRed => "Soft red cushion for birds to rest on",
            Self::CushionBlue => "Soft blue cushion for birds to rest on",
            Self::WoodenPerch => "Natural wooden perch for roosting",
            Self::FancyPerch => "Ornate decorative perch with intricate carving",
            Self::BasicBirdSeed => "Standard seed mix attracting common birds",
            Self::PremiumSeed => "High-quality seed mix for rare species",
            Self::SuetCake => "High-energy suet cake for woodpeckers",
            Self::NectarFeeder => "Sweet nectar for hummingbirds",
            Self::FruitDispenser => "Fresh fruit for fruit-eating birds",
            Self::BasicBirdbath => "Simple water source for drinking and bathing",
            Self::FountainBirdbath => "Elegant fountain that attracts more birds",
            Self::StreamFeature => "Flowing stream feature with naturalistic appeal",
            Self::GardenGnome => "Decorative gnome that some birds find intriguing",
            Self::WindChime => "Melodic chimes that create ambient sound",
            Self::FlowerPot => "Colorful flowers that attract insects and birds",
            Self::BirdHouse => "Nesting house for cavity-dwelling species",
            Self::NestingBox => "Specialized nesting box for breeding pairs",
            Self::MirrorToy => "Reflective toy that fascinates certain species",
            Self::BellToy => "Small bell that birds enjoy playing with",
            Self::SwingSeat => "Fun swing that playful birds love to use",
        }
    }
    
    pub fn attracts_species(&self) -> Vec<BirdSpecies> {
        match self {
            Self::CardboardBox => vec![BirdSpecies::Sparrow, BirdSpecies::Chickadee],
            Self::CushionRed | Self::CushionBlue => vec![BirdSpecies::Cardinal, BirdSpecies::Robin],
            Self::WoodenPerch | Self::FancyPerch => vec![
                BirdSpecies::BlueJay, BirdSpecies::Cardinal, BirdSpecies::Robin
            ],
            Self::SuetCake => vec![
                BirdSpecies::DownyWoodpecker, BirdSpecies::HairyWoodpecker, BirdSpecies::WhiteBreastedNuthatch
            ],
            Self::NectarFeeder => vec![BirdSpecies::RubyThroatedHummingbird],
            Self::FruitDispenser => vec![
                BirdSpecies::BrownThrasher, BirdSpecies::ScarletTanager, BirdSpecies::BaltimoreOriole
            ],
            _ => vec![], // Most items attract various species generally
        }
    }

    /// Returns the bird actions this item provides
    pub fn provides_actions(&self) -> Vec<BirdAction> {
        match self {
            // Comfort items - provide perching/resting
            Self::CardboardBox => vec![BirdAction::Explore, BirdAction::Perch],
            Self::CushionRed | Self::CushionBlue => vec![BirdAction::Perch],
            Self::WoodenPerch | Self::FancyPerch => vec![BirdAction::Perch],
            
            // Food items - provide eating
            Self::BasicBirdSeed | Self::PremiumSeed => vec![BirdAction::Eat],
            Self::SuetCake => vec![BirdAction::Eat],
            Self::NectarFeeder => vec![BirdAction::Eat], // Hummingbirds "eat" nectar
            Self::FruitDispenser => vec![BirdAction::Eat],
            
            // Water features - provide drinking and bathing
            Self::BasicBirdbath => vec![BirdAction::Drink, BirdAction::Bathe],
            Self::FountainBirdbath => vec![BirdAction::Drink, BirdAction::Bathe],
            Self::StreamFeature => vec![BirdAction::Drink, BirdAction::Bathe],
            
            // Decorative items - provide exploration
            Self::GardenGnome => vec![BirdAction::Explore],
            Self::WindChime => vec![BirdAction::Explore],
            Self::FlowerPot => vec![BirdAction::Explore, BirdAction::Perch], // Birds may perch on pot edge
            
            // Nesting items - provide nesting behavior
            Self::BirdHouse | Self::NestingBox => vec![BirdAction::Nest, BirdAction::Perch],
            
            // Special items - provide play behavior
            Self::MirrorToy => vec![BirdAction::Play, BirdAction::Explore],
            Self::BellToy => vec![BirdAction::Play],
            Self::SwingSeat => vec![BirdAction::Play, BirdAction::Perch],
        }
    }

    /// Returns the base utility value for this item type
    pub fn base_utility(&self) -> f32 {
        match self {
            // Comfort items - high utility for resting
            Self::CardboardBox => 0.6, // Good for exploration
            Self::CushionRed | Self::CushionBlue => 0.7, // Comfortable perching
            Self::WoodenPerch => 0.8, // Natural and preferred
            Self::FancyPerch => 0.9, // Premium perching experience
            
            // Food items - very high utility when hungry
            Self::BasicBirdSeed => 0.7,
            Self::PremiumSeed => 0.9, // Higher quality attracts more
            Self::SuetCake => 0.8, // Specialist food for specific species
            Self::NectarFeeder => 0.9, // Essential for hummingbirds
            Self::FruitDispenser => 0.8,
            
            // Water features - essential utility
            Self::BasicBirdbath => 0.8, // Essential for birds
            Self::FountainBirdbath => 0.9, // Moving water is more attractive
            Self::StreamFeature => 0.95, // Most natural and appealing
            
            // Decorative items - moderate utility for curiosity
            Self::GardenGnome => 0.4, // Some birds are curious
            Self::WindChime => 0.3, // Sound may attract or deter
            Self::FlowerPot => 0.5, // Insects around flowers attract birds
            
            // Nesting items - seasonal but very important during breeding
            Self::BirdHouse => 0.7, // Good for cavity nesters
            Self::NestingBox => 0.8, // Specialized nesting
            
            // Play items - moderate utility for enrichment
            Self::MirrorToy => 0.6, // Fascinating to some species
            Self::BellToy => 0.4, // Simple play object
            Self::SwingSeat => 0.7, // Fun and functional
        }
    }

    /// Returns the interaction range for this item
    pub fn interaction_range(&self) -> f32 {
        match self {
            // Comfort items
            Self::CardboardBox => 40.0,
            Self::CushionRed | Self::CushionBlue => 30.0,
            Self::WoodenPerch | Self::FancyPerch => 35.0,
            
            // Food items - larger range to attract birds
            Self::BasicBirdSeed | Self::PremiumSeed => 80.0,
            Self::SuetCake => 70.0,
            Self::NectarFeeder => 60.0, // Smaller range for precise feeders
            Self::FruitDispenser => 75.0,
            
            // Water features - large attraction range
            Self::BasicBirdbath => 90.0,
            Self::FountainBirdbath => 100.0, // Sound attracts from further away
            Self::StreamFeature => 120.0, // Large natural feature
            
            // Decorative items
            Self::GardenGnome => 50.0,
            Self::WindChime => 60.0, // Sound carries further
            Self::FlowerPot => 45.0,
            
            // Nesting items
            Self::BirdHouse | Self::NestingBox => 40.0,
            
            // Play items
            Self::MirrorToy => 35.0,
            Self::BellToy => 30.0,
            Self::SwingSeat => 45.0,
        }
    }

    /// Returns the physical size for collision detection
    pub fn physical_size(&self) -> Vec2 {
        match self {
            Self::CardboardBox => Vec2::new(50.0, 35.0),
            Self::CushionRed | Self::CushionBlue => Vec2::new(40.0, 40.0),
            Self::WoodenPerch => Vec2::new(60.0, 10.0),
            Self::FancyPerch => Vec2::new(70.0, 15.0),
            Self::BasicBirdSeed | Self::PremiumSeed => Vec2::new(30.0, 30.0),
            Self::SuetCake => Vec2::new(25.0, 25.0),
            Self::NectarFeeder => Vec2::new(20.0, 35.0),
            Self::FruitDispenser => Vec2::new(35.0, 40.0),
            Self::BasicBirdbath => Vec2::new(45.0, 45.0),
            Self::FountainBirdbath => Vec2::new(55.0, 55.0),
            Self::StreamFeature => Vec2::new(150.0, 30.0),
            Self::GardenGnome => Vec2::new(20.0, 35.0),
            Self::WindChime => Vec2::new(15.0, 40.0),
            Self::FlowerPot => Vec2::new(30.0, 25.0),
            Self::BirdHouse => Vec2::new(25.0, 35.0),
            Self::NestingBox => Vec2::new(30.0, 25.0),
            Self::MirrorToy => Vec2::new(20.0, 30.0),
            Self::BellToy => Vec2::new(15.0, 20.0),
            Self::SwingSeat => Vec2::new(35.0, 40.0),
        }
    }
}