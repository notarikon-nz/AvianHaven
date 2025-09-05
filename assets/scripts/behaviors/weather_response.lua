-- Weather Response Behavior Script  
-- Handles bird reactions to weather conditions

function evaluate_behavior()
    local weather_fear = get_weather_fear()
    local shelter_urgency = get_weather_shelter_urgency()
    local fear_level = get_bird_fear()
    
    log_info("Weather evaluation - Fear: " .. weather_fear .. ", Shelter urgency: " .. shelter_urgency)
    
    -- Panic response to severe weather
    if weather_fear > 0.8 then
        log_info("Severe weather panic - fleeing!")
        set_bird_state("Fleeing")
        return true
    end
    
    -- High weather fear causes general fleeing
    if fear_level + weather_fear > 0.7 then
        log_info("High weather fear - fleeing to safety")
        set_bird_state("Fleeing") 
        return true
    end
    
    -- Seek shelter in bad weather
    if shelter_urgency > 0.6 and check_action_available("Shelter") then
        log_info("Seeking weather shelter")
        set_bird_state("MovingToTarget")
        return true
    end
    
    -- Light weather - prefer shelter if low energy
    if shelter_urgency > 0.3 and get_bird_energy() < 0.7 then
        if check_action_available("Shelter") then
            log_info("Light weather + low energy, seeking shelter")
            set_bird_state("MovingToTarget")
            return true
        end
    end
    
    -- Windy conditions affect small birds more
    local wind_strength = get_wind_strength()
    local bird_size = get_bird_size()
    
    if wind_strength > 0.5 and bird_size < 3 then
        local wind_fear = wind_strength * (4 - bird_size) * 0.2
        if wind_fear > 0.4 and check_action_available("Shelter") then
            log_info("Small bird seeking wind protection")
            set_bird_state("MovingToTarget")
            return true
        end
    end
    
    return false
end

-- Evaluate if weather makes feeding more urgent (needs to feed before storm)
function weather_feeding_urgency()
    local upcoming_storm = get_weather_forecast()
    local current_hunger = get_bird_hunger()
    
    if upcoming_storm and upcoming_storm.severity > 0.6 then
        -- Storm coming, increase feeding urgency
        return current_hunger * 1.4
    end
    
    return current_hunger
end

-- Check if weather conditions are good for certain activities
function is_good_weather_for_activity(activity)
    local weather_fear = get_weather_fear()
    local visibility = get_weather_visibility()
    
    if activity == "Foraging" then
        return weather_fear < 0.3 and visibility > 0.7
    elseif activity == "Bathing" then
        -- Birds like to bathe in light rain but not heavy rain
        local rain_intensity = get_rain_intensity()
        return rain_intensity > 0.1 and rain_intensity < 0.4
    elseif activity == "Flying" then
        local wind_strength = get_wind_strength() 
        return weather_fear < 0.4 and wind_strength < 0.6
    end
    
    return weather_fear < 0.5
end