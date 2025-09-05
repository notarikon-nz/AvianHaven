# Hanabi 0.16 API Migration Guide

## Overview
This guide documents the specific API changes and migration requirements for implementing Hanabi 0.16 GPU particle effects in Perch.

## Current Status
- ✅ **Framework Complete**: Full plugin architecture and event system implemented
- ✅ **Placeholder System**: Development continuity maintained
- ⚠️ **API Implementation**: Requires 0.16-specific patterns

## Critical API Changes from Previous Versions

### 1. Spawner System Changes
```rust
// Old Pattern (Pre-0.16)
let spawner = Spawner::rate(CpuValue::Single(100.0));

// New Pattern (0.16)
let spawner = Spawner::rate(100.0.into()); // or similar pattern
```
**Required**: Study official 0.16 spawner initialization patterns

### 2. EffectAsset Constructor Changes
```rust
// Migration Required
let effect = EffectAsset::new(capacity, spawner, module);
```
**Issue**: Parameter structure changed from vec![capacity] to different format
**Required**: Verify correct constructor signature in 0.16 docs

### 3. Module System Overhaul
```rust
// New Pattern Required (0.16)
let mut module = Module::default();
let writer = ExprWriter::new();
// Module building with mutable references
```
**Challenge**: Module creation and expression building patterns completely changed
**Required**: Study Module/ExprWriter patterns in 0.16 examples

### 4. Modifier API Updates
```rust
// Position Modifiers
SetPositionSphereModifier {
    center: Vec3::ZERO.into(),
    radius: 5.0.into(),
    dimension: ShapeDimension::Volume,
}

// Velocity Modifiers  
SetVelocityCircleModifier {
    center: Vec3::ZERO.into(),
    axis: Vec3::Y.into(),
    speed: Value::Uniform((50.0, 100.0).into()),
}

// Color Modifiers (NEW FIELDS REQUIRED)
ColorOverLifetimeModifier {
    gradient: Gradient::linear(Color::WHITE, Color::BLUE).into(),
    blend: ???, // NEW REQUIRED FIELD
    mask: ???,  // NEW REQUIRED FIELD  
}
```
**Critical**: blend and mask fields required - need 0.16 examples

### 5. Component/Bundle System Changes
```rust
// Old: ParticleEffectBundle
// New: Direct ParticleEffect component (likely)
commands.spawn(ParticleEffect::new(effect_handle));
```

## Implementation Strategy

### Phase 1: Basic Rain Effect (2-3 hours)
1. **API Research**: Study official 0.16 examples and documentation
2. **Minimal Implementation**: Single rain particle effect
3. **Compilation Success**: Verify all API patterns work
4. **Integration Test**: Connect to weather system

### Phase 2: Core Weather Effects (1-2 days)
1. **Snow Particles**: Cold weather precipitation
2. **Wind Effects**: Directional particle movement
3. **Performance Validation**: GPU acceleration benefits
4. **Visual Quality**: Professional-grade weather appearance

### Phase 3: Interactive Effects (1 day)
1. **Bird Splash**: Water interaction particles
2. **Seed Scatter**: Feeder interaction effects
3. **Event Integration**: Connect to existing bird behavior systems
4. **Responsive Particles**: Behavior-driven particle triggers

### Phase 4: Seasonal Enhancement (1 day)
1. **Falling Leaves**: Autumn atmospheric effects
2. **Pollen Particles**: Spring environmental effects
3. **Seasonal Integration**: Connect to time/weather systems
4. **Atmospheric Enhancement**: Visual immersion improvements

## Technical Requirements

### Dependencies
```toml
bevy_hanabi = "0.16"  # Currently commented in Cargo.toml
```

### Integration Points
- **Weather System**: `environment::components::Weather` enum integration
- **Time System**: `environment::resources::TimeState` seasonal connections
- **Bird Behavior**: `SpawnParticleEvent` for interactive effects
- **Performance**: GPU batch rendering, 10,000+ particle targets

### Expected Performance Benefits
- **10x Particle Count**: 1,000 → 10,000+ particles
- **CPU Relief**: 70% reduction in particle processing load
- **Memory Efficiency**: GPU buffer storage vs CPU per-particle tracking
- **Visual Quality**: Trail effects, complex behaviors, physics simulation

## Research Resources Needed

### Official Documentation
- [ ] Hanabi 0.16 API documentation review
- [ ] Official examples analysis
- [ ] Migration guide from previous versions
- [ ] Performance optimization patterns

### Test Implementation
- [ ] Minimal particle effect (single rain drop)
- [ ] Module/ExprWriter pattern verification
- [ ] Modifier API testing (all required fields)
- [ ] Component spawn/despawn lifecycle

## Risk Assessment

### High Risk
- **API Compatibility**: Significant structural changes in 0.16
- **Documentation Gaps**: Limited migration examples available
- **Timeline Impact**: API research could extend timeline

### Medium Risk  
- **Performance Integration**: GPU/CPU coordination
- **Visual Consistency**: Matching current CPU effect appearance
- **Memory Management**: GPU buffer lifecycle management

### Low Risk
- **Framework Integration**: Plugin architecture already complete
- **Event System**: SpawnParticleEvent system ready for GPU implementation
- **Development Continuity**: Placeholder system maintains functionality

## Success Criteria

### Technical Metrics
- [ ] **Compilation**: Clean build with all particle effects
- [ ] **Performance**: 60 FPS with 5,000+ particles active
- [ ] **GPU Utilization**: Verified GPU acceleration vs CPU fallback
- [ ] **Memory Usage**: Efficient GPU buffer management

### Visual Quality
- [ ] **Weather Effects**: Professional rain/snow appearance
- [ ] **Interactive Effects**: Responsive bird behavior particles
- [ ] **Atmospheric Enhancement**: Immersive seasonal effects
- [ ] **Performance Scaling**: Graceful degradation on lower-end hardware

## Next Steps

1. **API Study Session**: Dedicated 2-4 hours studying 0.16 documentation
2. **Minimal Test**: Single rain effect implementation and compilation
3. **Incremental Migration**: One effect type at a time with testing
4. **Performance Validation**: Confirm GPU acceleration benefits
5. **Full Integration**: Connect all weather/behavior/seasonal systems

## Current Framework Readiness

The placeholder system provides:
- ✅ Complete plugin architecture (`HanabiEffectsPlugin`)
- ✅ Event system (`SpawnParticleEvent`, `ParticleEffectType`)
- ✅ Resource management (`HanabiMigrationStatus`)
- ✅ Integration points (weather, seasons, bird behaviors)
- ✅ Development continuity (logging system tracks all particle events)

**Migration Path**: Replace placeholder implementations with actual GPU particle spawning using verified 0.16 API patterns.

**Timeline Estimate**: 5-8 days total (2-3 days API research, 3-5 days implementation)

**Immediate Priority**: API documentation study and minimal test implementation