use crate::schema::posts;
use crate::schema::authors;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};
use rocket_validation::{Validate};
use std::convert::From;

// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(Selectable, Queryable, Deserialize, Serialize, Ord, Eq, PartialEq, PartialOrd, AsChangeset)]
#[diesel(belongs_to(Author))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub genre: String,
    pub published: bool,
    #[serde(skip)]
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub author_id: Option<i32>,
}

#[derive(Insertable, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
#[diesel(belongs_to(Author))]
#[diesel(table_name = posts)]
pub struct NewPost {
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

#[derive(Selectable, Queryable, Deserialize, Serialize, Ord, Eq, PartialEq, PartialOrd, AsChangeset, Validate)]
pub struct Author {
    pub id: i32,
    #[validate(length(min = 2, max = 120))]
    pub firstname: String,
    #[validate(length(min = 2, max = 120))]
    pub lastname: String,
    #[validate(email)]
    pub email: String,
    pub is_active: bool,
}

#[derive(Insertable, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = authors)]
pub struct NewAuthor {
    #[validate(length(min = 2, max = 255))]
    pub firstname: String,
    #[validate(length(min = 2, max = 120))]
    pub lastname: String,
    #[validate(email)]
    pub email: String,
    pub is_active: bool,
}

#[derive(Queryable, Deserialize, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct PostWithAuthor {
    #[serde(flatten)]
    pub post: Post,
    pub author: Option<Author>,
}