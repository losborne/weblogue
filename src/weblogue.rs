#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate tera;

use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use tera::Context;

#[cfg(test)] mod tests;

#[get("/")]
fn index() -> Template {
    let mut context = Context::new();

    context.insert("page_content", "Weblogue: notes from a lifelong quest of learning.");
    context.insert("posts", "a");
    Template::render("index", &context)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![index])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
