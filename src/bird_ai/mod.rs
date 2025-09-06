use bevy::prelude::*;
use bevy::asset::{io::Reader, AssetLoader, LoadContext};

pub mod components;
pub mod resources;
pub mod systems;
pub mod bt;
pub mod states;
pub mod config;

use resources::*;
use systems::*;
use config::*;
use crate::{AppState};

#[derive(Default)]
pub struct BehaviorTreeConfigAssetLoader;

impl AssetLoader for BehaviorTreeConfigAssetLoader {
    type Asset = BehaviorTreeConfig;
    type Settings = ();
    type Error = Box<dyn std::error::Error + Send + Sync>;
    
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        info!("BehaviorTreeConfigAssetLoader: Loading asset from path: {:?}", load_context.path());
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        
        #[cfg(debug_assertions)]
        info!("Debug: Read {} bytes from behavior tree config file", bytes.len());
        
        match ron::de::from_bytes::<BehaviorTreeConfig>(&bytes) {
            Ok(config) => {
                info!("BehaviorTreeConfigAssetLoader: Successfully loaded behavior tree config with {} rules", config.rules.len());
                #[cfg(debug_assertions)]
                info!("Debug: Config thresholds loaded - critical_hunger: {}", config.thresholds.critical_hunger);
                Ok(config)
            }
            Err(e) => {
                error!("BehaviorTreeConfigAssetLoader: Failed to parse RON file: {}", e);
                #[cfg(debug_assertions)]
                error!("Debug: RON parse error details: {:#?}", e);
                Err(Box::new(e))
            }
        }
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

pub struct BirdAiPlugin;

impl Plugin for BirdAiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset::<BehaviorTreeConfig>()
            .register_asset_loader(BehaviorTreeConfigAssetLoader)
            .init_resource::<UtilityTimer>()
            .init_resource::<BehaviorTreeTimer>()
            .init_resource::<resources::BehaviorTreeConfigResource>()
            .add_systems(Startup, (setup_test_world, load_behavior_tree_config))
            .add_systems(Update, (
                // Core AI systems
                world_utility_query_system,
                social_awareness_system,
                behavior_tree_system,
                need_decay_system,
            ).run_if(in_state(AppState::Playing)))
            .add_systems(Update, (
                // Basic behavior systems
                wandering_system,
                moving_to_target_system,
                eating_system,
                drinking_system,
                bathing_system,
                fleeing_system,
                resting_system,
            ).run_if(in_state(AppState::Playing)))
            .add_systems(Update, (
                // Advanced behavior systems
                playing_system,
                exploring_system,
                nesting_system,
                roosting_system,
                sheltering_system,
            ).run_if(in_state(AppState::Playing)))
            .add_systems(Update, (
                // Social behavior systems
                courting_system,
                territorial_system,
                flocking_system,
                following_system,
            ).run_if(in_state(AppState::Playing)))
            .add_systems(Update, (
                // Foraging behavior systems
                foraging_system,
                caching_system,
                retrieving_system,
                hover_feeding_system,
                competitive_feeding_system,
            ).run_if(in_state(AppState::Playing)))
            .add_systems(Update, check_behavior_tree_loading);
    }
}

// Load behavior tree config once at startup
pub fn load_behavior_tree_config(
    asset_server: Res<AssetServer>,
    mut config_resource: ResMut<resources::BehaviorTreeConfigResource>,
) {
    info!("Loading behavior tree config at startup from data/behavior_tree.ron");
    let config_handle: Handle<BehaviorTreeConfig> = asset_server.load("data/behavior_tree.ron");
    config_resource.handle = Some(config_handle);
}

// Check if the behavior tree config has loaded and store it in our resource
pub fn check_behavior_tree_loading(
    mut config_resource: ResMut<resources::BehaviorTreeConfigResource>,
    config_assets: Res<Assets<BehaviorTreeConfig>>,
) {
    if config_resource.config.is_some() {
        return; // Already loaded
    }
    
    if let Some(handle) = &config_resource.handle {
        if let Some(config) = config_assets.get(handle) {
            info!("Successfully loaded behavior tree config with {} rules", config.rules.len());
            #[cfg(debug_assertions)]
            info!("Debug: First rule name: {}, priority: {}", 
                config.rules.first().map(|r| &r.name).unwrap_or(&"none".to_string()),
                config.rules.first().map(|r| r.priority).unwrap_or(0));
            
            config_resource.config = Some(config.clone());
            config_resource.use_configurable = true;
        }
    }
}
