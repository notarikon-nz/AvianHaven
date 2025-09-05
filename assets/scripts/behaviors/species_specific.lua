-- Species-Specific Behavior Script
-- Handles unique behaviors for different bird species

function evaluate_behavior()
    local species = get_bird_species()
    local energy = get_bird_energy()
    local time = get_time_of_day()
    
    -- Route to species-specific behavior
    if species == "RubyThroatedHummingbird" then
        return evaluate_hummingbird_behavior()
    elseif species == "BlueJay" then 
        return evaluate_bluejay_behavior()
    elseif species == "Chickadee" then
        return evaluate_chickadee_behavior()
    elseif species == "RedTailedHawk" then
        return evaluate_hawk_behavior()
    elseif species == "GreatHornedOwl" then
        return evaluate_owl_behavior()
    else
        return evaluate_default_species_behavior()
    end
end

-- Hummingbird-specific behaviors
function evaluate_hummingbird_behavior() 
    local energy = get_bird_energy()
    local nectar_available = check_action_available("HoverFeed")
    
    -- Hummingbirds have very high metabolic needs
    if energy < 0.4 then
        if nectar_available then
            log_info("Hummingbird urgent nectar feeding")
            set_bird_state("MovingToTarget")
            return true
        else
            -- Emergency feeding on any available food
            if check_action_available("Eat") then
                log_info("Hummingbird emergency feeding")
                set_bird_state("MovingToTarget") 
                return true
            end
        end
    end
    
    -- Territorial defense of nectar sources
    if get_bird_territorial_stress() > 0.4 and nectar_available then
        log_info("Hummingbird defending nectar territory")
        set_bird_state("Territorial")
        return true
    end
    
    return false
end

-- Blue Jay aggressive and intelligent behaviors
function evaluate_bluejay_behavior()
    local intelligence = get_bird_intelligence()
    local dominance = get_bird_dominance()
    local nearby_birds = get_nearby_bird_count()
    
    -- Blue jays are very aggressive at feeders
    if dominance > 0.7 and nearby_birds > 0 then
        if check_action_available("Eat") then
            log_info("Blue Jay asserting feeding dominance")
            set_bird_state("MovingToTarget")
            -- Could also intimidate other birds
            intimidate_nearby_birds()
            return true
        end
    end
    
    -- Cache acorns and nuts intelligently
    if intelligence > 0.8 and get_season() == "Fall" then
        if check_action_available("Cache") then
            log_info("Blue Jay caching nuts for winter")
            set_bird_state("MovingToTarget")
            return true
        end
    end
    
    -- Mobbing behavior against predators
    if detect_nearby_predator() then
        log_info("Blue Jay mobbing predator")
        set_bird_state("Territorial") 
        call_for_mobbing_assistance()
        return true
    end
    
    return false
end

-- Chickadee social and acrobatic behaviors  
function evaluate_chickadee_behavior()
    local social_need = get_bird_social_need()
    local energy = get_bird_energy()
    
    -- Chickadees are highly social, rarely alone
    if social_need > 0.3 and get_flock_size() < 2 then
        log_info("Chickadee seeking flock companions")
        set_bird_state("Flocking")
        return true
    end
    
    -- Acrobatic feeding on small branches and suet
    if energy > 0.6 and check_action_available("Eat") then
        local feeder_type = get_target_feeder_type()
        if feeder_type == "Suet" then
            log_info("Chickadee acrobatic suet feeding")
            set_bird_state("MovingToTarget")
            set_feeding_style("acrobatic")
            return true
        end
    end
    
    -- Winter survival huddling
    if get_season() == "Winter" and get_temperature() < 0.3 then
        if social_need < 0.8 then -- Don't override strong social needs
            log_info("Chickadee winter huddling")
            set_bird_state("Flocking")
            set_social_behavior("huddling")
            return true
        end
    end
    
    return false
end

-- Hawk predatory behaviors
function evaluate_hawk_behavior()
    local hunger = get_bird_hunger()
    local energy = get_bird_energy()
    local hunting_success_rate = get_hunting_success_rate()
    
    -- Hawks hunt other birds - different from regular feeding
    if hunger > 0.6 and energy > 0.5 then
        local prey_detected = detect_suitable_prey()
        if prey_detected then
            log_info("Hawk detected prey, initiating hunt")
            set_bird_state("Hunting")
            return true
        end
    end
    
    -- Soaring behavior to conserve energy while hunting
    if energy > 0.4 and hunger > 0.3 then
        if is_good_soaring_conditions() then
            log_info("Hawk soaring to search for prey")
            set_bird_state("Soaring")
            return true
        end
    end
    
    -- Territorial defense of hunting grounds
    if detect_rival_predator() then
        log_info("Hawk defending hunting territory")
        set_bird_state("Territorial")
        return true
    end
    
    return false
end

-- Owl nocturnal behaviors
function evaluate_owl_behavior()
    local time = get_time_of_day()
    local hunger = get_bird_hunger()
    local energy = get_bird_energy()
    
    -- Owls are primarily nocturnal
    if time >= 20.0 or time <= 6.0 then
        -- Night hunting behavior
        if hunger > 0.5 and energy > 0.4 then
            local prey_detected = detect_small_mammals()
            if prey_detected then
                log_info("Owl night hunting")
                set_bird_state("Hunting")
                return true
            end
        end
        
        -- Silent flight patrol
        if energy > 0.6 then
            log_info("Owl silent patrol")
            set_bird_state("Patrolling")
            set_flight_mode("silent")
            return true
        end
    else
        -- Daytime roosting behavior
        if energy < 0.8 then
            log_info("Owl daytime roosting")
            set_bird_state("Roosting") 
            return true
        end
    end
    
    return false
end

-- Default behaviors for common species
function evaluate_default_species_behavior()
    local species_traits = get_species_traits()
    local energy = get_bird_energy()
    
    -- Species-specific feeding preferences
    local preferred_food = species_traits.preferred_food_type
    if check_food_type_available(preferred_food) then
        log_info("Species feeding on preferred food: " .. preferred_food)
        set_bird_state("MovingToTarget")
        return true
    end
    
    -- Species-specific social behaviors
    if species_traits.sociability > 0.7 then
        if get_flock_size() < species_traits.preferred_flock_size then
            log_info("Social species seeking larger flock")
            set_bird_state("Flocking")
            return true
        end
    end
    
    return false
end

-- Helper functions for species-specific behaviors
function intimidate_nearby_birds()
    -- Blue Jay intimidation behavior
    play_aggressive_call()
    set_posture("aggressive")
end

function call_for_mobbing_assistance()
    -- Call other birds to mob predator
    play_alarm_call()
    set_social_signal("mobbing_call")
end

function detect_suitable_prey()
    local prey_birds = get_nearby_smaller_birds()
    return #prey_birds > 0
end

function is_good_soaring_conditions() 
    local wind_strength = get_wind_strength()
    local thermals = get_thermal_strength()
    return wind_strength > 0.3 or thermals > 0.4
end