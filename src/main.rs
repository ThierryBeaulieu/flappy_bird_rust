use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, print_names)
        .run()
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Person {
        name: "Alex".to_string(),
    });
    commands.spawn(Person {
        name: "Bob".to_string(),
    });
    commands.spawn(Person {
        name: "Sylvain".to_string(),
    });
    commands.spawn(Person {
        name: "Mathiew".to_string(),
    });
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
