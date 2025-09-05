use bevy::prelude::*;
use std::collections::HashMap;
use crate::smart_objects::config::*;

// Creation Kit UI and functionality
#[derive(Resource, Default)]
pub struct CreationKitState {
    pub is_open: bool,
    pub current_template: Option<SmartObjectDefinition>,
    pub editing_mode: EditingMode,
    pub preview_entity: Option<Entity>,
    pub unsaved_changes: bool,
    pub selected_property_category: PropertyCategory,
}

#[derive(Default, PartialEq, Eq)]
pub enum EditingMode {
    #[default]
    Browsing,    // Browsing existing templates
    Creating,    // Creating new object from scratch
    Modifying,   // Modifying existing object
    Testing,     // Testing object in game world
}

#[derive(Default, PartialEq, Eq)]
pub enum PropertyCategory {
    #[default]
    Metadata,    // Name, description, category
    Visual,      // Appearance and animation
    Physics,     // Collision and weight
    Behavior,    // Actions and utility
    Attraction,  // Species preferences
    Economy,     // Cost and durability
    Workshop,    // Modding and sharing
}

// Templates for common object types
#[derive(Resource)]
pub struct CreationKitTemplates {
    pub templates: HashMap<String, SmartObjectDefinition>,
}

impl Default for CreationKitTemplates {
    fn default() -> Self {
        let mut templates = HashMap::new();
        
        // Basic perch template
        templates.insert("basic_perch".to_string(), SmartObjectDefinition {
            id: "custom_perch".to_string(),
            metadata: SmartObjectMetadata {
                name: "Custom Perch".to_string(),
                description: "A custom perching spot for birds".to_string(),
                category: "Comfort".to_string(),
                price: 50,
                rarity: "Common".to_string(),
                unlock_level: 1,
                workshop_compatible: true,
                version: "1.0.0".to_string(),
                author: "Player".to_string(),
                tags: vec!["perch".to_string(), "custom".to_string()],
            },
            visual: VisualData {
                sprite_filename: "custom_perch".to_string(),
                size: (40.0, 10.0),
                color_tint: (1.0, 1.0, 1.0, 1.0),
                scale: 1.0,
                z_order: 1.0,
                animation_set: None,
                particle_effects: Vec::new(),
                sound_effects: vec!["perch_land".to_string()],
            },
            physics: PhysicsData {
                collision_shape: "Rectangle".to_string(),
                collision_size: (40.0, 10.0),
                is_solid: true,
                weight: 5.0,
                can_be_moved: true,
                stability: 0.8,
            },
            behavior: BehaviorData {
                provides_actions: vec!["Perch".to_string()],
                base_utility: 0.7,
                interaction_range: 30.0,
                max_simultaneous_users: 2,
                usage_duration_range: (5.0, 20.0),
                cooldown_after_use: 0.0,
                seasonal_modifiers: Vec::new(),
                weather_resistance: 0.7,
            },
            attraction: AttractionData {
                attracts_species: Vec::new(), // Universal appeal
                species_preferences: HashMap::new(),
                size_preferences: vec![2, 3, 4],
                time_of_day_bonus: Vec::new(),
                seasonal_attraction: HashMap::new(),
            },
            economy: EconomyData {
                purchase_cost: 50,
                maintenance_cost: 1,
                durability: 0.8,
                decay_rate: 0.01,
                repair_cost_multiplier: 0.3,
                resale_value_multiplier: 0.5,
            },
            workshop: WorkshopData {
                is_moddable: true,
                custom_script_support: true,
                texture_replaceable: true,
                model_replaceable: true,
                behavior_scriptable: true,
                required_dependencies: Vec::new(),
            },
        });
        
        // Basic feeder template
        templates.insert("basic_feeder".to_string(), SmartObjectDefinition {
            id: "custom_feeder".to_string(),
            metadata: SmartObjectMetadata {
                name: "Custom Feeder".to_string(),
                description: "A custom feeding station".to_string(),
                category: "Food".to_string(),
                price: 80,
                rarity: "Common".to_string(),
                unlock_level: 1,
                workshop_compatible: true,
                version: "1.0.0".to_string(),
                author: "Player".to_string(),
                tags: vec!["feeder".to_string(), "food".to_string(), "custom".to_string()],
            },
            visual: VisualData {
                sprite_filename: "custom_feeder".to_string(),
                size: (30.0, 40.0),
                color_tint: (1.0, 1.0, 1.0, 1.0),
                scale: 1.0,
                z_order: 1.0,
                animation_set: None,
                particle_effects: Vec::new(),
                sound_effects: vec!["seed_scatter".to_string()],
            },
            physics: PhysicsData {
                collision_shape: "Rectangle".to_string(),
                collision_size: (30.0, 40.0),
                is_solid: true,
                weight: 10.0,
                can_be_moved: true,
                stability: 0.9,
            },
            behavior: BehaviorData {
                provides_actions: vec!["Eat".to_string()],
                base_utility: 0.8,
                interaction_range: 70.0,
                max_simultaneous_users: 3,
                usage_duration_range: (3.0, 10.0),
                cooldown_after_use: 1.0,
                seasonal_modifiers: Vec::new(),
                weather_resistance: 0.6,
            },
            attraction: AttractionData {
                attracts_species: Vec::new(),
                species_preferences: HashMap::new(),
                size_preferences: vec![1, 2, 3, 4, 5],
                time_of_day_bonus: Vec::new(),
                seasonal_attraction: HashMap::new(),
            },
            economy: EconomyData {
                purchase_cost: 80,
                maintenance_cost: 2,
                durability: 0.8,
                decay_rate: 0.015,
                repair_cost_multiplier: 0.4,
                resale_value_multiplier: 0.4,
            },
            workshop: WorkshopData {
                is_moddable: true,
                custom_script_support: true,
                texture_replaceable: true,
                model_replaceable: true,
                behavior_scriptable: true,
                required_dependencies: Vec::new(),
            },
        });
        
        Self { templates }
    }
}

