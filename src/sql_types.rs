use postgis_diesel::types::*;

#[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
#[diesel(postgres_type(name = "propertytypeenum"))]
pub struct Propertytypeenum;

#[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
#[diesel(postgres_type(name = "shapetypeenum"))]
pub struct Shapetypeenum;

