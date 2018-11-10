#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[get("/")]
fn index() -> &'static str {
    "A blog"
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
}

fn main() {
    rocket().launch();
}
