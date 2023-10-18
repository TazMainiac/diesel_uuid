
use uuid::Uuid;
use chrono::NaiveDateTime;

use postgis_diesel::types::*;
use diesel::prelude::*;
    
#[derive(diesel_derive_enum::DbEnum, Debug)]
#[ExistingTypePath = "crate::schema::sql_types::Shapetypeenum"]
pub enum Shapetypeenum {
    CIRCLE,
    ELLIPSE,
    GEOJSON,
}

#[derive(diesel_derive_enum::DbEnum, Debug)]
#[ExistingTypePath = "crate::schema::sql_types::Propertytypeenum"]
pub enum Propertytypeenum {
    STRING,
    FLOAT,
    BOOL,
}

#[derive(Insertable, Selectable, Queryable, Debug)]
#[diesel(table_name = crate::schema::entities)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Entity {
    pub uid: Uuid,
    pub source: String,
    pub source_id: String,
    
    pub ts_created: Option<NaiveDateTime>,
    pub ts_updated: Option<NaiveDateTime>,
    pub ts_begin: Option<NaiveDateTime>,
    pub ts_end: Option<NaiveDateTime>,

    pub stale_secs: Option<i32>,
    pub delete_secs: Option<i32>,
    pub stale: Option<bool>,

    pub latitude_degrees: f64,
    pub longitude_degrees: f64,
    pub altitude_msl_meters: f64,
    pub height_agl_meters: f64,
    pub height_agl_min_meters: f64,
    pub heading_degrees: f64,
    pub speed_knots: f64,
    pub pitch_degrees: f64,
    pub roll_degrees: f64,

    pub shape_type: Option<Shapetypeenum>,

    pub shape_latitude_degrees: Option<f64>,
    pub shape_longitude_degrees: Option<f64>,
    pub shape_radius_meters: Option<f64>,
    pub shape_minor_radius_meters: Option<f64>,
    pub shape_orientation_degrees: Option<f64>,
    
    pub shape_geojson: Option<String>,
    
    pub shape: GeometryContainer<Point>,
    
    pub entity_type: Option<String>,
    pub owner_key: Option<String>,
    pub field_key: Option<String>,
    pub locked_property_key: Option<String>,
    pub unlocked_property_key: Option<String>,
}


fn new_point(x: f64, y: f64) -> Point {
    Point::new(x, y, Some(4326))
}

impl Default for Entity {
    fn default() -> Entity {
	Entity {
	    
	    uid: Uuid::new_v4(),
	    source: "".to_string(),
	    source_id: "".to_string(),
	    
	    ts_created: None,
	    ts_updated: None,
	    ts_begin: None,
	    ts_end: None,

	    stale_secs: None,
	    delete_secs: None,
	    stale: None,

	    latitude_degrees: 0.0,
	    longitude_degrees: 0.0,
	    altitude_msl_meters: 0.0,
	    height_agl_meters: 0.0,
	    height_agl_min_meters: 0.0,
	    heading_degrees: 0.0,
	    speed_knots: 0.0,
	    pitch_degrees: 0.0,
	    roll_degrees: 0.0,

	    shape_type: None,

	    shape_latitude_degrees: None,
	    shape_longitude_degrees: None,
	    shape_radius_meters: None,
	    shape_minor_radius_meters: None,
	    shape_orientation_degrees: None,
	    shape_geojson: None,

	    shape: GeometryContainer::Point( new_point(0.0, 0.0) ),
	    
	    entity_type: None,
	    owner_key: None,
	    field_key: None,
	    locked_property_key: None,
	    unlocked_property_key: None,
	}
    }
}

#[derive(Insertable, Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::properties)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Property {
    pub uid: Uuid,
    pub name: String,
    pub prop_type: Propertytypeenum,
    pub str_value: Option<String>,
    pub float_value: Option<f64>,
    pub bool_value: Option<bool>,
    pub locked: bool,
}
