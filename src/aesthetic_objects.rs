use bevy::prelude::*;
use crate::environment::{components::Season, resources::TimeState};

pub struct AestheticObjectsPlugin;

impl Plugin for AestheticObjectsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_aesthetic_objects)
            .add_systems(Update, (
                seasonal_decoration_system,
                wind_sway_system,
                flower_bloom_system,
            ).run_if(in_state(crate::AppState::Playing)));
    }
}

// === AESTHETIC OBJECT TYPES ===

#[derive(Component)]
pub struct AestheticObject {
    pub object_type: AestheticType,
    pub seasonal_visibility: bool,    // Whether this object appears/disappears seasonally
    pub visual_appeal: f32,          // Contributes to photo composition scores
    pub environmental_story: String, // What story this object tells
}

#[derive(Component)]
pub struct SeasonalDecoration {
    pub active_seasons: Vec<Season>,
    pub transition_speed: f32,
    pub opacity: f32,
    pub scale_modifier: f32,
}

#[derive(Component)]
pub struct WindSway {
    pub sway_intensity: f32,
    pub sway_frequency: f32,
    pub base_rotation: f32,
    pub current_sway: f32,
}

#[derive(Component)]
pub struct FlowerBed {
    pub flower_type: FlowerType,
    pub bloom_stage: f32,        // 0.0 (dormant) to 1.0 (full bloom)
    pub peak_season: Season,
    pub color_intensity: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum AestheticType {
    // Natural decorations
    FlowerBed,
    FallenLog,
    MossyRock,
    Mushrooms,
    TallGrass,
    Wildflowers,
    
    // Seasonal decorations
    SpringBuds,
    SummerFlowers,
    FallLeaves,
    WinterBerries,
    
    // Background elements
    BackgroundTrees,
    DistantHills,
    CloudShadows,
    SunBeams,
    
    // Artificial decorations
    GardenOrnament,
    Pathway,
    PicketFence,
    MailboxPost,
}

#[derive(Debug, Clone, Copy)]
pub enum FlowerType {
    Tulips,      // Early spring
    Daffodils,   // Early spring
    Lilacs,      // Late spring
    Roses,       // Summer
    Sunflowers,  // Late summer
    Mums,        // Fall
    Pansies,     // Cool weather
}

impl AestheticType {
    pub fn base_color(&self) -> Color {
        match self {
            Self::FlowerBed => Color::srgb(0.8, 0.3, 0.6),
            Self::FallenLog => Color::srgb(0.4, 0.3, 0.2),
            Self::MossyRock => Color::srgb(0.2, 0.4, 0.2),
            Self::Mushrooms => Color::srgb(0.9, 0.8, 0.7),
            Self::TallGrass => Color::srgb(0.3, 0.5, 0.2),
            Self::Wildflowers => Color::srgb(0.9, 0.8, 0.3),
            
            Self::SpringBuds => Color::srgb(0.6, 0.8, 0.4),
            Self::SummerFlowers => Color::srgb(0.9, 0.7, 0.2),
            Self::FallLeaves => Color::srgb(0.9, 0.5, 0.2),
            Self::WinterBerries => Color::srgb(0.8, 0.2, 0.2),
            
            Self::BackgroundTrees => Color::srgb(0.2, 0.4, 0.2),
            Self::DistantHills => Color::srgb(0.4, 0.5, 0.6),
            Self::CloudShadows => Color::srgba(0.3, 0.3, 0.4, 0.3),
            Self::SunBeams => Color::srgba(1.0, 0.9, 0.7, 0.2),
            
            Self::GardenOrnament => Color::srgb(0.7, 0.6, 0.5),
            Self::Pathway => Color::srgb(0.6, 0.5, 0.4),
            Self::PicketFence => Color::srgb(0.9, 0.9, 0.9),
            Self::MailboxPost => Color::srgb(0.3, 0.3, 0.8),
        }
    }
    
