#![feature(proc_macro_hygiene, decl_macro)]

extern crate weblogue;
extern crate rocket;
extern crate rocket_contrib;
extern crate diesel;
extern crate tera;


use diesel::prelude::*;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
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

    println!("Loaded {} posts.", post_list.len());

    context.insert("post", &post_list.first());
    Template::render("index", &context)
}

#[get("/posts")]
fn posts(connection: DbConn) -> Template {
    use schema::posts::dsl::*;

    let mut context = Context::new();

    let post_list = posts.load::<Post>(&*connection).expect("Error loading posts");

    context.insert("posts", &post_list);
    Template::render("posts", &context)
}

#[get("/about")]
fn about() -> Template {
    let mut context = Context::new();
    Template::render("about", &context)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(create_db_pool_and_seed())
        .mount("/", routes![index, posts, about])
        .mount("/", StaticFiles::from("static"))
        .attach(Template::fairing())
}

fn main() {
    rocket()
        .launch();
}
