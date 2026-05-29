SELECT
  id AS fingerprint_id,
  user_id,
  formality_score,
  avg_sentence_length,
  sentence_length_variance,
  exclamation_ratio,
  ellipsis_ratio,
  emoji_frequency,
  contraction_ratio,
  hedging_ratio,
  common_openers,
  common_closers,
  message_count,
  created_at
FROM
  ed_api.fingerprints
WHERE
  user_id = $1
ORDER BY created_at DESC
LIMIT 1;
