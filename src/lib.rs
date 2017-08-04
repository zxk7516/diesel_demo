#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

use self::models::post::{Post, NewPost};

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &MysqlConnection, title: &str, body: &str) -> Post {
    use schema::posts::dsl::{posts, id};

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert(&new_post).into(posts)
        .execute(conn)
        .expect("Error saving new post");

    posts.order(id.desc()).first(conn).unwrap()
}