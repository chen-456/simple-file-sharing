use diesel::{Insertable, Queryable};

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hashed_pass: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub hashed_pass: &'a str,
}
