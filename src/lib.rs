#[macro_use] extern crate serde_derive;
#[macro_use] extern crate fake;

#[macro_use] extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::io;

use self::models::{Post, NewPost};

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts;

    let new_post = NewPost {
        title: title.to_string(),
        body: body.to_string(),
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

// Rocket web server
extern crate rocket;
extern crate rocket_contrib;
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::ops::Deref;

pub fn create_db_pool_and_seed() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::new(manager).expect("Failed to create pool");
    let connection = DbConn(pool.get().unwrap());
    seed(connection);
    return pool;
}

pub struct DbConn(PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool<ConnectionManager<PgConnection>>>>()?;

        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

// Implement Deref for the wrapper to PcConnection so we don't need to do .0
impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn seed(connection: DbConn) {
    println!("Seeding database with lorem ipsum posts.");

    use schema::posts::dsl::*;
    diesel::delete(posts).execute(&*connection).expect("Error deleting posts");

    fn generate_post_info() -> NewPost {
        NewPost {
            title: fake!(Lorem.sentence(1,4)),
            body: fake!(Lorem.paragraph(5,5)),
        }
    }

    let new_post_list: Vec<NewPost> = (0..10)
        .map( |_| generate_post_info())
        .collect();

    diesel::insert_into(posts)
        .values(&new_post_list)
        .execute(&*connection)
        .expect("Error inserting posts");
}
