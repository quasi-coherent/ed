/* Compute how well a generated string matches a StyleFingerprint. Mirrors
   the dimensions of Api.styleFingerprint so the per-dimension breakdown
   lines up with what the backend stored. */

[@mel.send] external jsSplit: (string, Js.Re.t) => array(string) = "split";
[@mel.send]
external jsMatchAll: (string, Js.Re.t) => Js.Nullable.t(array(string)) =
  "match";
[@mel.send] external jsTrim: string => string = "trim";
[@mel.send] external jsToLower: string => string = "toLowerCase";
[@mel.send] external jsIncludes: (string, string) => bool = "includes";

let trimAndKeep = (a: array(string)): array(string) =>
  a
  |> Array.to_list
  |> List.filter(s => String.length(jsTrim(s)) > 0)
  |> Array.of_list;

let splitSentences = (s: string): array(string) => {
  let re = [%mel.re "/[.!?]+\\s+|[.!?]+$/"];
  trimAndKeep(jsSplit(s, re));
};

let splitWords = (s: string): array(string) => {
  let re = [%mel.re "/\\s+/"];
  trimAndKeep(jsSplit(s, re));
};

let countMatches = (s: string, re: Js.Re.t): int =>
  switch (Js.Nullable.toOption(jsMatchAll(s, re))) {
  | Some(arr) => Array.length(arr)
  | None => 0
  };

let contractionsRe = [%mel.re "/\\b\\w+'(?:s|re|ve|d|ll|t|m)\\b/g"];
let exclamationRe = [%mel.re "/!/g"];
let ellipsisRe = [%mel.re "/\\.{3}|…/g"];
let emojiRe = [%mel.re "/[\\u{1F300}-\\u{1FAFF}\\u{2600}-\\u{27BF}]/gu"];

let hedgeWords = [|
  "just",
  "maybe",
  "perhaps",
  "kind of",
  "kinda",
  "sort of",
  "i think",
  "i guess",
  "probably",
  "possibly",
  "might",
  "somewhat",
  "a bit",
|];

let formalMarkers = [|
  "therefore",
  "furthermore",
  "however",
  "regarding",
  "additionally",
  "consequently",
  "sincerely",
  "respectfully",
  "kindly",
|];

let casualMarkers = [|
  "lol",
  "haha",
  "yeah",
  "yep",
  "nope",
  "gonna",
  "wanna",
  "gotta",
  "tbh",
  "imo",
  "btw",
|];

let containsAny = (text: string, needles: array(string)): bool => {
  let t = jsToLower(text);
  let found = ref(false);
  Array.iter(
    n =>
      if (! found^ && jsIncludes(t, n)) {
        found := true;
      },
    needles,
  );
  found^;
};

let countContaining =
    (sentences: array(string), needles: array(string)): int => {
  let c = ref(0);
  Array.iter(
    s =>
      if (containsAny(s, needles)) {
        incr(c);
      },
    sentences,
  );
  c^;
};

/* ---------- measure ---------- */

type measured = {
  formalityScore: float,
  avgSentenceLength: float,
  sentenceLengthVariance: float,
  exclamationRatio: float,
  ellipsisRatio: float,
  emojiFrequency: float,
  contractionRatio: float,
  hedgingRatio: float,
};

