#![allow(proc_macro_derive_resolution_fallback)] // https://github.com/diesel-rs/diesel/issues/1785

#[macro_use]
extern crate diesel;

mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use schema::{musics, tags};

pub fn music_with_tag(c: &PgConnection, tag_id: i32) -> Vec<i32> {
    tags::table
        .select(tags::id.eq(tag_id))
        .inner_join(musics::table)
        .select(musics::id)
        .load(c)
        .unwrap()
}

pub fn music_without_tag(c: &PgConnection, tag_id: i32) -> Vec<i32> {
    let filtered_tags = tags::table
        .select(tags::music_id)
        .filter(tags::id.eq(tag_id));
    musics::table
        .select(musics::id)
        .left_outer_join(filtered_tags.on(musics::id.eq(tags::music_id)))
        .filter(tags::music_id.is_null())
        .load(c)
        .unwrap()
}
