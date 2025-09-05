-- Advanced Foraging Intelligence Script
-- Handles intelligent foraging decisions and cache management

function evaluate_behavior()
    local hunger = get_bird_hunger()
    local energy = get_bird_energy()
    local intelligence = get_bird_intelligence()
    
    -- Intelligent cache management for smart species
    if intelligence > 0.6 then
        local cache_decision = evaluate_caching_strategy()
        if cache_decision ~= nil then
            return cache_decision
        end
    end
    
    -- Ground foraging with search patterns
    if hunger > 0.4 and not check_action_available("Eat") then
        if start_intelligent_foraging() then
            log_info("Starting intelligent ground foraging")
            set_bird_state("Foraging")
            return true
        end
    end
    
    -- Opportunistic feeding on abundant resources
    if energy > 0.8 and hunger < 0.3 then
        if check_abundant_food_source() then
            log_info("Opportunistic feeding on abundant resource")
            set_bird_state("MovingToTarget")
            return true
        end
    end
    
    return false
end

-- Intelligent caching strategy
function evaluate_caching_strategy()
    local hunger = get_bird_hunger()
    local energy = get_bird_energy()
    local cache_count = get_current_cache_count()
    local max_cache = get_max_cache_capacity()
    local food_abundance = assess_food_abundance()
    
    -- Cache food when not hungry but food is abundant
    if hunger < 0.3 and energy > 0.6 and cache_count < max_cache then
        if food_abundance > 0.7 and check_action_available("Cache") then
            log_info("Caching food for later - abundance: " .. food_abundance)
            set_bird_state("MovingToTarget")
            return true
        end
    end
    
    -- Retrieve cached food when hungry and no immediate food sources
    if hunger > 0.6 and cache_count > 0 then
        if not check_action_available("Eat") and check_action_available("Retrieve") then
            local cache_quality = get_best_cache_quality()
            if cache_quality > 0.5 then
                log_info("Retrieving cached food - quality: " .. cache_quality)
                set_bird_state("MovingToTarget") 
                return true
            end
        end
    end
    
    return nil
end

-- Start intelligent foraging with species-specific patterns
function start_intelligent_foraging()
    local foraging_style = get_species_foraging_style()
    local search_pattern = get_species_search_pattern()
    local ground_preference = get_ground_foraging_preference()
    
    if ground_preference < 0.3 then
        return false  -- This species doesn't ground forage
    end
    
    -- Set foraging parameters based on intelligence and species
    local intelligence = get_bird_intelligence()
    
    if search_pattern == "Grid" then
        set_foraging_pattern("systematic_grid")
        set_search_efficiency(0.8 + intelligence * 0.2)
    elseif search_pattern == "Spiral" then
        set_foraging_pattern("spiral_search")
        set_search_efficiency(0.7 + intelligence * 0.25)
    else
        set_foraging_pattern("random_walk") 
        set_search_efficiency(0.5 + intelligence * 0.3)
    end
    
    return true
end

-- Assess local food abundance for strategic decisions
function assess_food_abundance()
    local nearby_feeders = count_nearby_feeders()
    local feeder_capacity = get_total_feeder_capacity()
    local competing_birds = count_competing_birds()
    local natural_food = assess_natural_food_sources()
    
    local abundance_score = 0.0
    
    -- Factor in feeder availability  
    if competing_birds > 0 then
        abundance_score = abundance_score + (feeder_capacity / competing_birds) * 0.4
    end
    
    -- Natural food sources
    abundance_score = abundance_score + natural_food * 0.3
    
    -- Seasonal modifiers
    local season = get_season()
    if season == "Spring" or season == "Summer" then
        abundance_score = abundance_score * 1.2
    elseif season == "Winter" then
        abundance_score = abundance_score * 0.7
    end
    
    return math.min(abundance_score, 1.0)
end

-- Check for abundant food sources worth exploiting
function check_abundant_food_source()
    local nearby_feeders = get_nearby_feeder_info()
    
    for i, feeder in ipairs(nearby_feeders) do
        local capacity_ratio = feeder.current_capacity / feeder.max_capacity
        local competition_level = feeder.bird_count / feeder.max_birds
        
        -- High capacity, low competition = abundant source
        if capacity_ratio > 0.8 and competition_level < 0.3 then
            log_info("Found abundant feeder - capacity: " .. capacity_ratio .. ", competition: " .. competition_level)
            return true
        end
    end
    
    return false
end

-- Intelligent cache selection - choose best cache to retrieve
function select_best_cache_for_retrieval()
    local caches = get_all_caches()
    local best_cache = nil
    local best_score = 0.0
    
    for i, cache in ipairs(caches) do
        local distance = cache.distance
        local food_amount = cache.food_amount
        local freshness = cache.freshness
        local accessibility = cache.accessibility
        
        -- Score based on food amount, freshness, and proximity
        local score = (food_amount * 0.4) + (freshness * 0.3) + (accessibility * 0.2) - (distance * 0.1)
        
        if score > best_score then
            best_score = score
            best_cache = cache
        end
    end
    
    return best_cache
end