use bevy::prelude::*;
use super::{components::*, resources::*};

pub fn setup_lighting_system(
    mut commands: Commands,
) {
    // Create main ambient light
    commands.spawn((
        DynamicAmbientLight {
            base_color: Color::srgb(1.0, 0.95, 0.85), // Warm white
            seasonal_tint: Color::WHITE,
            time_intensity: 1.0,
            weather_modifier: 1.0,
        },
        Transform::default(),
    ));
    
    // Create day/night overlay
    commands.spawn((
        Sprite::from_color(Color::srgba(0.0, 0.0, 0.2, 0.0), Vec2::new(2000.0, 2000.0)),
        Transform::from_xyz(0.0, 0.0, 100.0), // High Z to overlay everything
        DayNightOverlay {
            color: Color::srgba(0.0, 0.0, 0.2, 0.0),
            opacity: 0.0,
            blend_mode: DayNightBlendMode::Multiply,
        },
    ));
    
    // Create directional light (sun)
    commands.spawn((
        SunLight {
            direction: Vec3::new(0.3, -0.7, 0.6).normalize(),
            color: Color::srgb(1.0, 0.95, 0.8),
            intensity: 1.0,
            cast_shadows: true,
        },
        Transform::default(),
    ));
    
    // Create seasonal lighting controller
    commands.spawn((
        SeasonalLighting {
            spring_tint: Color::srgb(0.9, 1.0, 0.85), // Soft green tint
            summer_tint: Color::srgb(1.0, 0.95, 0.8), // Warm golden
            fall_tint: Color::srgb(1.0, 0.85, 0.7),   // Orange/amber
            winter_tint: Color::srgb(0.8, 0.9, 1.0),  // Cool blue
            transition_speed: 0.5,
        },
        Transform::default(),
    ));
}

pub fn dynamic_lighting_system(
    time_state: Res<TimeState>,
    weather_state: Res<WeatherState>,
    mut ambient_query: Query<&mut DynamicAmbientLight>,
    mut overlay_query: Query<(&mut Sprite, &mut DayNightOverlay)>,
    mut directional_query: Query<&mut SunLight>,
    seasonal_query: Query<&SeasonalLighting>,
    time: Res<Time>,
) {
    let season = time_state.get_season();
    let daylight_factor = time_state.daylight_factor();
    let weather_modifier = weather_state.current_weather.lighting_modifier();
    
    // Update ambient lighting
    if let Ok(mut ambient) = ambient_query.get_single_mut() {
        // Get seasonal tint
        if let Ok(seasonal) = seasonal_query.get_single() {
            ambient.seasonal_tint = get_seasonal_color(season, seasonal);
        }
        
        ambient.time_intensity = daylight_factor;
        ambient.weather_modifier = weather_modifier;
    }
    
    // Update day/night overlay
    if let Ok((mut sprite, mut overlay)) = overlay_query.get_single_mut() {
        let night_intensity = calculate_night_intensity(time_state.hour);
        overlay.opacity = night_intensity * weather_modifier;
        
        // Blend seasonal colors into night overlay
        let seasonal_night_color = match season {
            Season::Spring => Color::srgba(0.0, 0.05, 0.1, night_intensity * 0.6),
            Season::Summer => Color::srgba(0.05, 0.0, 0.05, night_intensity * 0.5),
            Season::Fall => Color::srgba(0.1, 0.05, 0.0, night_intensity * 0.7),
            Season::Winter => Color::srgba(0.0, 0.0, 0.15, night_intensity * 0.8),
        };
        
        overlay.color = seasonal_night_color;
        sprite.color = seasonal_night_color;
    }
    
    // Update directional light (sun position and color)
    if let Ok(mut light) = directional_query.get_single_mut() {
        // Calculate sun position based on time
        let sun_angle = (time_state.hour - 12.0) * std::f32::consts::PI / 12.0; // -π to π over 24 hours
        light.direction = Vec3::new(
            sun_angle.sin() * 0.5,
            -sun_angle.cos().abs(), // Always pointing down
            0.6
        ).normalize();
        
        // Sun color changes throughout day
        light.color = calculate_sun_color(time_state.hour, season);
        light.intensity = daylight_factor * weather_modifier;
    }
}

