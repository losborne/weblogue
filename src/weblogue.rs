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

#[get("/<file..>")]
fn static_content(file: PathBuf) -> std::io::Result<NamedFile> {
    NamedFile::open(Path::new("static/").join(file))
}

#[get("/")]
fn index() -> Template {
    let mut context = Context::new();

    context.insert("page_content", "Weblogue: notes from a lifelong quest of learning.");
    Template::render("index", &context)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, static_content])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
