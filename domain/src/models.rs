use crate::schema::posts;
use crate::schema::authors;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};
use rocket_validation::{Validate};
use std::convert::From;

// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(Insertable, Deserialize, Serialize)]
#[derive(Selectable, Queryable, Identifiable)]
#[derive(Ord, Eq, PartialEq, PartialOrd, AsChangeset, Validate)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = authors)]
#[diesel(primary_key(id))]
pub struct Author {
    #[serde(skip_deserializing)]
    pub id: i32,

    #[validate(length(min = 2, max = 120))]
    pub firstname: String,

    #[validate(length(min = 2, max = 120))]
    pub lastname: String,

    #[validate(email)]
    pub email: String,
    pub is_active: bool,
}

#[derive(Insertable, Deserialize, Serialize)]
#[derive(Queryable)]
#[derive(Validate)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = authors)]
pub struct CreateAuthor {
    #[validate(length(min = 2, max = 120))]
    pub firstname: String,

    #[validate(length(min = 2, max = 120))]
    pub lastname: String,

    #[validate(email)]
    pub email: String,
    pub is_active: bool,
}

#[derive(Selectable, Deserialize, Serialize)]
#[derive(Queryable, Identifiable, Associations)]
#[derive(Ord, Eq, PartialEq, PartialOrd, AsChangeset, Validate)]
#[serde(crate = "rocket::serde")]
#[diesel(belongs_to(Author))]
#[diesel(table_name = posts)]
#[diesel(primary_key(id))]
pub struct Post {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub title: String,
    pub body: String,
    pub genre: String,
    pub published: bool,

    #[serde(skip_deserializing)]
    pub created_at: chrono::NaiveDateTime,

    #[serde(skip_deserializing)]
    pub updated_at: chrono::NaiveDateTime,

    #[serde(skip_serializing)]
    pub author_id: Option<i32>,
}


#[derive(Insertable, Deserialize, Serialize)]
#[derive(Queryable)]
#[derive(Validate)]
#[diesel(table_name = posts)]
pub struct CreatePost {
    #[validate(length(min = 3, max = 255))]
    pub title: String,
    pub body: String,

    #[validate(length(min = 3, max = 255))]
    pub genre: String,
    pub published: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub author_id: Option<i32>,
}


#[derive(Queryable, Deserialize, Serialize)]
#[derive(Ord, Eq, PartialEq, PartialOrd)]
pub struct PostWithRelations {
    #[serde(flatten)]
    pub post: Post,
    pub author: Option<Author>,
}