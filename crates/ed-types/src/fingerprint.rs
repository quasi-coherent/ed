//! Pure-computation style fingerprint analysis.
//!
//! Takes a `CorpusUpload` (raw text + optional audience hint), splits it into
//! discrete `Message` values, and derives a `StyleFingerprint` from them in a
//! single pass over the corpus.

use anyhow::{Result, anyhow};
use ed_axum::models::{Audience, CorpusUpload, StyleFingerprint};

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub body: String,
    pub audience: Audience,
}

const CONTRACTIONS: &[&str] = &[
    "don't",
    "won't",
    "can't",
    "isn't",
    "aren't",
    "wasn't",
    "weren't",
    "haven't",
    "hasn't",
    "hadn't",
    "doesn't",
    "didn't",
    "wouldn't",
    "shouldn't",
    "couldn't",
    "mustn't",
    "shan't",
    "i'm",
    "i've",
    "i'd",
    "i'll",
    "you're",
    "you've",
    "you'd",
    "you'll",
    "he's",
    "she's",
    "it's",
    "we're",
    "we've",
    "we'd",
    "we'll",
    "they're",
    "they've",
    "they'd",
    "they'll",
    "that's",
    "what's",
    "where's",
    "there's",
    "here's",
    "let's",
    "who's",
];

const HEDGES: &[&str] = &[
    "i think", "i guess", "kind of", "kinda", "sort of", "not sure", "just",
    "maybe", "probably", "perhaps", "might",
];

const CASUAL_GREETINGS: &[&str] = &["hey", "yo", "sup", "hiya"];
const FORMAL_GREETINGS: &[&str] = &["hello", "dear", "greetings"];

pub fn parse(upload: &CorpusUpload) -> Vec<Message> {
    let audience = upload.audience_hint.unwrap_or(Audience::Unknown);
    let text = upload.text.as_str();

    let by_blank: Vec<String> = text
        .split("\n\n")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let chunks = if by_blank.len() > 1 {
        by_blank
    } else {
        text.lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    };

    chunks.into_iter().map(|body| Message { body, audience }).collect()
}

#[derive(Default)]
struct Acc {
    total_words: usize,
    contraction_hits: usize,
    exclamation_count: usize,
    ellipsis_count: usize,
    emoji_count: usize,
    sentence_word_counts: Vec<f32>,
    hedge_sentences: usize,
    casual_greet: usize,
    formal_greet: usize,
    terminal_punct: usize,
    all_lower_msgs: usize,
    opener_phrases: Vec<String>,
    closer_phrases: Vec<String>,
}

pub fn analyze(messages: &[Message]) -> Result<StyleFingerprint> {
    if messages.is_empty() {
        return Err(anyhow!("cannot analyze an empty corpus"));
    }

    let mut acc = Acc::default();
    for m in messages {
        walk_message(m, &mut acc);
    }

    let sentence_count = acc.sentence_word_counts.len();
    let n_msgs = messages.len() as f32;

    let (avg_sentence_length, sentence_length_variance) = if sentence_count == 0
    {
        (0.0, 0.0)
    } else {
        let n = sentence_count as f32;
        let mean = acc.sentence_word_counts.iter().sum::<f32>() / n;
        let var = acc
            .sentence_word_counts
            .iter()
            .map(|l| (l - mean).powi(2))
            .sum::<f32>()
            / n;
        (mean, var)
    };

    let per_sentence = |count: usize| -> f32 {
        if sentence_count == 0 {
            0.0
        } else {
            count as f32 / sentence_count as f32
        }
    };

    let contraction_ratio = if acc.total_words == 0 {
        0.0
    } else {
        acc.contraction_hits as f32 / acc.total_words as f32
    };

    let mut formality_score: f32 = 0.5;
    formality_score -= 0.2 * (acc.casual_greet as f32 / n_msgs);
    formality_score += 0.2 * (acc.formal_greet as f32 / n_msgs);
    formality_score += 0.2 * (acc.terminal_punct as f32 / n_msgs);
    formality_score -= 0.2 * (acc.all_lower_msgs as f32 / n_msgs);
    formality_score -= 0.2 * contraction_ratio;
    let formality_score = formality_score.clamp(0.0, 1.0);

    Ok(StyleFingerprint {
        formality_score,
        avg_sentence_length,
        sentence_length_variance,
        exclamation_ratio: per_sentence(acc.exclamation_count),
        ellipsis_ratio: per_sentence(acc.ellipsis_count),
        emoji_frequency: acc.emoji_count as f32 / n_msgs,
        contraction_ratio,
        hedging_ratio: per_sentence(acc.hedge_sentences),
        common_openers: top_n(&acc.opener_phrases, 5),
        common_closers: top_n(&acc.closer_phrases, 5),
        message_count: messages.len() as i32,
    })
}