// Events for the Creation Kit
#[derive(Event)]
pub struct CreateNewObjectEvent {
    pub template_id: String,
}

#[derive(Event)]
pub struct SaveObjectEvent {
    pub definition: SmartObjectDefinition,
    pub save_path: String,
}

#[derive(Event)]
pub struct LoadObjectEvent {
    pub file_path: String,
}

#[derive(Event)]
pub struct TestObjectEvent {
    pub definition: SmartObjectDefinition,
    pub test_position: Vec3,
}

#[derive(Event)]
pub struct ExportObjectEvent {
    pub definition: SmartObjectDefinition,
    pub export_format: ExportFormat,
    pub export_path: String,
}

#[derive(Debug, Clone)]
pub enum ExportFormat {
    Ron,          // RON file for game use
    WorkshopPack, // Packaged for Steam Workshop
    JsonData,     // JSON for external tools
}

// Property validation and constraints
#[derive(Debug, Clone)]
pub struct PropertyConstraints {
    pub min_price: u32,
    pub max_price: u32,
    pub min_size: (f32, f32),
    pub max_size: (f32, f32),
    pub min_utility: f32,
    pub max_utility: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub allowed_actions: Vec<String>,
    pub allowed_categories: Vec<String>,
    pub max_simultaneous_users: u32,
}

impl Default for PropertyConstraints {
    fn default() -> Self {
        Self {
            min_price: 1,
            max_price: 10000,
            min_size: (5.0, 5.0),
            max_size: (200.0, 200.0),
            min_utility: 0.1,
            max_utility: 1.0,
            min_range: 10.0,
            max_range: 300.0,
            allowed_actions: vec![
                "Eat".to_string(), "Drink".to_string(), "Bathe".to_string(), 
                "Perch".to_string(), "Play".to_string(), "Explore".to_string(),
                "Nest".to_string(), "Roost".to_string(), "Shelter".to_string(),
            ],
            allowed_categories: vec![
                "Comfort".to_string(), "Food".to_string(), "Water".to_string(),
                "Decorative".to_string(), "Special".to_string(),
            ],
            max_simultaneous_users: 10,
        }
    }
}

// Validation functions
pub struct ObjectValidator;

impl ObjectValidator {
    pub fn validate_definition(definition: &SmartObjectDefinition, constraints: &PropertyConstraints) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        
        // Validate metadata
        if definition.metadata.name.is_empty() {
            errors.push("Object name cannot be empty".to_string());
        }
        
        if definition.metadata.price < constraints.min_price || definition.metadata.price > constraints.max_price {
            errors.push(format!("Price must be between {} and {}", constraints.min_price, constraints.max_price));
        }
        
        if !constraints.allowed_categories.contains(&definition.metadata.category) {
            errors.push(format!("Category '{}' is not allowed", definition.metadata.category));
        }
        
