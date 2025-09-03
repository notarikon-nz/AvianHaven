# Bevy Lunex UI Migration Plan

## Overview
Migrating from Bevy's built-in UI system to bevy_lunex for improved performance, flexibility, and advanced layout capabilities.

## Migration Strategy
Gradual migration approach to minimize disruption and allow testing at each stage.

### Phase 1: Foundation (CURRENT)
- ✅ Add bevy_lunex dependency 
- ✅ Create LunexUiPlugin with basic setup
- ✅ Define migration utilities and components
- ✅ Create LunexContainer abstraction layer

### Phase 2: Simple Elements (NEXT - 1 day)
**Priority 1 - Settings Menu Buttons**
- Convert SettingsButton components to Lunex equivalents
- Migrate text labels in settings menu
- Test button interactions and navigation

**Priority 2 - Menu Navigation**
- Convert MainMenuButton components  
- Migrate menu backgrounds and containers
- Ensure state transitions still work

### Phase 3: Layout Systems (2 days)
**Priority 3 - Container Migration**
- Convert Node-based layouts to Lunex containers
- Migrate flexbox layouts to Lunex equivalents
- Update menu container hierarchies

**Priority 4 - Interactive Widgets**
- Convert SliderWidget to Lunex sliders
- Migrate GraphicsToggle components
- Update keybinding display elements

### Phase 4: Complex UI (3 days)
**Priority 5 - Journal System**
- Convert JournalTab navigation
- Migrate species cards and detail panels
- Update research mission UI

**Priority 6 - Advanced Features**
- Convert Tooltip system to Lunex
- Migrate photo mode UI overlays
- Update achievement notification system

## Technical Benefits
- **Performance**: Lunex uses more efficient rendering
- **Flexibility**: Better layout algorithms and responsive design
- **Advanced Features**: Built-in animations and state management
- **Maintainability**: Cleaner component architecture

## Implementation Notes
- Maintain backward compatibility during migration
- Use LunexMigrationMarker for tracking converted components
- Test each phase thoroughly before proceeding
- Keep existing UI as fallback until full migration complete

## Current Status: Phase 1 Complete
Ready to begin Phase 2 - Simple Elements migration.