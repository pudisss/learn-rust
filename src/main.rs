// Importing any packages using the use keyword

use std::io::stdin;


struct Username {
    username: String,
    email: String,
    age: u8,
}
fn main() {

    // Get user inputs
    let mut username = String::new();
    let mut email = String::new();
    let mut age = String::new();

    stdin().read_line(&mut username).ok().expect("Cannot get input sorry");
    stdin().read_line(&mut email).ok().expect("Cannot get inputs");
    stdin().read_line(&mut age).ok().expect("Cannot get inputs");

    let mut ages: u8 = age.parse().unwrap();


    // Check for the age before storing to the struct
    if (ages >= 15) {
        let mut instance = create_username_instance(username, email, ages);

        
    } else {
        println!("You are not old enough to sign in to our platform");
    }




}

fn create_username_instance(username: String, email: String, age: u8) -> Username {
    Username {
        username,
        email,
        age,
    }
}