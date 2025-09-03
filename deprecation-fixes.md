# Bevy Deprecation Fixes Applied

## EventWriter::send → EventWriter::write

Fixed deprecated `send()` method calls on EventWriter in the following files:

- `src/social_features/systems.rs` (5 instances)
- `src/photo_mode/advanced_ui.rs` (2 instances)  
- `src/ui_widgets.rs` (1 instance)
- `src/predator_prey.rs` (4 instances)

## Other Deprecations Remaining

- `EntityCommands::despawn_recursive()` → `entity.despawn()` (3 instances)
- `Query::get_single()` → `single()` (1 instance)

These can be addressed in future updates.