let measure = (text: string): measured => {
  let sentences = splitSentences(text);
  let nSentences = Array.length(sentences);
  let safeSentences = max(nSentences, 1);

  let wordCounts =
    Array.map(s => float_of_int(Array.length(splitWords(s))), sentences);

  let avgLen =
    if (nSentences == 0) {
      0.0;
    } else {
      Array.fold_left((+.), 0.0, wordCounts) /. float_of_int(nSentences);
    };

  let variance =
    if (nSentences == 0) {
      0.0;
    } else {
      let sumSq =
        Array.fold_left(
          (acc, w) => acc +. (w -. avgLen) ** 2.0,
          0.0,
          wordCounts,
        );
      sumSq /. float_of_int(nSentences);
    };

  let totalWords =
    Array.fold_left((acc, w) => acc +. w, 0.0, wordCounts) |> int_of_float;
  let safeWords = max(totalWords, 1);

  let exclamations = countMatches(text, exclamationRe);
  let ellipses = countMatches(text, ellipsisRe);
  let emoji = countMatches(text, emojiRe);
  let contractions = countMatches(jsToLower(text), contractionsRe);

  let formalHits = countContaining(sentences, formalMarkers);
  let casualHits = countContaining(sentences, casualMarkers);
  let formalRate = float_of_int(formalHits) /. float_of_int(safeSentences);
  let casualRate = float_of_int(casualHits) /. float_of_int(safeSentences);
  let formality = max(0.0, min(1.0, 0.5 +. formalRate -. casualRate));

  {
    formalityScore: formality,
    avgSentenceLength: avgLen,
    sentenceLengthVariance: variance,
    exclamationRatio:
      float_of_int(exclamations) /. float_of_int(safeSentences),
    ellipsisRatio: float_of_int(ellipses) /. float_of_int(safeSentences),
    emojiFrequency: float_of_int(emoji),
    contractionRatio: float_of_int(contractions) /. float_of_int(safeWords),
    hedgingRatio:
      float_of_int(countContaining(sentences, hedgeWords))
      /. float_of_int(safeSentences),
  };
};

/* ---------- per-dimension scoring ---------- */

let absScore = (corpus: float, generated: float, tolerance: float): float => {
  let diff = abs_float(corpus -. generated);
  let raw = 1.0 -. diff /. tolerance;
  max(0.0, min(1.0, raw));
};

let relScore = (corpus: float, generated: float, frac: float): float => {
  let tol = max(frac *. abs_float(corpus), 1.0);
  absScore(corpus, generated, tol);
};

let score =
    (~fingerprint: Api.styleFingerprint, ~text: string): Api.confidenceScore => {
  let m = measure(text);
  let dims: array(Api.dimensionScore) = [|
    {
      dimension: "formality_score",
      corpusValue: fingerprint.formalityScore,
      generatedValue: m.formalityScore,
      score: absScore(fingerprint.formalityScore, m.formalityScore, 0.5),
    },
    {
      dimension: "avg_sentence_length",
      corpusValue: fingerprint.avgSentenceLength,
      generatedValue: m.avgSentenceLength,
      score:
        relScore(fingerprint.avgSentenceLength, m.avgSentenceLength, 0.5),
    },
    {
      dimension: "sentence_length_variance",
      corpusValue: fingerprint.sentenceLengthVariance,
      generatedValue: m.sentenceLengthVariance,
      score:
        relScore(
          fingerprint.sentenceLengthVariance,
          m.sentenceLengthVariance,
          1.0,
        ),
    },
    {
      dimension: "exclamation_ratio",
      corpusValue: fingerprint.exclamationRatio,
      generatedValue: m.exclamationRatio,
      score: absScore(fingerprint.exclamationRatio, m.exclamationRatio, 0.5),
    },
    {
      dimension: "ellipsis_ratio",
      corpusValue: fingerprint.ellipsisRatio,
      generatedValue: m.ellipsisRatio,
      score: absScore(fingerprint.ellipsisRatio, m.ellipsisRatio, 0.5),
    },
    {
      dimension: "emoji_frequency",
      corpusValue: fingerprint.emojiFrequency,
      generatedValue: m.emojiFrequency,
      score: absScore(fingerprint.emojiFrequency, m.emojiFrequency, 3.0),
    },
    {
      dimension: "contraction_ratio",
      corpusValue: fingerprint.contractionRatio,
      generatedValue: m.contractionRatio,
      score: absScore(fingerprint.contractionRatio, m.contractionRatio, 0.2),
    },
    {
      dimension: "hedging_ratio",
      corpusValue: fingerprint.hedgingRatio,
      generatedValue: m.hedgingRatio,
      score: absScore(fingerprint.hedgingRatio, m.hedgingRatio, 0.4),
    },
  |];
  let total =
    Array.fold_left(
      (acc, d: Api.dimensionScore) => acc +. d.score,
      0.0,
      dims,
    );
  let overall = total /. float_of_int(Array.length(dims));
  {
    overall,
    dimensions: dims,
  };
};
