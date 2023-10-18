use diesel::prelude::*;
use diesel_uuid::models;

fn main() {
    use diesel_uuid::schema::uuid_users::dsl::*;

    let connection = &mut diesel_uuid::establish_connection();
    let results = uuid_users
        .limit(5)
        .select(models::UuidUser::as_select())
        .load(connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{} - {}", user.user_uid, user.name );
    }
}
