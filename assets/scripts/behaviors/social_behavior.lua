-- Social Behavior Script
-- Handles flocking, territorial, and courtship behaviors

function evaluate_behavior()
    local social_need = get_bird_social_need()
    local territorial_stress = get_bird_territorial_stress()
    local energy = get_bird_energy()
    local fear = get_bird_fear()
    
    log_info("Social evaluation - Social need: " .. social_need .. ", Territorial stress: " .. territorial_stress)
    
    -- High territorial stress - challenge rivals
    if territorial_stress > 0.7 and check_action_available("Challenge") then
        log_info("High territorial stress - challenging rival")
        set_bird_state("MovingToTarget")
        return true
    end
    
    -- Courtship behavior during breeding season
    if get_breeding_season() and social_need > 0.6 and energy > 0.6 then
        if check_action_available("Court") then
            log_info("Breeding season courtship behavior")
            set_bird_state("MovingToTarget") 
            return true
        end
    end
    
    -- Flocking behavior for social species
    local species_sociability = get_species_sociability()
    if species_sociability > 0.5 and social_need > 0.4 then
        if check_action_available("Flock") then
            log_info("Social species flocking")
            set_bird_state("MovingToTarget")
            return true
        end
    end
    
    -- Following behavior - low risk social interaction  
    if social_need > 0.3 and fear < 0.4 and energy > 0.4 then
        if check_action_available("Follow") then
            log_info("Following for social interaction")
            set_bird_state("MovingToTarget")
            return true
        end
    end
    
    -- Mixed species flocking during winter/migration
    if get_season() == "Winter" or get_migration_period() then
        if social_need > 0.2 and check_action_available("Flock") then
            log_info("Winter/migration flocking")
            set_bird_state("MovingToTarget")
            return true
        end
    end
    
    return false
end

-- Calculate optimal flock size based on species and conditions
function get_optimal_flock_size()
    local species_max = get_species_max_flock()
    local current_season = get_season()
    local food_scarcity = get_food_scarcity_level()
    
    local optimal_size = species_max
    
    -- Larger flocks in winter for warmth and protection
    if current_season == "Winter" then
        optimal_size = optimal_size * 1.3
    end
    
    -- Smaller flocks when food is scarce to reduce competition
    if food_scarcity > 0.6 then
        optimal_size = optimal_size * 0.7
    end
    
    -- Larger flocks during migration for navigation
    if get_migration_period() then
        optimal_size = optimal_size * 1.5
    end
    
    return math.max(1, math.floor(optimal_size))
end

-- Determine if this bird should be a flock leader
function should_be_flock_leader()
    local dominance = get_bird_dominance() 
    local experience = get_bird_experience()
    local energy = get_bird_energy()
    local nearby_birds = get_nearby_bird_count()
    
    -- Need sufficient dominance and energy to lead
    if dominance < 0.6 or energy < 0.7 then
        return false
    end
    
    -- More experienced birds are better leaders
    local leadership_score = dominance * 0.4 + experience * 0.4 + (energy * 0.2)
    
    -- Random factor to prevent all high-dominance birds from trying to lead
    local random_factor = random_float() * 0.3
    leadership_score = leadership_score + random_factor
    
    return leadership_score > 0.8 and nearby_birds >= 2
end

-- Social distance preferences based on relationship and dominance
function get_preferred_social_distance(other_bird_id)
    local relationship = get_relationship_with(other_bird_id)
    local other_dominance = get_other_bird_dominance(other_bird_id)
    local my_dominance = get_bird_dominance()
    
    local base_distance = 30.0
    
    if relationship == "mate" then
        return base_distance * 0.3  -- Very close to mate
    elseif relationship == "rival" then  
        return base_distance * 2.0  -- Keep distance from rivals
    elseif other_dominance > my_dominance then
        return base_distance * 1.5  -- Give space to dominant birds
    else
        return base_distance
    end
end