use std::io;

use person::Person;

mod person;
fn main() {
    println!("Introduce yourself!");

    let mut first_name = String::new();
    let mut last_name = String::new();
    let mut age = String::new();

    println!("What's your first name?");
    io::stdin()
        .read_line(&mut first_name)
        .expect("Failed to read first name");

    println!("What's your last name?");
    io::stdin()
        .read_line(&mut last_name)
        .expect("Failed to read last name");

    println!("How old are you?");
    io::stdin().read_line(&mut age).expect("Failed to read age");

    let user: Person = Person {
        first_name: first_name.trim().to_string(),
        last_name: last_name.trim().to_string(),
        age: age
            .trim()
            .parse()
            .expect("Could not parse string age to integer"),
    };

    println!("{}", user.introduce());
    // let person1: Person = Person {
    //     first_name: String::from("John"),
    //     last_name: String::from("Doe"),
    //     age: 30,
    // };
    // let person2: Person = Person {
    //     first_name: String::from("Jane"),
    //     last_name: String::from("Doe"),
    //     age: 20,
    // };
    // let list_of_people: Vec<Person> = vec![person1, person2];

    // for person in &list_of_people {
    //     println!("{}", person.full_name());
    // }
    // for person in &list_of_people {
    //     println!("{}", person.details());
    // }
    // for person in &list_of_people {
    //     println!("{}", person.introduce());
    // }
}
