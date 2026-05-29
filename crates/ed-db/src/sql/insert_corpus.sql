INSERT INTO ed_api.corpora (user_id, body, audience)
  VALUES ($1, $2, $3)
  RETURNING id;
