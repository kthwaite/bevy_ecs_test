use bevy_ecs::prelude::*;

// Stage label.
const MAIN: &'static str = "MAIN";

// 'Hello world' system.
fn hello_world() {
    println!("Hello world");
}

fn main() {
    // 1. Create Schedule
    let mut schedule = Schedule::default();
    // 2. Register stages.
    schedule.add_stage(MAIN, SystemStage::parallel());
    // 2b. Add systems to registered stages. Failure to register stages used in calls
    // to `add_system_to_stage` will result in a panic.
    // Here, `.system` is provided by `IntoQuerySystem`
    schedule.add_system_to_stage(MAIN, hello_world.system());

    // 3. Create World.
    let mut world = World::default();
    // 4. Run the executor.
    schedule.run(&mut world);
}
