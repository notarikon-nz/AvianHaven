use bevy::prelude::*;
use super::{components::*, resources::*};
use crate::notifications::{resources::ShowNotificationEvent, components::NotificationType};
use rand::Rng;

pub fn setup_environment(mut commands: Commands) {
    // Spawn sky background entity
    commands.spawn((
        Sprite::from_color(Color::srgb(0.7, 0.9, 1.0), Vec2::new(2000.0, 1500.0)),
        Transform::from_xyz(0.0, 0.0, -10.0), // Far background
        EnvironmentEntity,
    ));
}

pub fn time_progression_system(
    mut time_state: ResMut<TimeState>,
    mut seasonal_state: ResMut<SeasonalState>,
    mut time_events: EventWriter<TimeChangeEvent>,
    time: Res<Time>,
) {
    let old_hour = time_state.hour;
    let old_day = time_state.day_of_year;
    let old_season = time_state.get_season();
    
    // Progress time
    time_state.hour += time.delta().as_secs_f32() / time_state.time_speed;
    
    // Handle day rollover
    if time_state.hour >= 24.0 {
        time_state.hour -= 24.0;
        time_state.day_of_year += 1;
        
        if time_state.day_of_year > 365 {
            time_state.day_of_year = 1;
        }
        
        // Check for season change
        let new_season = time_state.get_season();
        if new_season != old_season {
            seasonal_state.update_for_season(new_season);
            
            info!("Season changed to {:?}! Available species updated.", new_season);
        }
    }
    
    // Send time change events for significant changes
    if (time_state.hour as u32) != (old_hour as u32) || time_state.day_of_year != old_day {
        time_events.write(TimeChangeEvent {
            new_hour: time_state.hour,
            new_day: time_state.day_of_year,
        });
    }
}

pub fn weather_system(
    mut weather_state: ResMut<WeatherState>,
    time_state: Res<TimeState>,
    mut weather_events: EventWriter<WeatherChangeEvent>,
    mut notification_events: EventWriter<ShowNotificationEvent>,
    time: Res<Time>,
) {
    weather_state.weather_timer.tick(time.delta());
    
    if weather_state.weather_timer.just_finished() {
        let mut rng = rand::rng();
        let season = time_state.get_season();
        
        // Weather probabilities based on season
        let new_weather = match season {
            Season::Spring => {
                match rng.random_range(0..10) {
                    0..=4 => Weather::Clear,
                    5..=6 => Weather::Cloudy,
                    7..=8 => Weather::Rainy,
                    _ => Weather::Windy,
                }
            },
            Season::Summer => {
                match rng.random_range(0..10) {
                    0..=6 => Weather::Clear,
                    7..=8 => Weather::Cloudy,
                    9 => Weather::Rainy,
                    _ => Weather::Windy,
                }
            },
            Season::Fall => {
                match rng.random_range(0..10) {
                    0..=3 => Weather::Clear,
                    4..=6 => Weather::Cloudy,
                    7..=8 => Weather::Rainy,
                    _ => Weather::Windy,
                }
            },
            Season::Winter => {
                match rng.random_range(0..10) {
                    0..=2 => Weather::Clear,
                    3..=5 => Weather::Cloudy,
                    6..=7 => Weather::Snowy,
                    8 => Weather::Rainy,
                    _ => Weather::Windy,
                }
            },
        };
        
        // Update temperature based on season and weather
        let base_temp = match season {
            Season::Spring => 15.0,
            Season::Summer => 25.0,
            Season::Fall => 10.0,
            Season::Winter => -2.0,
        };
        
        let temp_modifier = match new_weather {
            Weather::Clear => 3.0,
            Weather::Cloudy => 0.0,
            Weather::Rainy => -5.0,
            Weather::Snowy => -10.0,
            Weather::Windy => -2.0,
        };
        
        weather_state.temperature = base_temp + temp_modifier + rng.random_range(-3.0..3.0);
        
        if new_weather != weather_state.current_weather {
            let old_weather = weather_state.current_weather;
            weather_state.current_weather = new_weather;
            
            // Send weather change event
            weather_events.write(WeatherChangeEvent {
                new_weather,
                temperature: weather_state.temperature,
            });
            
            // Show weather notification for significant changes
            if matches!(new_weather, Weather::Rainy | Weather::Snowy) {
                let weather_name = match new_weather {
                    Weather::Rainy => "Rain",
                    Weather::Snowy => "Snow",
                    _ => "Weather Change",
                };
                
                notification_events.write(ShowNotificationEvent {
                    notification: NotificationType::Info {
                        message: format!("{} has begun! Bird activity may be reduced.", weather_name),
                    },
                });
            }
            
            info!("Weather changed from {:?} to {:?} ({}°C)", old_weather, new_weather, weather_state.temperature as i32);
        }
    }
}

pub fn seasonal_migration_system(
    time_state: Res<TimeState>,
    seasonal_state: Res<SeasonalState>,
    mut notification_events: EventWriter<ShowNotificationEvent>,
) {
    // Check for major migration events
    let season = time_state.get_season();
    
    // Spring migration peak (day 90-120)
    if time_state.day_of_year == 90 && season == Season::Spring {
        notification_events.write(ShowNotificationEvent {
            notification: NotificationType::Info {
                message: "Spring migration has begun! New species are arriving.".to_string(),
            },
        });
    }
    
    // Fall migration peak (day 265-295)
    if time_state.day_of_year == 265 && season == Season::Fall {
        notification_events.write(ShowNotificationEvent {
            notification: NotificationType::Info {
                message: "Fall migration is starting! Look for migrating species.".to_string(),
            },
        });
    }
}

pub fn environment_effect_system(
    time_state: Res<TimeState>,
    weather_state: Res<WeatherState>,
    mut sky_query: Query<&mut Sprite, With<EnvironmentEntity>>,
    mut weather_events: EventReader<WeatherChangeEvent>,
) {
    // Update sky color based on time and weather
    for mut sprite in &mut sky_query {
        let base_color = weather_state.current_weather.background_color();
        
        // Adjust for time of day
        let time_factor = if time_state.hour < 6.0 || time_state.hour > 20.0 {
            0.2 // Night - very dark
        } else if time_state.hour < 8.0 || time_state.hour > 18.0 {
            0.6 // Dawn/dusk - dimmed
        } else {
            1.0 // Day - full brightness
        };
        
        sprite.color = Color::srgb(
            base_color.to_srgba().red * time_factor,
            base_color.to_srgba().green * time_factor,
            base_color.to_srgba().blue * time_factor,
        );
    }
    
    // Handle weather change effects
    for event in weather_events.read() {
        match event.new_weather {
            Weather::Rainy => {
                info!("Rain effects started - bird activity reduced");
            },
            Weather::Snowy => {
                info!("Snow effects started - cold weather behavior activated");
            },
            Weather::Windy => {
                info!("Wind effects started - birds prefer sheltered feeders");
            },
            _ => {},
        }
    }
}