use crate::schema::posts;
use diesel::prelude::*;

#[derive(Queryable, Identifiable)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub name: &'a str,
    pub age: i32,
}
