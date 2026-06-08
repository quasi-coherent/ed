-- V3 creates all the tables needed by the backend.

-- Messages stored as bytea to support encryption. `audience` is an enum but if
-- we make it an enum in the db we're going to have a bad time.
CREATE TABLE IF NOT EXISTS ed_api.corpora (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id uuid NOT NULL REFERENCES ed_api.users(id) ON DELETE CASCADE,
  body bytea NOT NULL,
  audience text NOT NULL DEFAULT 'unknown',
  created_at timestamptz(3) NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS corpora_uid_aud_idx
  ON ed_api.corpora (user_id, audience, created_at DESC);

ALTER TABLE ed_api.corpora ENABLE ROW LEVEL SECURITY;

CREATE POLICY corpora_uid_pol ON ed_api.corpora
  USING (user_id = current_setting('ed_api.user_id')::uuid);

CREATE TABLE IF NOT EXISTS ed_api.fingerprints (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id uuid NOT NULL REFERENCES ed_api.users(id) ON DELETE CASCADE,
  formality_score real NOT NULL,
  avg_sentence_length real NOT NULL,
  sentence_length_variance real NOT NULL,
  exclamation_ratio real NOT NULL,
  ellipsis_ratio real NOT NULL,
  emoji_frequency real NOT NULL,
  contraction_ratio real NOT NULL,
  hedging_ratio real NOT NULL,
  common_openers text[] NOT NULL,
  common_closers text[] NOT NULL,
  message_count integer NOT NULL,
  created_at timestamptz(3) NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS fingerprints_uid_idx
  ON ed_api.fingerprints (user_id, created_at DESC);

CREATE TABLE IF NOT EXISTS ed_api.embeddings (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  message_id uuid NOT NULL REFERENCES ed_api.corpora(id) ON DELETE CASCADE,
  user_id uuid NOT NULL REFERENCES ed_api.users(id) ON DELETE CASCADE,
  vector vector(1536) NOT NULL,
  created_at timestamptz(3) NOT NULL DEFAULT now(),
  UNIQUE (message_id)
);

-- HNSW index with `vector_cosine_ops` is leveraged when we go to calculate
-- cosine distance.
CREATE INDEX IF NOT EXISTS embeddings_v_hnsw_idx
  ON ed_api.embeddings
  USING hnsw (vector vector_cosine_ops);

CREATE INDEX IF NOT EXISTS embeddings_uid_idx
  ON ed_api.embeddings (user_id, created_at DESC);

-- Stores all simulation results.
CREATE TABLE IF NOT EXISTS ed_api.simulations (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id uuid NOT NULL REFERENCES ed_api.users(id) ON DELETE CASCADE,
  prompt text NOT NULL,
  audience text NOT NULL DEFAULT 'unknown',
  nudge text,
  generated_text text NOT NULL,
  confidence_overall real NOT NULL,
  confidence_dimensions jsonb NOT NULL,
  retrieved_examples jsonb NOT NULL,
  fingerprint_snapshot jsonb NOT NULL,
  created_at timestamptz(3) NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS simulations_uid_idx
  ON ed_api.simulations (user_id, created_at DESC);
