extern crate weblogue;
extern crate diesel;

#[macro_use] extern crate fake;

use diesel::prelude::*;
use weblogue::*;
use weblogue::models::*;

fn main() {
    use schema::posts::dsl::*;
    let connection = create_db_pool().get().unwrap();
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
