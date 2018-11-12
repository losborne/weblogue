#![feature(proc_macro_hygiene, decl_macro)]

extern crate weblogue;
extern crate rocket;
extern crate rocket_contrib;
extern crate diesel;
extern crate tera;


use diesel::prelude::*;
use rocket_contrib::templates::Template;
use weblogue::*;
use weblogue::models::*;
use rocket::{get, routes};
use tera::Context;

#[cfg(test)] mod tests;

#[get("/")]
fn index(connection: DbConn) -> Template {
    use schema::posts::dsl::*;

    let mut context = Context::new();

    let post_list = posts.load::<Post>(&*connection).expect("Error loading posts");

    context.insert("posts", &post_list);
    Template::render("index", &context)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(create_db_pool())
        .mount("/", routes![index])
        .attach(Template::fairing())
}

fn main() {
    rocket()
        .launch();
}
