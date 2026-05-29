/* Domain types mirroring the OpenAPI schema in api/openapi.yaml. */

type audience = [
  | `Email
  | `Slack
  | `Sms
  | `Unknown
];

type styleFingerprint = {
  formalityScore: float,
  avgSentenceLength: float,
  sentenceLengthVariance: float,
  exclamationRatio: float,
  ellipsisRatio: float,
  emojiFrequency: float,
  contractionRatio: float,
  hedgingRatio: float,
  commonOpeners: array(string),
  commonClosers: array(string),
  messageCount: int,
};

type corpusUpload = {
  text: string,
  audienceHint: option(audience),
};

type ingestResult = {
  messageCount: int,
  embeddingCount: int,
  fingerprint: styleFingerprint,
};

type generateRequest = {
  prompt: string,
  audience: option(audience),
  retrievalCount: option(int),
  nudge: option(string),
};

type retrievedMessage = {
  text: string,
  audience,
  similarityScore: float,
};

type dimensionScore = {
  dimension: string,
  corpusValue: float,
  generatedValue: float,
  score: float,
};

type confidenceScore = {
  overall: float,
  dimensions: array(dimensionScore),
};

type generateResponse = {
  generatedText: string,
  retrievedExamples: array(retrievedMessage),
  fingerprintUsed: styleFingerprint,
  confidence: confidenceScore,
};

type storedMessage = {
  messageId: string,
  text: string,
  audience,
};

type messageList = {
  messages: array(storedMessage),
  total: int,
  limit: int,
  offset: int,
};

type historyEntry = {
  simulationId: string,
  prompt: string,
  audience,
  nudge: option(string),
  createdAt: string,
  result: generateResponse,
};

type historyList = {
  entries: array(historyEntry),
  total: int,
  limit: int,
  offset: int,
};

/* ----- Audience helpers ----- */

let audienceToString = (a: audience) =>
  switch (a) {
  | `Email => "email"
  | `Slack => "slack"
  | `Sms => "sms"
  | `Unknown => "unknown"
  };

let audienceOfString = (s: string): option(audience) =>
  switch (s) {
  | "email" => Some(`Email)
  | "slack" => Some(`Slack)
  | "sms" => Some(`Sms)
  | "unknown" => Some(`Unknown)
  | _ => None
  };

let audienceLabel = (a: audience) =>
  switch (a) {
  | `Email => "Email"
  | `Slack => "Slack"
  | `Sms => "SMS"
  | `Unknown => "Unknown"
  };

/* ----- Decoding ----- */

exception Decode_error(string);

let asObject = (json: Js.Json.t): Js.Dict.t(Js.Json.t) =>
  switch (Js.Json.classify(json)) {
  | Js.Json.JSONObject(o) => o
  | _ => raise(Decode_error("expected object"))
  };

let asString = (json: Js.Json.t): string =>
  switch (Js.Json.classify(json)) {
  | Js.Json.JSONString(s) => s
  | _ => raise(Decode_error("expected string"))
  };

let asNumber = (json: Js.Json.t): float =>
  switch (Js.Json.classify(json)) {
  | Js.Json.JSONNumber(n) => n
  | _ => raise(Decode_error("expected number"))
  };

let asInt = (json: Js.Json.t): int => int_of_float(asNumber(json));

let asArray = (json: Js.Json.t): array(Js.Json.t) =>
  switch (Js.Json.classify(json)) {
  | Js.Json.JSONArray(a) => a
  | _ => raise(Decode_error("expected array"))
  };

let field = (obj: Js.Dict.t(Js.Json.t), name: string): Js.Json.t =>
  switch (Js.Dict.get(obj, name)) {
  | Some(v) => v
  | None => raise(Decode_error("missing field: " ++ name))
  };

let optField = (obj: Js.Dict.t(Js.Json.t), name: string): option(Js.Json.t) =>
  switch (Js.Dict.get(obj, name)) {
  | Some(v) =>
    switch (Js.Json.classify(v)) {
    | Js.Json.JSONNull => None
    | _ => Some(v)
    }
  | None => None
  };

let decodeAudience = (json: Js.Json.t): audience => {
  let s = asString(json);
  switch (audienceOfString(s)) {
  | Some(a) => a
  | None => raise(Decode_error("unknown audience: " ++ s))
  };
};

let decodeStringArray = (json: Js.Json.t): array(string) =>
  Array.map(asString, asArray(json));

let decodeStyleFingerprint = (json: Js.Json.t): styleFingerprint => {
  let o = asObject(json);
  {
    formalityScore: asNumber(field(o, "formality_score")),
    avgSentenceLength: asNumber(field(o, "avg_sentence_length")),
    sentenceLengthVariance: asNumber(field(o, "sentence_length_variance")),
    exclamationRatio: asNumber(field(o, "exclamation_ratio")),
    ellipsisRatio: asNumber(field(o, "ellipsis_ratio")),
    emojiFrequency: asNumber(field(o, "emoji_frequency")),
    contractionRatio: asNumber(field(o, "contraction_ratio")),
    hedgingRatio: asNumber(field(o, "hedging_ratio")),
    commonOpeners: decodeStringArray(field(o, "common_openers")),
    commonClosers: decodeStringArray(field(o, "common_closers")),
    messageCount: asInt(field(o, "message_count")),
  };
};

