use diesel::PgConnection;

use crate::models::post::{NewPost, Post};
use diesel::prelude::*;

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn publish_post<'a>(conn: &PgConnection, post_id: i32) -> Post {
    use crate::schema::posts::dsl::*;

    let post = diesel::update(posts.find(post_id))
        .set(published.eq(true))
        .get_result::<Post>(conn)
        .expect(&format!("Unable to find post {:?}", id));

    return post;
}
