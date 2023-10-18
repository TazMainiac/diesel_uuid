use diesel_uuid::*;
use std::io::stdin;

fn main() {
    let connection = &mut establish_connection();

    let mut name = String::new();

    println!("What would you like your name to be?");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end(); // Remove the trailing newline

    let user = create_user(connection, name);
    println!("\nCreated User {} with id {}", user.name, user.user_uid );
}
