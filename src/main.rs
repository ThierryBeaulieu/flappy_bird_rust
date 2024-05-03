use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, print_names)
        .run()
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Alex".to_string(),
        },
        Employed { job: Job::Doctor },
    ));
    commands.spawn((
        Person {
            name: "Bob".to_string(),
        },
        Employed { job: Job::Lawyer },
    ));
    commands.spawn((
        Person {
            name: "Sylvain".to_string(),
        },
        Employed {
            job: Job::FireFighter,
        },
    ));
    commands.spawn((Person {
        name: "Mathew".to_string(),
    },));
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name : {}", person.name);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Doctor,
    FireFighter,
    Lawyer,
}
