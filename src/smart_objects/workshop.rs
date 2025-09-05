use bevy::prelude::*;
use std::collections::HashMap;
use crate::smart_objects::config::*;
use crate::smart_objects::creation_kit::*;

// Steam Workshop integration framework
#[derive(Resource)]
pub struct SteamWorkshopClient {
    pub is_initialized: bool,
    pub user_subscriptions: HashMap<String, WorkshopSubscription>,
    pub published_items: Vec<String>,
    pub download_queue: Vec<WorkshopDownloadTask>,
    pub upload_queue: Vec<WorkshopUploadTask>,
}

impl Default for SteamWorkshopClient {
    fn default() -> Self {
        Self {
            is_initialized: false,
            user_subscriptions: HashMap::new(),
            published_items: Vec::new(),
            download_queue: Vec::new(),
            upload_queue: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorkshopSubscription {
    pub item_id: String,
    pub title: String,
    pub local_path: String,
    pub last_updated: u64,
    pub is_installed: bool,
    pub needs_update: bool,
}

#[derive(Debug, Clone)]
pub struct WorkshopDownloadTask {
    pub item_id: String,
    pub priority: DownloadPriority,
    pub callback_entity: Option<Entity>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownloadPriority {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct WorkshopUploadTask {
    pub package: WorkshopPackage,
    pub upload_metadata: WorkshopUploadMetadata,
    pub preview_image_path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WorkshopUploadMetadata {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub visibility: WorkshopVisibility,
    pub change_notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkshopVisibility {
    Public,
    FriendsOnly,
    Private,
}

// Workshop Browser UI
#[derive(Component)]
pub struct WorkshopBrowser;

#[derive(Component)]
pub struct WorkshopItemCard {
    pub item_id: String,
}

#[derive(Component)]
pub struct WorkshopSearchBar;

#[derive(Component)]
pub struct WorkshopFilterPanel;

#[derive(Resource)]
pub struct WorkshopBrowserState {
    pub is_open: bool,
    pub current_search: String,
    pub active_filters: WorkshopFilters,
    pub sort_order: WorkshopSortOrder,
    pub current_page: u32,
    pub items_per_page: u32,
    pub loaded_items: Vec<WorkshopBrowserItem>,
    pub selected_item: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WorkshopFilters {
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub rating_min: f32,
    pub author: Option<String>,
    pub subscribed_only: bool,
    pub compatible_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkshopSortOrder {
    Popular,
    Recent,
    Rating,
    Alphabetical,
    Author,
    Downloads,
}

#[derive(Debug, Clone)]
pub struct WorkshopBrowserItem {
    pub item_id: String,
    pub title: String,
    pub author: String,
    pub description: String,
    pub preview_image: Option<Handle<Image>>,
    pub rating: f32,
    pub download_count: u32,
    pub file_size: u64,
    pub last_updated: u64,
    pub tags: Vec<String>,
    pub is_subscribed: bool,
    pub is_installed: bool,
}

impl Default for WorkshopBrowserState {
    fn default() -> Self {
        Self {
            is_open: false,
            current_search: String::new(),
            active_filters: WorkshopFilters::default(),
            sort_order: WorkshopSortOrder::Popular,
            current_page: 0,
            items_per_page: 20,
            loaded_items: Vec::new(),
            selected_item: None,
        }
    }
}

impl Default for WorkshopFilters {
    fn default() -> Self {
        Self {
            categories: Vec::new(),
            tags: Vec::new(),
            rating_min: 0.0,
            author: None,
            subscribed_only: false,
            compatible_only: true,
        }
    }
}

// Workshop Events
#[derive(Event)]
pub struct SubscribeToItemEvent {
    pub item_id: String,
}

#[derive(Event)]
pub struct UnsubscribeFromItemEvent {
    pub item_id: String,
}

#[derive(Event)]
pub struct RateItemEvent {
    pub item_id: String,
    pub rating: u32, // 1-5 stars
}

#[derive(Event)]
pub struct ReportItemEvent {
    pub item_id: String,
    pub report_reason: String,
}

#[derive(Event)]
pub struct PublishItemEvent {
    pub package: WorkshopPackage,
    pub metadata: WorkshopUploadMetadata,
}

#[derive(Event)]
pub struct UpdatePublishedItemEvent {
    pub item_id: String,
    pub package: WorkshopPackage,
    pub change_notes: String,
}

// Workshop integration systems
pub fn initialize_steam_workshop(
    mut workshop_client: ResMut<SteamWorkshopClient>,
) {
    info!("Initializing Steam Workshop integration...");
    
    // TODO: Initialize Steam Workshop API
    // This would involve:
    // 1. Checking if Steam is running
    // 2. Initializing the Steam API
    // 3. Setting up Workshop callbacks
    // 4. Loading user subscriptions
    
    workshop_client.is_initialized = true;
    info!("Steam Workshop integration initialized");
}

pub fn process_workshop_downloads(
    mut workshop_client: ResMut<SteamWorkshopClient>,
    mut download_events: EventReader<crate::smart_objects::WorkshopDownloadEvent>,
) {
    for event in download_events.read() {
        let task = WorkshopDownloadTask {
            item_id: event.workshop_item_id.clone(),
            priority: DownloadPriority::Normal,
            callback_entity: None,
        };
        
        workshop_client.download_queue.push(task);
        info!("Queued workshop item for download: {}", event.workshop_item_id);
    }
    
    // Process download queue
    if let Some(task) = workshop_client.download_queue.first().cloned() {
        // TODO: Implement actual Steam Workshop download
        // This would involve:
        // 1. Calling Steam Workshop API to download item
        // 2. Extracting package to local directory
        // 3. Validating package integrity
        // 4. Loading smart object definition
        // 5. Installing textures and assets
        
        info!("Processing download for item: {}", task.item_id);
        workshop_client.download_queue.remove(0);
    }
}

pub fn process_workshop_uploads(
    mut workshop_client: ResMut<SteamWorkshopClient>,
    mut upload_events: EventReader<crate::smart_objects::WorkshopUploadEvent>,
) {
    for event in upload_events.read() {
        // TODO: Convert event to upload task
        info!("Queued workshop item for upload: {}", event.object_id);
    }
    
    // Process upload queue
    if let Some(task) = workshop_client.upload_queue.first().cloned() {
        // TODO: Implement actual Steam Workshop upload
        // This would involve:
        // 1. Packaging smart object definition and assets
        // 2. Creating preview image
        // 3. Uploading to Steam Workshop
        // 4. Setting metadata and tags
        // 5. Publishing or updating existing item
        
        info!("Processing upload for item: {}", task.package.definition.id);
        workshop_client.upload_queue.remove(0);
    }
}

pub fn handle_workshop_subscriptions(
    mut subscribe_events: EventReader<SubscribeToItemEvent>,
    mut unsubscribe_events: EventReader<UnsubscribeFromItemEvent>,
    mut workshop_client: ResMut<SteamWorkshopClient>,
) {
    for event in subscribe_events.read() {
        // TODO: Call Steam Workshop API to subscribe
        let subscription = WorkshopSubscription {
            item_id: event.item_id.clone(),
            title: "Loading...".to_string(),
            local_path: String::new(),
            last_updated: 0,
            is_installed: false,
            needs_update: false,
        };
        
        workshop_client.user_subscriptions.insert(event.item_id.clone(), subscription);
        info!("Subscribed to workshop item: {}", event.item_id);
    }
    
    for event in unsubscribe_events.read() {
        // TODO: Call Steam Workshop API to unsubscribe
        workshop_client.user_subscriptions.remove(&event.item_id);
        info!("Unsubscribed from workshop item: {}", event.item_id);
    }
}

pub fn update_workshop_browser(
    mut browser_state: ResMut<WorkshopBrowserState>,
    workshop_client: Res<SteamWorkshopClient>,
) {
    if !browser_state.is_open {
        return;
    }
    
    // TODO: Query Steam Workshop API based on current search and filters
    // This would involve:
    // 1. Building query parameters from search and filters
    // 2. Making API request to Steam Workshop
    // 3. Parsing results into WorkshopBrowserItem structs
    // 4. Loading preview images asynchronously
    // 5. Updating browser state with results
    
    // For now, just log the current search state
    if !browser_state.current_search.is_empty() {
        debug!("Workshop search: '{}' with {} filters", 
               browser_state.current_search, 
               browser_state.active_filters.tags.len());
    }
}

// Workshop package validation
pub struct WorkshopValidator;

impl WorkshopValidator {
    pub fn validate_package(package: &WorkshopPackage) -> WorkshopValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        
        // Validate definition
        let constraints = PropertyConstraints::default();
        let definition_result = ObjectValidator::validate_definition(&package.definition, &constraints);
        errors.extend(definition_result.errors);
        warnings.extend(definition_result.warnings);
        
        // Validate required assets
        for asset_path in &package.required_assets {
            if asset_path.is_empty() {
                errors.push("Asset path cannot be empty".to_string());
            }
            
            // Check if asset files actually exist
            let full_path = format!("assets/{}", asset_path);
            if !std::path::Path::new(&full_path).exists() {
                errors.push(format!("Required asset not found: {}", full_path));
            }
        }
        
        // Validate workshop-specific requirements
        if package.definition.metadata.name.len() > 100 {
            errors.push("Workshop item names must be 100 characters or less".to_string());
        }
        
        if package.definition.metadata.description.len() > 8000 {
            errors.push("Workshop item descriptions must be 8000 characters or less".to_string());
        }
        
        if package.definition.metadata.tags.len() > 10 {
            warnings.push("Too many tags may reduce discoverability".to_string());
        }
        
        WorkshopValidationResult { errors, warnings }
    }
}

#[derive(Debug)]
pub struct WorkshopValidationResult {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl WorkshopValidationResult {
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
}

// Helper functions for workshop integration
pub struct WorkshopHelpers;

impl WorkshopHelpers {
    pub fn generate_preview_image(definition: &SmartObjectDefinition) -> String {
        // Generate a preview image filename based on object properties
        let category_prefix = definition.metadata.category.to_lowercase();
        let object_id = &definition.id;
        
        // For now, return a standardized preview path
        // In full implementation, this would render the actual object with:
        // 1. Object sprite with category background
        // 2. Key stats overlay (utility, range, etc.)
        // 3. Category icon
        format!("previews/{}_{}_preview.png", category_prefix, object_id)
    }
    
    pub fn create_workshop_tags(definition: &SmartObjectDefinition) -> Vec<String> {
        let mut tags = definition.metadata.tags.clone();
        
        // Add category as tag
        tags.push(definition.metadata.category.clone());
        
        // Add rarity as tag
        tags.push(definition.metadata.rarity.clone());
        
        // Add behavior tags based on provided actions
        for action in &definition.behavior.provides_actions {
            tags.push(action.to_lowercase());
        }
        
        // Deduplicate and return
        tags.sort();
        tags.dedup();
        tags
    }
    
    pub fn estimate_download_size(package: &WorkshopPackage) -> u64 {
        // Calculate estimated file sizes based on content
        let base_size = 1024u64; // Base package metadata size
        let definition_size = 2048u64; // RON definition file
        let preview_size = 50 * 1024u64; // Preview image estimate
        let sprite_size = 50 * 1024u64; // Estimated sprite size
        let script_size = if package.definition.workshop.custom_script_support { 10 * 1024u64 } else { 0u64 };
        
        // Count additional assets
        let assets_size = package.required_assets.len() as u64 * 25 * 1024u64; // Avg 25KB per asset
        
        base_size + definition_size + preview_size + sprite_size + script_size + assets_size
    }
}