use bevy::core::FixedTimestep;
use bevy::ecs::schedule::{IntoRunCriteria, IntoSystemDescriptor, RunOnce, ShouldRun};
use bevy_sample::init::common::build;
use bevy_sample::main::systems::{snake_movement, head_translation};
use bevy_sample::main::{components, systems, resources};
use bevy::prelude::*;



struct Empty {
    value: String
}

impl Default for Empty {
    fn default() -> Self {
        Empty { value: "Test".to_string() }
    }
}

impl Empty {
    fn test(self: &Self) {
        println!("{}", self.value);
    }
}



fn main() {

    let movement_time: f64 = 0.1;

    let food_timer: f64 = 2.0;

    App::new()
    .insert_resource(WindowDescriptor {
        title: "Snake".to_string(),
        width: 600.0,
        height: 600.0,
        ..WindowDescriptor::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource(resources::SnakeProperties::default())
    .insert_resource(resources::StaleSnakeProperties::default())
    .add_startup_system(systems::spawn_snake)
    .add_startup_system(systems::spawn_tail.after(systems::spawn_snake))
    .add_startup_system(systems::spawn_tail.after(systems::spawn_snake))
    .add_startup_system(systems::spawn_tail.after(systems::spawn_snake))
    .add_startup_system(systems::food_spawner)
    .add_startup_system(systems::setup_camera)
    .add_system(systems::snake_direction.before(snake_movement))
    .add_system_set(
        SystemSet::new()
        .with_run_criteria(FixedTimestep::step(movement_time))
        .with_system(snake_movement)
    )
    .add_system_set(
        SystemSet::new()
        .with_run_criteria(FixedTimestep::step(food_timer))
        .with_system(systems::food_spawner)
    )

    .add_system_set_to_stage(
        CoreStage::PostUpdate, 
        SystemSet::new()
        .with_system(systems::size_scaling)
        .with_system(systems::head_translation)
        .with_system(systems::body_translation.before(head_translation))
    )
    .add_plugins(DefaultPlugins)
    .run();
}