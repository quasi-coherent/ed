SELECT
  c.id AS message_id,
  c.body,
  c.audience,
  (1 - (e.vector <=> $1::vector))::real AS "similarity!"
FROM
  ed_api.corpora c
  JOIN ed_api.embeddings e
  ON c.id = e.message_id
WHERE
  c.user_id = $2
ORDER BY
  e.vector <=> $1::vector
LIMIT $3;
