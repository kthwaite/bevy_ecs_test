use bevy_ecs::{IntoForEachSystem, ParallelExecutor, Resources, Schedule, World};

const MAIN: &'static str = "MAIN";

fn hello_blank(name: &String) {
    println!("hello {}", name);
}

fn main() {
    let mut schedule = Schedule::default();
    schedule.add_stage(MAIN);
    //  Here, `.system` is provided by `IntoForEachSystem`
    //  The function signature of `hello_blank` determines the 'for each' in question
    schedule.add_system_to_stage(MAIN, hello_blank.system());

    let mut executor = ParallelExecutor::without_tracker_clears();
    let mut world = World::default();
    let mut resources = Resources::default();

    // Add some entities, populated with &str components, to the World.
    for name in ["Olive", "Lydia", "Bob", "Magnus"].iter() {
        world.spawn((name.to_string(),));
    }

    executor.initialize(&mut resources);
    schedule.initialize(&mut world, &mut resources);
    executor.run(&mut schedule, &mut world, &mut resources);
}
