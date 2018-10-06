I'd like to run the following query with Diesel.
```sql
SELECT musics.id
FROM (
  musics LEFT OUTER JOIN
  (SELECT tags.music_id FROM tags WHERE tags.id = 2) AS selected_tags
  ON (musics.id = selected_tags.music_id))
WHERE (selected_tags.music_id IS NULL)
```

The following does not work:
```Rust
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
```

Because the `filtered_tags` `diesel::query_builder::SelectStatement` does not implement the `diesel::JoinOnDsl`.

Is there a way to express the desired query?

If not I'll try to use [`ne_all`](https://docs.diesel.rs/diesel/expression_methods/trait.ExpressionMethods.html#method.ne_all) with a select query.