    pub fn seasonal_appeal(&self, season: Season) -> f32 {
        match (self, season) {
            (Self::SpringBuds, Season::Spring) => 1.0,
            (Self::SummerFlowers, Season::Summer) => 1.0,
            (Self::FallLeaves, Season::Fall) => 1.0,
            (Self::WinterBerries, Season::Winter) => 1.0,
            
            (Self::FlowerBed, Season::Spring | Season::Summer) => 0.9,
            (Self::Wildflowers, Season::Spring | Season::Summer) => 0.8,
            (Self::TallGrass, Season::Summer | Season::Fall) => 0.7,
            
            // Year-round objects
            (Self::FallenLog | Self::MossyRock | Self::Mushrooms, _) => 0.6,
            (Self::BackgroundTrees | Self::DistantHills, _) => 0.4,
            (Self::GardenOrnament | Self::Pathway | Self::PicketFence, _) => 0.3,
            
            _ => 0.2, // Minimal appeal outside ideal seasons
        }
    }
    
    pub fn provides_photo_bonus(&self) -> u32 {
        match self {
            // High visual interest objects
            Self::SummerFlowers | Self::FallLeaves | Self::SpringBuds => 15,
            Self::FlowerBed | Self::Wildflowers => 12,
            Self::SunBeams | Self::CloudShadows => 10,
            
            // Background composition elements
            Self::BackgroundTrees | Self::DistantHills => 8,
            Self::TallGrass | Self::MossyRock => 6,
            
            // Storytelling elements
            Self::PicketFence | Self::Pathway => 5,
            Self::GardenOrnament | Self::MailboxPost => 4,
            
            _ => 3,
        }
    }
}

impl FlowerType {
    pub fn bloom_season(&self) -> Season {
        match self {
            Self::Tulips | Self::Daffodils => Season::Spring,
            Self::Lilacs => Season::Spring,
            Self::Roses | Self::Sunflowers => Season::Summer,
            Self::Mums => Season::Fall,
            Self::Pansies => Season::Winter, // Cool weather flowers
        }
    }
    
