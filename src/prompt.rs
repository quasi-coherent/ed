//! Prompt assembly and confidence scoring helpers.

use ed_axum::models::{
    ConfidenceScore, DimensionScore, RetrievedMessage, StyleFingerprint,
};

pub fn render_prompt(
    fp: &StyleFingerprint,
    examples: &[RetrievedMessage],
    user_prompt: &str,
    nudge: Option<&str>,
) -> String {
    let mut out = String::new();
    out.push_str("You are ghostwriting a message on behalf of someone.\n");
    out.push_str("Here is a description of their writing style:\n");
    out.push_str(&describe_fingerprint(fp));
    out.push_str("\n\nHere are real examples of their writing:\n");
    for ex in examples {
        out.push_str(&format!("- ({}): {}\n", ex.audience, ex.text));
    }
    out.push_str("\nNow write the following, matching their voice exactly:\n");
    out.push_str(user_prompt);
    if let Some(n) = nudge {
        out.push_str("\n\nAdditional guidance: ");
        out.push_str(n);
    }
    out
}

fn describe_fingerprint(fp: &StyleFingerprint) -> String {
    let mut s = String::new();
    s.push_str(&format!(
        "- Formality score: {:.2} (0 = very casual, 1 = very formal).\n",
        fp.formality_score
    ));
    s.push_str(&format!(
        "- Sentences average {:.1} words (variance {:.2}).\n",
        fp.avg_sentence_length, fp.sentence_length_variance
    ));
    s.push_str(&format!(
        "- Exclamations per sentence: {:.2}.\n",
        fp.exclamation_ratio
    ));
    s.push_str(&format!(
        "- Ellipses per sentence: {:.2}.\n",
        fp.ellipsis_ratio
    ));
    s.push_str(&format!("- Emoji per message: {:.2}.\n", fp.emoji_frequency));
    s.push_str(&format!("- Contraction ratio: {:.2}.\n", fp.contraction_ratio));
    s.push_str(&format!("- Hedging ratio: {:.2}.\n", fp.hedging_ratio));
    if !fp.common_openers.is_empty() {
        s.push_str(&format!(
            "- Common openers: {}.\n",
            fp.common_openers.join(", ")
        ));
    }
    if !fp.common_closers.is_empty() {
        s.push_str(&format!(
            "- Common closers: {}.\n",
            fp.common_closers.join(", ")
        ));
    }
    s
}

pub fn score_confidence(
    corpus: &StyleFingerprint,
    generated: &StyleFingerprint,
) -> ConfidenceScore {
    let dims = [
        ("formality_score", corpus.formality_score, generated.formality_score),
        (
            "avg_sentence_length",
            corpus.avg_sentence_length,
            generated.avg_sentence_length,
        ),
        (
            "sentence_length_variance",
            corpus.sentence_length_variance,
            generated.sentence_length_variance,
        ),
        (
            "exclamation_ratio",
            corpus.exclamation_ratio,
            generated.exclamation_ratio,
        ),
        ("ellipsis_ratio", corpus.ellipsis_ratio, generated.ellipsis_ratio),
        ("emoji_frequency", corpus.emoji_frequency, generated.emoji_frequency),
        (
            "contraction_ratio",
            corpus.contraction_ratio,
            generated.contraction_ratio,
        ),
        ("hedging_ratio", corpus.hedging_ratio, generated.hedging_ratio),
    ];

    let dimensions: Vec<DimensionScore> = dims
        .iter()
        .map(|(name, c, g)| DimensionScore {
            dimension: name.to_string(),
            corpus_value: *c,
            generated_value: *g,
            score: dimension_score(*c, *g),
        })
        .collect();

    let overall = if dimensions.is_empty() {
        0.0
    } else {
        dimensions.iter().map(|d| d.score).sum::<f32>()
            / dimensions.len() as f32
    };
    ConfidenceScore { overall, dimensions }
}

fn dimension_score(corpus: f32, generated: f32) -> f32 {
    if corpus.abs() < 1e-6 {
        if generated.abs() < 1e-6 { 1.0 } else { 0.0 }
    } else {
        let rel = (generated - corpus).abs() / corpus.abs();
        (1.0 - rel).clamp(0.0, 1.0)
    }
}
