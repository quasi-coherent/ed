INSERT INTO ed_api.fingerprints (user_id, formality_score, avg_sentence_length,
  sentence_length_variance, exclamation_ratio, ellipsis_ratio, emoji_frequency,
  contraction_ratio, hedging_ratio, common_openers, common_closers,
  message_count)
  VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
  RETURNING id;
