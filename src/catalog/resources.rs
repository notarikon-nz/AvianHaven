use bevy::prelude::*;
use std::collections::HashMap;
use crate::catalog::components::PlaceableItemType;

#[derive(Resource, Default)]
pub struct CatalogState {
    pub is_open: bool,
    pub selected_category: ItemCategory,
    pub selected_item: Option<PlaceableItemType>,
}

#[derive(Resource)]
pub struct PlayerInventory {
    pub currency: u32,
    pub owned_items: HashMap<PlaceableItemType, u32>, // item type -> quantity owned
}

impl Default for PlayerInventory {
    fn default() -> Self {
        Self {
            currency: 100, // Starting currency
            owned_items: HashMap::new(),
        }
    }
}

#[derive(Resource, Default)]
pub struct PlacedObjects {
    pub objects: HashMap<Entity, PlaceableItemType>,
    pub placement_mode: bool,
    pub ghost_entity: Option<Entity>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum ItemCategory {
    #[default]
    Comfort,
    Food,
    Water,
    Decorative,
    Special,
}

impl ItemCategory {
    pub fn name(&self) -> &str {
        match self {
            Self::Comfort => "Comfort",
            Self::Food => "Food",
            Self::Water => "Water",
            Self::Decorative => "Decorative", 
            Self::Special => "Special",
        }
    }
    
    pub fn items(&self) -> Vec<PlaceableItemType> {
        match self {
            Self::Comfort => vec![
                PlaceableItemType::CardboardBox,
                PlaceableItemType::CushionRed,
                PlaceableItemType::CushionBlue,
                PlaceableItemType::WoodenPerch,
                PlaceableItemType::FancyPerch,
            ],
            Self::Food => vec![
                PlaceableItemType::BasicBirdSeed,
                PlaceableItemType::PremiumSeed,
                PlaceableItemType::SuetCake,
                PlaceableItemType::NectarFeeder,
                PlaceableItemType::FruitDispenser,
            ],
            Self::Water => vec![
                PlaceableItemType::BasicBirdbath,
                PlaceableItemType::FountainBirdbath,
                PlaceableItemType::StreamFeature,
            ],
            Self::Decorative => vec![
                PlaceableItemType::GardenGnome,
                PlaceableItemType::WindChime,
                PlaceableItemType::FlowerPot,
                PlaceableItemType::BirdHouse,
                PlaceableItemType::NestingBox,
            ],
            Self::Special => vec![
                PlaceableItemType::MirrorToy,
                PlaceableItemType::BellToy,
                PlaceableItemType::SwingSeat,
            ],
        }
    }
}

#[derive(Event)]
pub struct PurchaseItemEvent {
    pub item_type: PlaceableItemType,
}

#[derive(Event)]
pub struct PlaceObjectEvent {
    pub item_type: PlaceableItemType,
    pub position: Vec3,
}