use bevy_ecs::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
enum Stages {
    Startup,
    Main,
}

// Greet entities with a Name component.
fn hello_blank(names: Query<&Name>) {
    for name in names.iter() {
        println!("Hello, {}", name.0);
    }
}

#[derive(Debug, Clone)]
struct Name(pub String);

impl Name {
    pub fn new(s: &str) -> Self {
        Name(s.to_owned())
    }
}

impl Default for Name {
    fn default() -> Self {
        Name("NONAME".to_owned())
    }
}

// Add some entities, populated with Name components, to the World.
fn add_people(mut cmds: Commands) {
    for name in ["Olive", "Lydia", "Bob", "Magnus"].iter() {
        cmds.spawn().insert(Name::new(name));
    }
}

fn main() {
    let mut schedule = Schedule::default();

    schedule.add_stage(Stages::Startup, SystemStage::parallel());
    schedule.add_stage_after(Stages::Startup, Stages::Main, SystemStage::parallel());

    schedule.add_system_to_stage(Stages::Startup, add_people.system());
    schedule.add_system_to_stage(Stages::Main, hello_blank.system());

    let mut world = World::default();
    schedule.run(&mut world);
}
