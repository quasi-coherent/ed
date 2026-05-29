INSERT INTO ed_api.simulations (user_id, prompt, audience, nudge, generated_text,
  confidence_overall, confidence_dimensions, retrieved_examples,
  fingerprint_snapshot)
  VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
  RETURNING id;