        // Validate visual data
        if definition.visual.size.0 < constraints.min_size.0 || definition.visual.size.0 > constraints.max_size.0 ||
           definition.visual.size.1 < constraints.min_size.1 || definition.visual.size.1 > constraints.max_size.1 {
            errors.push(format!("Size must be between {:?} and {:?}", constraints.min_size, constraints.max_size));
        }
        
        // Validate behavior data
        if definition.behavior.base_utility < constraints.min_utility || definition.behavior.base_utility > constraints.max_utility {
            errors.push(format!("Base utility must be between {} and {}", constraints.min_utility, constraints.max_utility));
        }
        
        if definition.behavior.interaction_range < constraints.min_range || definition.behavior.interaction_range > constraints.max_range {
            errors.push(format!("Interaction range must be between {} and {}", constraints.min_range, constraints.max_range));
        }
        
        if definition.behavior.max_simultaneous_users > constraints.max_simultaneous_users {
            errors.push(format!("Max simultaneous users cannot exceed {}", constraints.max_simultaneous_users));
        }
        
        // Check for invalid actions
        for action in &definition.behavior.provides_actions {
            if !constraints.allowed_actions.contains(action) {
                errors.push(format!("Action '{}' is not allowed", action));
            }
        }
        
        // Generate warnings for potentially problematic values
        if definition.behavior.base_utility > 0.9 {
            warnings.push("Very high utility values may make other objects less attractive".to_string());
        }
        
        if definition.behavior.interaction_range > 150.0 {
            warnings.push("Large interaction ranges may cause performance issues".to_string());
        }
        
        if definition.economy.durability < 0.3 {
            warnings.push("Low durability objects will require frequent maintenance".to_string());
        }
        
        ValidationResult { errors, warnings }
    }
}

#[derive(Debug)]
pub struct ValidationResult {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
    
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

// Helper functions for the Creation Kit UI
pub struct CreationKitHelpers;

impl CreationKitHelpers {
    pub fn get_property_display_name(property: &str) -> &str {
        match property {
            "name" => "Display Name",
            "description" => "Description",
            "category" => "Category",
            "price" => "Purchase Cost",
            "base_utility" => "Base Attractiveness",
            "interaction_range" => "Detection Range",
            "max_simultaneous_users" => "Max Users",
            "provides_actions" => "Bird Behaviors",
            "attracts_species" => "Preferred Species",
            "weather_resistance" => "Weather Resistance",
            "durability" => "Durability",
            _ => property,
        }
    }
    
    pub fn get_property_tooltip(property: &str) -> &str {
        match property {
            "base_utility" => "How attractive this object is to birds (0.0 = ignored, 1.0 = highly attractive)",
            "interaction_range" => "How far away birds can detect and path to this object",
            "max_simultaneous_users" => "Maximum number of birds that can use this object at once",
            "weather_resistance" => "How well this object functions in bad weather (0.0 = very affected, 1.0 = unaffected)",
            "durability" => "How long this object lasts before needing maintenance",
            "provides_actions" => "What behaviors birds can perform with this object",
            _ => "No additional information available",
        }
    }
    
    pub fn get_category_color(category: &str) -> Color {
        match category {
            "Comfort" => Color::srgb(0.6, 0.8, 0.6),
            "Food" => Color::srgb(0.8, 0.6, 0.4),
            "Water" => Color::srgb(0.4, 0.6, 0.8),
            "Decorative" => Color::srgb(0.8, 0.6, 0.8),
            "Special" => Color::srgb(0.9, 0.7, 0.3),
            _ => Color::srgb(0.6, 0.6, 0.6),
        }
    }
    
    pub fn generate_unique_id(base_name: &str) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let clean_name = base_name
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();
        
        format!("{}_{}", clean_name, timestamp)
    }
    
    pub fn export_to_workshop_format(definition: &SmartObjectDefinition) -> WorkshopPackage {
        WorkshopPackage {
            definition: definition.clone(),
            required_assets: vec![
                format!("sprites/objects/{}.png", definition.visual.sprite_filename),
            ],
            metadata_version: "1.0".to_string(),
            compatibility_version: "1.0".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorkshopPackage {
    pub definition: SmartObjectDefinition,
    pub required_assets: Vec<String>,
    pub metadata_version: String,
    pub compatibility_version: String,
}