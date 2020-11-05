use bevy_ecs::{IntoQuerySystem, ParallelExecutor, Resources, Schedule, World};

// Stage label.
const MAIN: &'static str = "MAIN";

// 'Hello world' system.
fn hello_world() {
    println!("Hello world");
}

fn main() {
    // 1. Create Schedule
    let mut schedule = Schedule::default();
    // 1a. Register stages.
    schedule.add_stage(MAIN);
    // 1b. Add systems to registered stages. Failure to register stages used in calls
    //  to `add_system_to_stage` will result in a panic.
    //  Here, `.system` is provided by `IntoQuerySystem`
    schedule.add_system_to_stage(MAIN, hello_world.system());

    // 2. Create ParallelExecutor. The `without_tracker_clears` constructor means
    // World and Resources will not clear resource/entity tracker state on each
    // executor pass. This appears to refer to 'mutated' and 'added' flags?
    let mut executor = ParallelExecutor::without_tracker_clears();
    // 2a. Create World.
    let mut world = World::default();
    // 2b. Create Resources.
    let mut resources = Resources::default();

    // 3. Initialize the executor.
    executor.initialize(&mut resources);
    // 4. Initialize the scheduler.
    schedule.initialize(&mut world, &mut resources);
    // 5. Run the executor.
    executor.run(&mut schedule, &mut world, &mut resources);
}
