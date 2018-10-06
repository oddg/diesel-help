SELECT musics.id
FROM (
  musics LEFT OUTER JOIN
  (SELECT tags.music_id FROM tags WHERE tags.id = 2) AS selected_tags
  ON (musics.id = selected_tags.music_id))
WHERE (selected_tags.music_id IS NULL)