fn walk_message(m: &Message, acc: &mut Acc) {
    let trimmed = m.body.trim();
    if trimmed.is_empty() {
        return;
    }

    // Opener phrase + greeting classification (first 1–2 words).
    let mut head_iter = trimmed.split_whitespace();
    let w1 = head_iter.next().unwrap_or("");
    let w2 = head_iter.next();
    let first_word_norm =
        w1.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase();
    if CASUAL_GREETINGS.iter().any(|g| *g == first_word_norm) {
        acc.casual_greet += 1;
    }
    if FORMAL_GREETINGS.iter().any(|g| *g == first_word_norm) {
        acc.formal_greet += 1;
    }
    let opener = match w2 {
        Some(w2) => format!("{} {}", w1.to_lowercase(), w2.to_lowercase()),
        None => w1.to_lowercase(),
    };
    acc.opener_phrases.push(opener);

    // Closer: last non-empty short line (< 6 words).
    if let Some(last_line) =
        m.body.lines().filter(|l| !l.trim().is_empty()).last()
    {
        let lt = last_line.trim();
        if lt.split_whitespace().count() < 6 {
            acc.closer_phrases.push(lt.to_lowercase());
        }
    }

    // Terminal punctuation on the message as a whole.
    if let Some(last) = trimmed.chars().last() {
        if last == '.' || last == '!' || last == '?' {
            acc.terminal_punct += 1;
        }
    }

    // All-lowercase check.
    let mut has_alpha = false;
    let mut all_lower = true;
    for c in trimmed.chars() {
        if c.is_alphabetic() {
            has_alpha = true;
            if !c.is_lowercase() {
                all_lower = false;
                break;
            }
        }
    }
    if has_alpha && all_lower {
        acc.all_lower_msgs += 1;
    }

    // Single char walk: emoji, exclamation, ellipsis, sentence splitting.
    let mut sentence = String::new();
    let mut consecutive_dots: u8 = 0;
    for ch in m.body.chars() {
        if unic_emoji_char::is_emoji(ch) {
            acc.emoji_count += 1;
        }
        if ch == '!' {
            acc.exclamation_count += 1;
        }
        if ch == '…' {
            acc.ellipsis_count += 1;
        }
        if ch == '.' {
            consecutive_dots += 1;
            if consecutive_dots == 3 {
                acc.ellipsis_count += 1;
                consecutive_dots = 0;
            }
        } else {
            consecutive_dots = 0;
        }

        if ch == '.' || ch == '!' || ch == '?' {
            flush_sentence(&sentence, acc);
            sentence.clear();
        } else {
            sentence.push(ch);
        }
    }
    flush_sentence(&sentence, acc);
}

fn flush_sentence(sentence: &str, acc: &mut Acc) {
    let trimmed = sentence.trim();
    if trimmed.is_empty() {
        return;
    }
    let lower = trimmed.to_lowercase();
    if HEDGES.iter().any(|h| lower.contains(h)) {
        acc.hedge_sentences += 1;
    }
    let mut wc: usize = 0;
    for word in trimmed.split_whitespace() {
        wc += 1;
        acc.total_words += 1;
        let normalized: String = word
            .chars()
            .filter(|c| c.is_alphabetic() || *c == '\'')
            .collect::<String>()
            .to_lowercase();
        if CONTRACTIONS.iter().any(|c| *c == normalized) {
            acc.contraction_hits += 1;
        }
    }
    acc.sentence_word_counts.push(wc as f32);
}

fn top_n(items: &[String], n: usize) -> Vec<String> {
    let mut counts: Vec<(String, usize)> = Vec::new();
    for item in items {
        if let Some(entry) = counts.iter_mut().find(|(s, _)| s == item) {
            entry.1 += 1;
        } else {
            counts.push((item.clone(), 1));
        }
    }
    counts.sort_by(|a, b| b.1.cmp(&a.1));
    counts.into_iter().take(n).map(|(s, _)| s).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn msg(body: &str) -> Message {
        Message { body: body.to_string(), audience: Audience::Unknown }
    }

    #[test]
    fn empty_corpus_errors() {
        assert!(analyze(&[]).is_err());
    }

    #[test]
    fn known_contractions_produce_expected_ratio() {
        let messages = vec![msg("I'm fine and you're great")];
        let fp = analyze(&messages).expect("analyze");
        assert!((fp.contraction_ratio - (2.0 / 5.0)).abs() < 1e-5);
    }

    #[test]
    fn known_emoji_produces_expected_frequency() {
        let messages =
            vec![msg("hello 😀 world"), msg("nothing here"), msg("🎉🎉")];
        let fp = analyze(&messages).expect("analyze");
        assert!((fp.emoji_frequency - 1.0).abs() < 1e-5);
    }

    #[test]
    fn parse_splits_on_blank_lines() {
        let upload = CorpusUpload {
            text: "first message\nwith two lines\n\nsecond message\n\nthird"
                .to_string(),
            audience_hint: Some(Audience::Email),
        };
        let parsed = parse(&upload);
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[0].audience, Audience::Email);
    }

    #[test]
    fn parse_falls_back_to_lines_when_no_blank_separators() {
        let upload = CorpusUpload {
            text: "line one\nline two\nline three".to_string(),
            audience_hint: None,
        };
        let parsed = parse(&upload);
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[0].audience, Audience::Unknown);
    }
}