    pub fn color(&self) -> Color {
        match self {
            Self::Tulips => Color::srgb(0.9, 0.3, 0.6),
            Self::Daffodils => Color::srgb(1.0, 0.9, 0.2),
            Self::Lilacs => Color::srgb(0.7, 0.5, 0.9),
            Self::Roses => Color::srgb(0.9, 0.2, 0.3),
            Self::Sunflowers => Color::srgb(1.0, 0.8, 0.1),
            Self::Mums => Color::srgb(0.8, 0.5, 0.2),
            Self::Pansies => Color::srgb(0.6, 0.3, 0.8),
        }
    }
}

// === SETUP SYSTEMS ===

pub fn setup_aesthetic_objects(mut commands: Commands) {
    spawn_natural_decorations(&mut commands);
    spawn_seasonal_decorations(&mut commands);
    spawn_background_elements(&mut commands);
    spawn_artificial_decorations(&mut commands);
    spawn_flower_beds(&mut commands);
}

fn spawn_natural_decorations(commands: &mut Commands) {
    // Fallen log - provides rustic charm and perching
    commands.spawn((
        Sprite::from_color(
            AestheticType::FallenLog.base_color(), 
            Vec2::new(120.0, 25.0)
        ),
        Transform::from_xyz(-100.0, -180.0, 0.1),
        AestheticObject {
            object_type: AestheticType::FallenLog,
            seasonal_visibility: true,
            visual_appeal: 0.7,
            environmental_story: "A weathered log provides natural perching".to_string(),
        },
    ));
    
    // Mossy rock cluster
    commands.spawn((
        Sprite::from_color(
            AestheticType::MossyRock.base_color(), 
            Vec2::new(80.0, 60.0)
        ),
        Transform::from_xyz(150.0, -150.0, 0.2),
        AestheticObject {
            object_type: AestheticType::MossyRock,
            seasonal_visibility: true,
            visual_appeal: 0.6,
            environmental_story: "Ancient rocks covered in soft moss".to_string(),
        },
    ));
    
    // Tall grass clusters
    for i in 0..3 {
        let x_pos = -200.0 + i as f32 * 80.0;
        commands.spawn((
            Sprite::from_color(
                AestheticType::TallGrass.base_color(), 
                Vec2::new(40.0, 80.0)
            ),
            Transform::from_xyz(x_pos, -200.0, 0.15),
            AestheticObject {
                object_type: AestheticType::TallGrass,
                seasonal_visibility: true,
                visual_appeal: 0.5,
                environmental_story: "Swaying grass provides natural movement".to_string(),
            },
            WindSway {
                sway_intensity: 0.3,
                sway_frequency: 2.0,
                base_rotation: 0.0,
                current_sway: 0.0,
            },
        ));
    }
    
    // Mushroom ring
    for i in 0..5 {
        let angle = i as f32 * std::f32::consts::TAU / 5.0;
        let radius = 40.0;
        let x = angle.cos() * radius + 80.0;
        let y = angle.sin() * radius - 120.0;
        
        commands.spawn((
            Sprite::from_color(
                AestheticType::Mushrooms.base_color(), 
                Vec2::new(15.0, 20.0)
            ),
            Transform::from_xyz(x, y, 0.2),
            AestheticObject {
                object_type: AestheticType::Mushrooms,
                seasonal_visibility: true,
                visual_appeal: 0.4,
                environmental_story: "Fairy ring mushrooms add woodland magic".to_string(),
            },
        ));
    }
}

fn spawn_seasonal_decorations(commands: &mut Commands) {
    // Spring buds on branches
    commands.spawn((
        Sprite::from_color(
            AestheticType::SpringBuds.base_color(), 
            Vec2::new(60.0, 40.0)
        ),
        Transform::from_xyz(-150.0, 120.0, 0.25),
        AestheticObject {
            object_type: AestheticType::SpringBuds,
            seasonal_visibility: true,
            visual_appeal: 0.8,
            environmental_story: "Fresh buds herald spring's arrival".to_string(),
        },
        SeasonalDecoration {
            active_seasons: vec![Season::Spring],
            transition_speed: 0.5,
            opacity: 1.0,
            scale_modifier: 1.0,
        },
    ));
    
    // Summer flower clusters
    commands.spawn((
        Sprite::from_color(
            AestheticType::SummerFlowers.base_color(), 
            Vec2::new(90.0, 50.0)
        ),
        Transform::from_xyz(120.0, -80.0, 0.25),
        AestheticObject {
            object_type: AestheticType::SummerFlowers,
            seasonal_visibility: true,
            visual_appeal: 0.9,
            environmental_story: "Vibrant summer blooms attract both birds and photographers".to_string(),
        },
        SeasonalDecoration {
            active_seasons: vec![Season::Summer],
            transition_speed: 0.3,
            opacity: 1.0,
            scale_modifier: 1.0,
        },
    ));
    
    // Fall leaves (ground scatter)
    for i in 0..8 {
        let x = -250.0 + i as f32 * 60.0;
        let y = -180.0 + (i % 3) as f32 * 30.0;
        
        commands.spawn((
            Sprite::from_color(
                AestheticType::FallLeaves.base_color(), 
                Vec2::new(25.0, 25.0)
            ),
            Transform::from_xyz(x, y, 0.05),
            AestheticObject {
                object_type: AestheticType::FallLeaves,
                seasonal_visibility: true,
                visual_appeal: 0.7,
                environmental_story: "Autumn leaves create a carpet of color".to_string(),
            },
            SeasonalDecoration {
                active_seasons: vec![Season::Fall],
                transition_speed: 0.2,
                opacity: 1.0,
                scale_modifier: 1.0,
            },
        ));
    }
    
    // Winter berries on bushes
    commands.spawn((
        Sprite::from_color(
            AestheticType::WinterBerries.base_color(), 
            Vec2::new(70.0, 50.0)
        ),
        Transform::from_xyz(-180.0, 60.0, 0.3),
        AestheticObject {
            object_type: AestheticType::WinterBerries,
            seasonal_visibility: true,
            visual_appeal: 0.8,
            environmental_story: "Bright berries provide winter food and color".to_string(),
        },
        SeasonalDecoration {
            active_seasons: vec![Season::Winter, Season::Fall],
            transition_speed: 0.4,
            opacity: 1.0,
            scale_modifier: 1.0,
        },
    ));
}

fn spawn_background_elements(commands: &mut Commands) {
    // Background trees (distant)
    for i in 0..6 {
        let x = -300.0 + i as f32 * 120.0;
        let tree_height = 150.0 + (i % 3) as f32 * 40.0;
        
        commands.spawn((
            Sprite::from_color(
                AestheticType::BackgroundTrees.base_color(), 
                Vec2::new(40.0, tree_height)
            ),
            Transform::from_xyz(x, 200.0 + tree_height / 2.0, -0.5), // Behind everything
            AestheticObject {
                object_type: AestheticType::BackgroundTrees,
                seasonal_visibility: true,
                visual_appeal: 0.5,
                environmental_story: "Distant forest creates depth and natural setting".to_string(),
            },
            WindSway {
                sway_intensity: 0.1,
                sway_frequency: 0.5,
                base_rotation: 0.0,
                current_sway: 0.0,
            },
        ));
    }
    
    // Distant hills
    commands.spawn((
        Sprite::from_color(
            AestheticType::DistantHills.base_color(), 
            Vec2::new(800.0, 100.0)
        ),
        Transform::from_xyz(0.0, 250.0, -1.0), // Far background
        AestheticObject {
            object_type: AestheticType::DistantHills,
            seasonal_visibility: true,
            visual_appeal: 0.3,
            environmental_story: "Rolling hills frame the bird watching area".to_string(),
        },
    ));
    
    // Moving cloud shadows (dynamic aesthetic element)
    for i in 0..3 {
        commands.spawn((
            Sprite::from_color(
                AestheticType::CloudShadows.base_color(), 
                Vec2::new(150.0, 80.0)
            ),
            Transform::from_xyz(-200.0 + i as f32 * 200.0, 50.0, 0.8), // Above ground objects
            AestheticObject {
                object_type: AestheticType::CloudShadows,
                seasonal_visibility: true,
                visual_appeal: 0.4,
                environmental_story: "Drifting cloud shadows add dynamic lighting".to_string(),
            },
            CloudShadow {
                drift_speed: 20.0 + i as f32 * 10.0,
                direction: Vec2::new(1.0, 0.2),
                opacity_cycle: 0.0,
            },
        ));
    }
}

fn spawn_artificial_decorations(commands: &mut Commands) {
    // Garden ornament (birdbath-adjacent)
    commands.spawn((
        Sprite::from_color(
            AestheticType::GardenOrnament.base_color(), 
            Vec2::new(30.0, 40.0)
        ),
        Transform::from_xyz(-50.0, -80.0, 0.3),
        AestheticObject {
            object_type: AestheticType::GardenOrnament,
            seasonal_visibility: true,
            visual_appeal: 0.6,
            environmental_story: "Garden ornament adds human touch to natural space".to_string(),
        },
    ));
    
    // Winding pathway
    for i in 0..5 {
        let x = -150.0 + i as f32 * 75.0;
        let y = -220.0 + (i as f32 * 20.0).sin() * 15.0; // Curved path
        
        commands.spawn((
            Sprite::from_color(
                AestheticType::Pathway.base_color(), 
                Vec2::new(40.0, 20.0)
            ),
            Transform::from_xyz(x, y, 0.05),
            AestheticObject {
                object_type: AestheticType::Pathway,
                seasonal_visibility: true,
                visual_appeal: 0.4,
                environmental_story: "Garden path guides the eye through the scene".to_string(),
            },
        ));
    }
    
    // Picket fence section
    commands.spawn((
        Sprite::from_color(
            AestheticType::PicketFence.base_color(), 
            Vec2::new(200.0, 60.0)
        ),
        Transform::from_xyz(200.0, -200.0, 0.2),
        AestheticObject {
            object_type: AestheticType::PicketFence,
            seasonal_visibility: true,
            visual_appeal: 0.5,
            environmental_story: "White picket fence frames the garden space".to_string(),
        },
    ));
}

fn spawn_flower_beds(commands: &mut Commands) {
    // Spring tulip bed
    commands.spawn((
        Sprite::from_color(
            FlowerType::Tulips.color(), 
            Vec2::new(80.0, 40.0)
        ),
        Transform::from_xyz(-50.0, 120.0, 0.25),
        FlowerBed {
            flower_type: FlowerType::Tulips,
            bloom_stage: 0.0,
            peak_season: Season::Spring,
            color_intensity: 0.8,
        },
        AestheticObject {
            object_type: AestheticType::FlowerBed,
            seasonal_visibility: true,
            visual_appeal: 0.9,
            environmental_story: "Colorful tulips mark the arrival of spring".to_string(),
        },
    ));
    
    // Summer rose garden
    commands.spawn((
        Sprite::from_color(
            FlowerType::Roses.color(), 
            Vec2::new(100.0, 60.0)
        ),
        Transform::from_xyz(80.0, 140.0, 0.25),
        FlowerBed {
            flower_type: FlowerType::Roses,
            bloom_stage: 0.0,
            peak_season: Season::Summer,
            color_intensity: 0.9,
        },
        AestheticObject {
            object_type: AestheticType::FlowerBed,
            seasonal_visibility: true,
            visual_appeal: 1.0,
            environmental_story: "Rose garden provides classic beauty and fragrance".to_string(),
        },
    ));
    
    // Fall mums
    commands.spawn((
        Sprite::from_color(
            FlowerType::Mums.color(), 
            Vec2::new(70.0, 45.0)
        ),
        Transform::from_xyz(180.0, 120.0, 0.25),
        FlowerBed {
            flower_type: FlowerType::Mums,
            bloom_stage: 0.0,
            peak_season: Season::Fall,
            color_intensity: 0.8,
        },
        AestheticObject {
            object_type: AestheticType::FlowerBed,
            seasonal_visibility: true,
            visual_appeal: 0.8,
            environmental_story: "Autumn mums bring warmth to the cooling season".to_string(),
        },
    ));
}

// === DYNAMIC SYSTEMS ===

#[derive(Component)]
pub struct CloudShadow {
    pub drift_speed: f32,
    pub direction: Vec2,
    pub opacity_cycle: f32,
}

pub fn seasonal_decoration_system(
    time_state: Res<TimeState>,
    mut seasonal_query: Query<(&mut Visibility, &mut Sprite, &SeasonalDecoration, &AestheticObject)>,
    time: Res<Time>,
) {
    let current_season = time_state.get_season();
    
    for (mut visibility, mut sprite, decoration, aesthetic) in &mut seasonal_query {
        let should_be_visible = decoration.active_seasons.contains(&current_season);
        
        if should_be_visible {
            *visibility = Visibility::Inherited;
            
            // Gradually increase opacity and scale when coming into season
            let target_opacity = 1.0;
            let current_opacity = sprite.color.alpha();
            
            if current_opacity < target_opacity {
                let new_opacity = (current_opacity + decoration.transition_speed * time.delta().as_secs_f32())
                    .min(target_opacity);
                let [r, g, b, _] = sprite.color.to_srgba().to_f32_array();
                sprite.color = Color::srgba(r, g, b, new_opacity);
            }
        } else {
            // Gradually fade out when going out of season
            let current_opacity = sprite.color.alpha();
            
            if current_opacity > 0.0 {
                let new_opacity = (current_opacity - decoration.transition_speed * time.delta().as_secs_f32())
                    .max(0.0);
                let [r, g, b, _] = sprite.color.to_srgba().to_f32_array();
                sprite.color = Color::srgba(r, g, b, new_opacity);
                
                if new_opacity <= 0.0 {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}

pub fn wind_sway_system(
    mut sway_query: Query<(&mut Transform, &mut WindSway)>,
    weather_state: Res<crate::environment::resources::WeatherState>,
    time: Res<Time>,
) {
    let wind_modifier = match weather_state.current_weather {
        crate::environment::components::Weather::Windy => 2.0,
        crate::environment::components::Weather::Clear => 1.0,
        crate::environment::components::Weather::Rainy => 1.5,
        _ => 0.5,
    };
    
    for (mut transform, mut wind_sway) in &mut sway_query {
        // Calculate sway using sine wave
        wind_sway.current_sway += wind_sway.sway_frequency * time.delta().as_secs_f32();
        
        let sway_amount = wind_sway.current_sway.sin() * wind_sway.sway_intensity * wind_modifier;
        transform.rotation = Quat::from_rotation_z(wind_sway.base_rotation + sway_amount);
    }
}

pub fn flower_bloom_system(
    time_state: Res<TimeState>,
    mut flower_query: Query<(&mut Sprite, &mut FlowerBed)>,
    time: Res<Time>,
) {
    let current_season = time_state.get_season();
    
    for (mut sprite, mut flower_bed) in &mut flower_query {
        let in_bloom_season = flower_bed.flower_type.bloom_season() == current_season;
        
        if in_bloom_season {
            // Blooming - increase bloom stage
            flower_bed.bloom_stage = (flower_bed.bloom_stage + 0.5 * time.delta().as_secs_f32()).min(1.0);
        } else {
            // Out of season - decrease bloom stage
            flower_bed.bloom_stage = (flower_bed.bloom_stage - 0.3 * time.delta().as_secs_f32()).max(0.0);
        }
        
        // Adjust visual appearance based on bloom stage
        let base_color = flower_bed.flower_type.color();
        let [r, g, b, _] = base_color.to_srgba().to_f32_array();
        let intensity = flower_bed.bloom_stage * flower_bed.color_intensity;
        
        sprite.color = Color::srgba(
            r * intensity + 0.3 * (1.0 - intensity), // Fade towards brown when not blooming
            g * intensity + 0.3 * (1.0 - intensity),
            b * intensity + 0.2 * (1.0 - intensity),
            1.0
        );
    }
}

// Cloud shadow movement system
pub fn cloud_shadow_system(
    mut shadow_query: Query<(&mut Transform, &mut CloudShadow)>,
    time: Res<Time>,
) {
    for (mut transform, mut shadow) in &mut shadow_query {
        // Move cloud shadows across the scene
        let movement = shadow.direction * shadow.drift_speed * time.delta().as_secs_f32();
        transform.translation += movement.extend(0.0);
        
        // Reset position when shadow moves off screen
        if transform.translation.x > 400.0 {
            transform.translation.x = -400.0;
        }
        
        // Cycle opacity for realistic cloud shadow effects
        shadow.opacity_cycle += time.delta().as_secs_f32() * 0.5;
        let opacity = 0.1 + 0.2 * (shadow.opacity_cycle.sin() * 0.5 + 0.5);
        
        let base_color = AestheticType::CloudShadows.base_color();
        let [r, g, b, _] = base_color.to_srgba().to_f32_array();
        // Update sprite color would require access to Sprite component
    }
}

// === PHOTO MODE INTEGRATION ===

// This function can be called from the photo mode scoring system
// to add environmental richness bonuses
pub fn calculate_aesthetic_bonus(
    camera_pos: Vec2,
    aesthetic_query: &Query<(&Transform, &AestheticObject)>,
    season: Season,
) -> u32 {
    let mut bonus = 0;
    let mut visible_aesthetics = Vec::new();
    
    // Find aesthetic objects in frame (within camera view)
    for (transform, aesthetic) in aesthetic_query.iter() {
        let distance = camera_pos.distance(transform.translation.truncate());
        if distance < 300.0 { // Within camera frame
            let seasonal_appeal = aesthetic.object_type.seasonal_appeal(season);
            if seasonal_appeal > 0.3 {
                visible_aesthetics.push((aesthetic, seasonal_appeal));
            }
        }
    }
    
    // Calculate bonus based on variety and seasonal appropriateness
    bonus += match visible_aesthetics.len() {
        0 => 0,
        1..=2 => 5,
        3..=4 => 12,
        5..=6 => 20,
        _ => 25,
    };
    
    // Additional bonus for highly seasonal objects
    let seasonal_bonus: u32 = visible_aesthetics.iter()
        .filter(|(_, appeal)| *appeal > 0.8)
        .map(|(aesthetic, _)| aesthetic.object_type.provides_photo_bonus())
        .sum();
    
    bonus + seasonal_bonus.min(15) // Cap seasonal bonus at 15
}