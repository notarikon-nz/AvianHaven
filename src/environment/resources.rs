use bevy::prelude::*;
use super::components::{Season, Weather};
use crate::bird::BirdSpecies;
use std::collections::HashMap;

#[derive(Resource)]
pub struct TimeState {
    pub hour: f32,           // 0.0-24.0
    pub time_speed: f32,     // Real seconds per game hour
    pub day_of_year: u32,    // 1-365
}

impl Default for TimeState {
    fn default() -> Self {
        Self {
            hour: 8.0,           // Start at 8 AM
            time_speed: 60.0,    // 1 minute real time = 1 hour game time
            day_of_year: 120,    // Start in late spring (day 120)
        }
    }
}

impl TimeState {
    pub fn get_season(&self) -> Season {
        match self.day_of_year {
            1..=79 => Season::Winter,
            80..=171 => Season::Spring,
            172..=264 => Season::Summer,
            265..=355 => Season::Fall,
            _ => Season::Winter,
        }
    }
    
    pub fn is_prime_feeding_time(&self) -> bool {
        // Birds are most active in early morning and late afternoon
        (self.hour >= 6.0 && self.hour <= 10.0) || (self.hour >= 16.0 && self.hour <= 19.0)
    }
    
    pub fn daylight_factor(&self) -> f32 {
        // Simple daylight curve
        if self.hour < 6.0 || self.hour > 20.0 {
            0.1 // Night time - minimal activity
        } else if self.hour < 8.0 || self.hour > 18.0 {
            0.6 // Dawn/dusk - moderate activity
        } else {
            1.0 // Daylight - full activity
        }
    }
}

#[derive(Resource)]
pub struct WeatherState {
    pub current_weather: Weather,
    pub weather_timer: Timer,
    pub temperature: f32,    // Celsius
}

impl Default for WeatherState {
    fn default() -> Self {
        Self {
            current_weather: Weather::Clear,
            weather_timer: Timer::from_seconds(300.0, TimerMode::Repeating), // Change every 5 minutes
            temperature: 20.0,
        }
    }
}

#[derive(Resource)]
pub struct SeasonalState {
    pub available_species: HashMap<BirdSpecies, f32>, // Species -> spawn probability
    pub migration_active: bool,
}

impl Default for SeasonalState {
    fn default() -> Self {
        let mut available_species = HashMap::new();
        
        // Initialize with year-round residents
        available_species.insert(BirdSpecies::Cardinal, 1.0);
        available_species.insert(BirdSpecies::BlueJay, 1.0);
        available_species.insert(BirdSpecies::Chickadee, 1.0);
        available_species.insert(BirdSpecies::HouseFinch, 1.0);
        available_species.insert(BirdSpecies::Sparrow, 1.0);
        available_species.insert(BirdSpecies::Robin, 1.0);
        
        Self {
            available_species,
            migration_active: false,
        }
    }
}

impl SeasonalState {
    pub fn update_for_season(&mut self, season: Season) {
        self.available_species.clear();
        
        // Year-round residents (always available)
        let residents = vec![
            BirdSpecies::Cardinal, BirdSpecies::BlueJay, BirdSpecies::Chickadee,
            BirdSpecies::HouseFinch, BirdSpecies::Sparrow, BirdSpecies::CommonCrow,
            BirdSpecies::EuropeanStarling, BirdSpecies::CommonGrackle,
        ];
        
        for species in residents {
            self.available_species.insert(species, 1.0);
        }
        
        // Seasonal species
        match season {
            Season::Spring => {
                // Spring migrants
                self.available_species.insert(BirdSpecies::Robin, 1.2);
                self.available_species.insert(BirdSpecies::YellowWarbler, 0.8);
                self.available_species.insert(BirdSpecies::RedWingedBlackbird, 0.9);
                self.available_species.insert(BirdSpecies::BrownThrasher, 0.6);
                self.migration_active = true;
            },
            Season::Summer => {
                // Peak breeding season - high activity
                self.available_species.insert(BirdSpecies::Robin, 1.3);
                self.available_species.insert(BirdSpecies::Goldfinch, 1.1);
                self.available_species.insert(BirdSpecies::CedarWaxwing, 0.8);
                self.available_species.insert(BirdSpecies::YellowWarbler, 1.0);
                self.available_species.insert(BirdSpecies::RedWingedBlackbird, 1.1);
                self.migration_active = false;
            },
            Season::Fall => {
                // Fall migration - high diversity
                self.available_species.insert(BirdSpecies::Robin, 0.9);
                self.available_species.insert(BirdSpecies::YellowWarbler, 0.7);
                self.available_species.insert(BirdSpecies::WhiteBreastedNuthatch, 1.0);
                self.available_species.insert(BirdSpecies::CedarWaxwing, 1.2);
                self.available_species.insert(BirdSpecies::BlueGrayGnatcatcher, 0.6);
                self.migration_active = true;
            },
            Season::Winter => {
                // Mostly residents, some winter visitors
                self.available_species.insert(BirdSpecies::WhiteBreastedNuthatch, 1.1);
                self.available_species.insert(BirdSpecies::TuftedTitmouse, 1.0);
                self.available_species.insert(BirdSpecies::CarolinaWren, 0.8);
                self.available_species.insert(BirdSpecies::MourningDove, 0.9);
                self.migration_active = false;
            },
        }
    }
    
    pub fn get_spawn_probability(&self, species: &BirdSpecies) -> f32 {
        self.available_species.get(species).copied().unwrap_or(0.0)
    }
}

#[derive(Event)]
pub struct WeatherChangeEvent {
    pub new_weather: Weather,
    pub temperature: f32,
}

#[derive(Event)]
pub struct TimeChangeEvent {
    pub new_hour: f32,
    pub new_day: u32,
}