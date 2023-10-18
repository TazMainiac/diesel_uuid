use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::uuid_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UuidUser {
    pub user_uid: uuid::Uuid,
    pub name: String,
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::uuid_users)]
pub struct NewUser<'a> {
    pub name: &'a str,
}
