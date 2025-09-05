-- Basic Needs Behavior Script
-- Handles fundamental survival behaviors: hunger, thirst, energy

function evaluate_behavior()
    local hunger = get_bird_hunger()
    local energy = get_bird_energy() 
    local thirst = get_bird_thirst()
    local time = get_time_of_day()
    
    log_info("Evaluating basic needs - Hunger: " .. hunger .. ", Energy: " .. energy)
    
    -- Critical hunger override
    if hunger > 0.9 then
        if check_action_available("Eat") then
            log_info("Critical hunger detected, seeking food")
            set_bird_state("MovingToTarget") 
            return true
        end
    end
    
    -- Critical thirst override  
    if thirst > 0.9 then
        if check_action_available("Drink") then
            log_info("Critical thirst detected, seeking water")
            set_bird_state("MovingToTarget")
            return true
        end
    end
    
    -- Energy management
    if energy < 0.2 then
        log_info("Low energy, resting")
        set_bird_state("Resting")
        return true
    end
    
    -- Evening roosting
    if time >= 18.0 and time <= 20.0 then
        if check_action_available("Roost") then
            log_info("Evening roosting time")
            set_bird_state("MovingToTarget")
            return true
        end
    end
    
    -- Moderate hunger/thirst
    if hunger > 0.6 and check_action_available("Eat") then
        log_info("Moderate hunger, seeking food")
        set_bird_state("MovingToTarget") 
        return true
    end
    
    if thirst > 0.5 and check_action_available("Drink") then
        log_info("Moderate thirst, seeking water")
        set_bird_state("MovingToTarget")
        return true
    end
    
    return false
end

-- Helper function to check if it's prime feeding time
function is_prime_feeding_time()
    local time = get_time_of_day()
    return (time >= 6.0 and time <= 10.0) or (time >= 16.0 and time <= 19.0)
end

-- Calculate feeding urgency based on time and hunger
function get_feeding_urgency()
    local hunger = get_bird_hunger()
    local base_urgency = hunger
    
    if is_prime_feeding_time() then
        base_urgency = base_urgency * 1.2
    end
    
    return math.min(base_urgency, 1.0)
end