use bevy::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)          // all default plugins by bevy
        .run();
}

pub struct PeoplePluging;

impl Plugin for PeoplePluging {
    fn build(&self, app: &mut App) {                        // for bevy to build the plugin
        app
            .add_plugins(DefaultPlugins)          // all default plugins by bevy
            .add_startup_system(setup)                  // system that runs on startup (once i guess)
            .add_system(print_names)                    // run every step of the app
            .add_system(i_have_a_job)                   // run every step of the app
            .add_system(they_took_our_jobs)             // run every step of the app
            .add_system(person_is_);                     // run every step of the app
    }
}

// commands: system parameter - give to us by bevy - used to deal with entities
pub fn setup(mut commands: Commands) {
    commands.spawn(Person {
        name: "Grabrulenzo".to_string(),
    });

    commands.spawn((
        Person {
        name: "Dionilsonzinete".to_string(),
    },
        Employed {
        job: Job::Doctor,
    },
));
       
}

pub fn print_names(person_query: Query<&Person>) {  // search for all `Person`
    for person in person_query.iter() {
        println!("name: {}", person.name)
    }
}

pub fn i_have_a_job(person_query: Query<&Person, With<Employed>>) { // search for all `Person` that also have `Employed`
    for person in person_query.iter(){
        println!("{} tem um trampo", person.name)
    }
}

pub fn they_took_our_jobs(person_query: Query<&Person, Without<Employed>>) {    // search for all `Person` that explicity don't have `Emplyoed`
    for person in person_query.iter() {
        println!("{} said: 'they took our jobs'.", person.name)
    }
}

pub fn person_is_(
    person_query: Query<(&Person, &Employed)>
) {
    for (person, employed) in person_query.iter() {
        let job_name = match employed.job {
            Job::Doctor => "Doctor",
            Job::FireFighter => "FireFighter",
            Job::Lawyer => "Lawyer",
        };
        println!("{} is a {}", person.name, job_name)
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String
}


#[derive(Component)]
pub struct Employed {
    pub job: Job
}

#[derive(Debug)]
pub enum Job{
    Doctor,
    FireFighter,
    Lawyer,
}