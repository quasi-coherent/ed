INSERT INTO ed_api.embeddings (message_id, user_id, vector)
  SELECT *
  FROM unnest(
    $1::uuid[],
    $2::uuid[],
    $3::vector[]
  )
  RETURNING id;
