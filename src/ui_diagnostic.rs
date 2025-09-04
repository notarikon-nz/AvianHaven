// UI Diagnostic and Testing Module
// Comprehensive error checking and self-testing for UI systems

use bevy::prelude::*;
use crate::lunex_ui::{LunexJournalUI, LunexCatalogUI, BevyJournalUI};

pub struct UiDiagnosticPlugin;

impl Plugin for UiDiagnosticPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                diagnostic_journal_state,
                diagnostic_catalog_state,
                comprehensive_ui_health_check,
            ))
            .add_systems(OnEnter(crate::AppState::Journal), verify_journal_setup)
            .add_systems(OnEnter(crate::AppState::Catalog), verify_catalog_setup);
    }
}

// Diagnostic system for journal state
pub fn diagnostic_journal_state(
    journal_state: Res<crate::journal::resources::JournalState>,
    app_state: Res<State<crate::AppState>>,
    bevy_journal_query: Query<Entity, With<BevyJournalUI>>,
    lunex_journal_query: Query<Entity, With<LunexJournalUI>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        info!("🔍 JOURNAL DIAGNOSTIC:");
        info!("  App State: {:?}", app_state.get());
        info!("  Journal State Open: {}", journal_state.is_open);
        info!("  Current Tab: {:?}", journal_state.current_tab);
        info!("  Bevy UI Entities: {}", bevy_journal_query.iter().count());
        info!("  Lunex UI Entities: {}", lunex_journal_query.iter().count());
        
        match app_state.get() {
            crate::AppState::Journal => {
                if bevy_journal_query.is_empty() && lunex_journal_query.is_empty() {
                    error!("❌ JOURNAL DIAGNOSTIC: In Journal state but NO UI entities found!");
                } else {
                    info!("✅ JOURNAL DIAGNOSTIC: Journal state has UI entities");
                }
            }
            _ => {
                if !bevy_journal_query.is_empty() || !lunex_journal_query.is_empty() {
                    warn!("⚠️  JOURNAL DIAGNOSTIC: Not in Journal state but UI entities exist");
                }
            }
        }
    }
}

// Diagnostic system for catalog state
pub fn diagnostic_catalog_state(
    catalog_state: Res<crate::catalog::resources::CatalogState>,
    app_state: Res<State<crate::AppState>>,
    lunex_catalog_query: Query<Entity, With<LunexCatalogUI>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F2) {
        info!("🔍 CATALOG DIAGNOSTIC:");
        info!("  App State: {:?}", app_state.get());
        info!("  Catalog State Open: {}", catalog_state.is_open);
        info!("  Lunex UI Entities: {}", lunex_catalog_query.iter().count());
        
        match app_state.get() {
            crate::AppState::Catalog => {
                if lunex_catalog_query.is_empty() {
                    error!("❌ CATALOG DIAGNOSTIC: In Catalog state but NO UI entities found!");
                } else {
                    info!("✅ CATALOG DIAGNOSTIC: Catalog state has UI entities");
                }
            }
            _ => {
                if !lunex_catalog_query.is_empty() {
                    warn!("⚠️  CATALOG DIAGNOSTIC: Not in Catalog state but UI entities exist");
                }
            }
        }
    }
}

// Comprehensive health check system
pub fn comprehensive_ui_health_check(
    app_state: Res<State<crate::AppState>>,
    bevy_journal_query: Query<Entity, With<BevyJournalUI>>,
    lunex_journal_query: Query<Entity, With<LunexJournalUI>>,
    lunex_catalog_query: Query<Entity, With<LunexCatalogUI>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        info!("🏥 UI HEALTH CHECK:");
        info!("  Current App State: {:?}", app_state.get());
        
        // State-specific checks
        match app_state.get() {
            crate::AppState::Journal => {
                let bevy_count = bevy_journal_query.iter().count();
                let lunex_count = lunex_journal_query.iter().count();
                
                if bevy_count == 0 && lunex_count == 0 {
                    error!("🚨 CRITICAL: Journal state active but NO journal UI found!");
                    error!("🚨 Expected: At least 1 BevyJournalUI or LunexJournalUI entity");
                } else if bevy_count > 0 && lunex_count > 0 {
                    warn!("⚠️  WARNING: Both Bevy and Lunex journal UI active (potential conflict)");
                    warn!("⚠️  Bevy entities: {}, Lunex entities: {}", bevy_count, lunex_count);
                } else {
                    info!("✅ HEALTHY: Journal UI active (Bevy: {}, Lunex: {})", bevy_count, lunex_count);
                }
            }
            crate::AppState::Catalog => {
                let catalog_count = lunex_catalog_query.iter().count();
                
                if catalog_count == 0 {
                    error!("🚨 CRITICAL: Catalog state active but NO catalog UI found!");
                    error!("🚨 Expected: At least 1 LunexCatalogUI entity");
                } else {
                    info!("✅ HEALTHY: Catalog UI active ({} entities)", catalog_count);
                }
            }
            crate::AppState::Playing => {
                let total_ui = bevy_journal_query.iter().count() + 
                              lunex_journal_query.iter().count() + 
                              lunex_catalog_query.iter().count();
                
                if total_ui > 0 {
                    warn!("⚠️  WARNING: In Playing state but UI entities still exist");
                    warn!("⚠️  Bevy Journal: {}, Lunex Journal: {}, Lunex Catalog: {}", 
                         bevy_journal_query.iter().count(),
                         lunex_journal_query.iter().count(),
                         lunex_catalog_query.iter().count());
                } else {
                    info!("✅ HEALTHY: Playing state with clean UI");
                }
            }
            _ => {
                info!("ℹ️  State: {:?} (no specific UI checks)", app_state.get());
            }
        }
        
        info!("🏥 Health check completed. Use F1 (Journal) or F2 (Catalog) for detailed diagnostics.");
    }
}

// Verification system for journal setup
pub fn verify_journal_setup(
    bevy_journal_query: Query<Entity, With<BevyJournalUI>>,
    lunex_journal_query: Query<Entity, With<LunexJournalUI>>,
) {
    info!("🔍 JOURNAL VERIFY: Checking journal setup after state entry");
    
    let bevy_count = bevy_journal_query.iter().count();
    let lunex_count = lunex_journal_query.iter().count();
    
    if bevy_count == 0 && lunex_count == 0 {
        error!("❌ JOURNAL VERIFY: No journal UI entities found after setup!");
    } else {
        info!("✅ JOURNAL VERIFY: Found {} Bevy UI and {} Lunex UI entities", bevy_count, lunex_count);
    }
}

// Verification system for catalog setup
pub fn verify_catalog_setup(
    lunex_catalog_query: Query<Entity, With<LunexCatalogUI>>,
) {
    info!("🔍 CATALOG VERIFY: Checking catalog setup after state entry");
    
    let catalog_count = lunex_catalog_query.iter().count();
    
    if catalog_count == 0 {
        error!("❌ CATALOG VERIFY: No catalog UI entities found after setup!");
    } else {
        info!("✅ CATALOG VERIFY: Found {} catalog UI entities", catalog_count);
    }
}