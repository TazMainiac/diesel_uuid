// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "propertytypeenum"))]
    pub struct Propertytypeenum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "shapetypeenum"))]
    pub struct Shapetypeenum;
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;
    use super::sql_types::Shapetypeenum;

    entities (uid) {
        uid -> Uuid,
        source -> Text,
        source_id -> Text,
        reference_uid -> Nullable<Uuid>,
        ts_created -> Nullable<Timestamp>,
        ts_updated -> Nullable<Timestamp>,
        ts_begin -> Nullable<Timestamp>,
        ts_end -> Nullable<Timestamp>,
        stale_secs -> Nullable<Int4>,
        delete_secs -> Nullable<Int4>,
        stale -> Nullable<Bool>,
        latitude_degrees -> Float8,
        longitude_degrees -> Float8,
        altitude_msl_meters -> Float8,
        height_agl_meters -> Float8,
        height_agl_min_meters -> Float8,
        heading_degrees -> Float8,
        speed_knots -> Float8,
        pitch_degrees -> Float8,
        roll_degrees -> Float8,
        shape_type -> Nullable<Shapetypeenum>,
        shape_latitude_degrees -> Nullable<Float8>,
        shape_longitude_degrees -> Nullable<Float8>,
        shape_radius_meters -> Nullable<Float8>,
        shape_minor_radius_meters -> Nullable<Float8>,
        shape_orientation_degrees -> Nullable<Float8>,
        shape_geojson -> Nullable<Text>,
        shape -> Geography,
        entity_type -> Nullable<Text>,
        owner_key -> Nullable<Text>,
        field_key -> Nullable<Text>,
        locked_property_key -> Nullable<Text>,
        unlocked_property_key -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;
    use super::sql_types::Propertytypeenum;

    properties (uid, name) {
        uid -> Uuid,
        name -> Text,
        prop_type -> Propertytypeenum,
        str_value -> Nullable<Text>,
        float_value -> Nullable<Float8>,
        bool_value -> Nullable<Bool>,
        locked -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    spatial_ref_sys (srid) {
        srid -> Int4,
        #[max_length = 256]
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        #[max_length = 2048]
        srtext -> Nullable<Varchar>,
        #[max_length = 2048]
        proj4text -> Nullable<Varchar>,
    }
}

diesel::joinable!(properties -> entities (uid));

diesel::allow_tables_to_appear_in_same_query!(
    entities,
    properties,
    spatial_ref_sys,
);
