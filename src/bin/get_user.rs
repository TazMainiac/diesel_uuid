use diesel::prelude::*;
use diesel_uuid::*;
use std::env::args;

fn main() {
    use diesel_uuid::schema::uuid_users::dsl::*;
    use uuid::Uuid;

    let id = args()
        .nth(1)
        .expect("get_user requires a name")
        .parse::<String>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();

    let real_uid = Uuid::parse_str( &id );

    let results = uuid_users
        .filter( user_uid.eq( real_uid.unwrap() ) )
        .limit(5)
        .select(models::UuidUser::as_select())
        .load(connection)
        .expect("Error loading users");

    for user in results {
	println!("Got User {} - {}", user.user_uid, user.name );
    }
}
