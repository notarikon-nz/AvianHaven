use bevy::prelude::*;
use bevy_scriptum::{prelude::*, runtimes::lua::prelude::*};

pub mod lua_api;

use lua_api::*;

pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ScriptingPlugin::default())
            .add_scripting::<LuaRuntime>(|runtime| {
                // Register bird AI functions
                runtime.add_function(String::from("log_info"), log_info);
                runtime.add_function(String::from("get_bird_hunger"), get_bird_hunger);
                runtime.add_function(String::from("set_bird_state"), set_bird_state);
                runtime.add_function(String::from("check_action_available"), check_action_available);
                runtime.add_function(String::from("get_weather_fear"), get_weather_fear);
                runtime.add_function(String::from("get_time_of_day"), get_time_of_day);
                
                // Register utility functions
                runtime.add_function(String::from("random_float"), random_float);
                runtime.add_function(String::from("distance_to_target"), distance_to_target);
            })
            .add_systems(Update, execute_behavior_scripts);
    }
}

// System to execute Lua behavior scripts
fn execute_behavior_scripts(
    mut script_query: Query<&mut Script<LuaRuntime>>,
) {
    for mut script in script_query.iter_mut() {
        // Execute the script's behavior evaluation function if it exists
        if script.has_function("evaluate_behavior") {
            let _ = script.call_if_exists("evaluate_behavior", ());
        }
    }
}

#[derive(Component)]
pub struct LuaBehaviorScript {
    pub script_path: String,
    pub priority: u32,
}

// Resource to hold script configuration
#[derive(Resource, Default)]
pub struct ScriptingConfig {
    pub enabled: bool,
    pub script_directory: String,
    pub hot_reload: bool,
}