table! {
    musics (id) {
        id -> Int4,
    }
}

table! {
    tags (id) {
        id -> Int4,
        music_id -> Int4,
    }
}

joinable!(tags -> musics (music_id));

allow_tables_to_appear_in_same_query!(
    musics,
    tags,
);
