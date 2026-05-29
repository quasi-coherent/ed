SELECT
  id AS message_id,
  body,
  audience,
  created_at
FROM
  ed_api.corpora
WHERE
  user_id = $1
  AND ($2 = '' OR audience = $2)
ORDER BY created_at DESC
LIMIT $3
OFFSET $4;
