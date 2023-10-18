//use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_demo::*;

use postgis_diesel::types::*;

use self::models::Entity;
use self::schema::entities;
// use self::models::Property;

use uuid::Uuid;

fn new_point(x: f64, y: f64) -> Point {
    Point::new(x, y, Some(4326))
}

fn main() {
    let connection = &mut establish_connection();

    let source   = "Source";
    let source_id = "Source_ID";

    let mut polygon = Polygon::new(Some(4326));
    polygon.add_points([
        new_point(72.0, 64.0),
        new_point(73.0, 65.0),
        new_point(71.0, 62.0),
        new_point(72.0, 64.0),
    ]);

    let new_entity = Entity { uid: Uuid::new_v4(),
			      source: source.to_string(),
			      source_id: source_id.to_string(),
			      shape: GeometryContainer::Polygon( polygon ),
			      ..Default::default()
    };

    let e1 = diesel::insert_into(entities::table)
                    .values(&new_entity)
                    .returning(Entity::as_returning())
                    .get_result(connection)
                    .expect("Error saving new entity");
    println!("\nCreated Entity {:?}", e1 );
}
