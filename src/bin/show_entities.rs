use self::models::*;
use diesel::prelude::*;
use diesel_demo::*;

fn main() {
    use self::schema::entities::dsl::*;

    let connection = &mut establish_connection();
    let results = entities
        .limit(5)
        .select(Entity::as_select())
        .load(connection)
        .expect("Error loading entities");

    println!("Displaying {} entities", results.len());
    for post in results {
        println!("-----------\n");
        println!("{} - {:?}/{:?} - {:?}", post.uid, post.source, post.source_id, post.shape )
    }
}