pub fn seasonal_lighting_transition_system(
    time_state: Res<TimeState>,
    mut seasonal_query: Query<&mut SeasonalLighting>,
    time: Res<Time>,
) {
    let Ok(mut seasonal) = seasonal_query.get_single_mut() else {
        return;
    };
    
    // Smoothly transition seasonal colors based on day of year
    let season = time_state.get_season();
    let season_progress = get_season_progress(time_state.day_of_year, season);
    
    // Gradually shift towards next season's colors
    seasonal.transition_speed = 0.5 + season_progress * 0.5; // Faster transition near season boundaries
}

fn calculate_night_intensity(hour: f32) -> f32 {
    if hour >= 6.0 && hour <= 18.0 {
        0.0 // Full daylight
    } else if hour < 6.0 {
        // Night to dawn transition
        let night_factor = (6.0 - hour) / 6.0;
        (night_factor * 0.8).min(0.8)
    } else {
        // Dusk to night transition
        let night_factor = (hour - 18.0) / 6.0;
        (night_factor * 0.8).min(0.8)
    }
}

fn calculate_sun_color(hour: f32, season: Season) -> Color {
    let base_color = if hour < 6.0 || hour > 20.0 {
        Color::srgb(0.3, 0.4, 0.8) // Night - cool blue
    } else if hour < 8.0 || hour > 18.0 {
        Color::srgb(1.0, 0.6, 0.3) // Dawn/dusk - warm orange
    } else if hour < 10.0 || hour > 16.0 {
        Color::srgb(1.0, 0.9, 0.7) // Morning/afternoon - golden
    } else {
        Color::srgb(1.0, 0.95, 0.85) // Midday - bright white
    };
    
    // Apply seasonal color modifications
    let [r, g, b, a] = base_color.to_srgba().to_f32_array();
    match season {
        Season::Spring => Color::srgba(r * 0.95, g * 1.0, b * 0.9, a),  // Slight green tint
        Season::Summer => Color::srgba(r * 1.0, g * 0.98, b * 0.85, a), // Warm golden
        Season::Fall => Color::srgba(r * 1.0, g * 0.9, b * 0.75, a),    // Amber/orange
        Season::Winter => Color::srgba(r * 0.9, g * 0.95, b * 1.0, a),  // Cool blue tint
    }
}

fn get_seasonal_color(season: Season, seasonal: &SeasonalLighting) -> Color {
    match season {
        Season::Spring => seasonal.spring_tint,
        Season::Summer => seasonal.summer_tint,
        Season::Fall => seasonal.fall_tint,
        Season::Winter => seasonal.winter_tint,
    }
}

fn get_season_progress(day_of_year: u32, current_season: Season) -> f32 {
    let season_start = match current_season {
        Season::Winter => 1,
        Season::Spring => 80,
        Season::Summer => 172,
        Season::Fall => 265,
    };
    
    let season_length = match current_season {
        Season::Winter => 79,
        Season::Spring => 91,
        Season::Summer => 92,
        Season::Fall => 90,
    };
    
    let days_into_season = if day_of_year >= season_start {
        day_of_year - season_start
    } else {
        // Handle winter wraparound
        day_of_year + (365 - season_start)
    };
    
    (days_into_season as f32 / season_length as f32).min(1.0)
}

pub fn weather_lighting_system(
    weather_state: Res<WeatherState>,
    mut ambient_query: Query<&mut DynamicAmbientLight>,
    mut directional_query: Query<&mut SunLight>,
) {
    let weather_modifier = weather_state.current_weather.lighting_modifier();
    
    // Update ambient light based on weather
    if let Ok(mut ambient) = ambient_query.get_single_mut() {
        ambient.weather_modifier = weather_modifier;
    }
    
    // Update directional light based on weather
    if let Ok(mut light) = directional_query.get_single_mut() {
        light.intensity *= weather_modifier;
        
        // Adjust sun color for weather conditions
        let [r, g, b, a] = light.color.to_srgba().to_f32_array();
        light.color = match weather_state.current_weather {
            Weather::Rainy => Color::srgba(r * 0.7, g * 0.8, b * 0.9, a), // Cool, muted
            Weather::Snowy => Color::srgba(r * 0.9, g * 0.9, b * 1.0, a), // Cool white
            Weather::Cloudy => Color::srgba(r * 0.8, g * 0.8, b * 0.85, a), // Slightly muted
            Weather::Windy => Color::srgba(r * 0.9, g * 0.85, b * 0.8, a), // Dusty
            Weather::Clear => light.color, // No modification
        };
    }
}