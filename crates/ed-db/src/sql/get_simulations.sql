SELECT
  id AS simulation_id,
  user_id,
  prompt,
  audience,
  nudge,
  generated_text,
  confidence_overall,
  confidence_dimensions,
  retrieved_examples,
  fingerprint_snapshot,
  created_at
FROM
  ed_api.simulations
WHERE
  user_id = $1
  AND ($2 = '' OR audience = $2)
ORDER BY created_at DESC
LIMIT $3
OFFSET $4;