let decodeIngestResult = (json: Js.Json.t): ingestResult => {
  let o = asObject(json);
  {
    messageCount: asInt(field(o, "message_count")),
    embeddingCount: asInt(field(o, "embedding_count")),
    fingerprint: decodeStyleFingerprint(field(o, "fingerprint")),
  };
};

let decodeRetrievedMessage = (json: Js.Json.t): retrievedMessage => {
  let o = asObject(json);
  {
    text: asString(field(o, "text")),
    audience: decodeAudience(field(o, "audience")),
    similarityScore: asNumber(field(o, "similarity_score")),
  };
};

let decodeDimensionScore = (json: Js.Json.t): dimensionScore => {
  let o = asObject(json);
  {
    dimension: asString(field(o, "dimension")),
    corpusValue: asNumber(field(o, "corpus_value")),
    generatedValue: asNumber(field(o, "generated_value")),
    score: asNumber(field(o, "score")),
  };
};

let decodeConfidenceScore = (json: Js.Json.t): confidenceScore => {
  let o = asObject(json);
  {
    overall: asNumber(field(o, "overall")),
    dimensions:
      Array.map(decodeDimensionScore, asArray(field(o, "dimensions"))),
  };
};

let decodeGenerateResponse = (json: Js.Json.t): generateResponse => {
  let o = asObject(json);
  {
    generatedText: asString(field(o, "generated_text")),
    retrievedExamples:
      Array.map(
        decodeRetrievedMessage,
        asArray(field(o, "retrieved_examples")),
      ),
    fingerprintUsed: decodeStyleFingerprint(field(o, "fingerprint_used")),
    confidence: decodeConfidenceScore(field(o, "confidence")),
  };
};

let decodeStoredMessage = (json: Js.Json.t): storedMessage => {
  let o = asObject(json);
  {
    messageId: asString(field(o, "message_id")),
    text: asString(field(o, "text")),
    audience: decodeAudience(field(o, "audience")),
  };
};

let decodeMessageList = (json: Js.Json.t): messageList => {
  let o = asObject(json);
  {
    messages: Array.map(decodeStoredMessage, asArray(field(o, "messages"))),
    total: asInt(field(o, "total")),
    limit: asInt(field(o, "limit")),
    offset: asInt(field(o, "offset")),
  };
};

let decodeHistoryEntry = (json: Js.Json.t): historyEntry => {
  let o = asObject(json);
  {
    simulationId: asString(field(o, "simulation_id")),
    prompt: asString(field(o, "prompt")),
    audience: decodeAudience(field(o, "audience")),
    nudge:
      switch (optField(o, "nudge")) {
      | Some(v) => Some(asString(v))
      | None => None
      },
    createdAt: asString(field(o, "created_at")),
    result: decodeGenerateResponse(field(o, "result")),
  };
};

let decodeHistoryList = (json: Js.Json.t): historyList => {
  let o = asObject(json);
  {
    entries: Array.map(decodeHistoryEntry, asArray(field(o, "entries"))),
    total: asInt(field(o, "total")),
    limit: asInt(field(o, "limit")),
    offset: asInt(field(o, "offset")),
  };
};

let decodeErrorBody = (json: Js.Json.t): string => {
  let o = asObject(json);
  asString(field(o, "error"));
};

/* ----- Encoding ----- */

let encodeCorpusUpload = (u: corpusUpload): Js.Json.t => {
  let pairs = [|("text", Js.Json.string(u.text))|];
  let pairs =
    switch (u.audienceHint) {
    | Some(a) =>
      Array.append(
        pairs,
        [|("audience_hint", Js.Json.string(audienceToString(a)))|],
      )
    | None => pairs
    };
  Js.Json.object_(Js.Dict.fromArray(pairs));
};

let encodeGenerateRequest = (r: generateRequest): Js.Json.t => {
  let pairs = ref([|("prompt", Js.Json.string(r.prompt))|]);
  switch (r.audience) {
  | Some(a) =>
    pairs :=
      Array.append(
        pairs^,
        [|("audience", Js.Json.string(audienceToString(a)))|],
      )
  | None => ()
  };
  switch (r.retrievalCount) {
  | Some(n) =>
    pairs :=
      Array.append(
        pairs^,
        [|("retrieval_count", Js.Json.number(float_of_int(n)))|],
      )
  | None => ()
  };
  switch (r.nudge) {
  | Some(s) when String.length(s) > 0 =>
    pairs := Array.append(pairs^, [|("nudge", Js.Json.string(s))|])
  | _ => ()
  };
  Js.Json.object_(Js.Dict.fromArray(pairs^));
};
