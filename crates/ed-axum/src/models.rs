#![allow(unused_qualifications)]

use http::HeaderValue;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::{models, types::*};

#[allow(dead_code)]
fn from_validation_error(
    e: validator::ValidationError,
) -> validator::ValidationErrors {
    let mut errs = validator::ValidationErrors::new();
    errs.add("na", e);
    errs
}

#[allow(dead_code)]
pub fn check_xss_string(
    v: &str,
) -> std::result::Result<(), validator::ValidationError> {
    if ammonia::is_html(v) {
        std::result::Result::Err(validator::ValidationError::new(
            "xss detected",
        ))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_vec_string(
    v: &[String],
) -> std::result::Result<(), validator::ValidationError> {
    if v.iter().any(|i| ammonia::is_html(i)) {
        std::result::Result::Err(validator::ValidationError::new(
            "xss detected",
        ))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_map_string(
    v: &std::collections::HashMap<String, String>,
) -> std::result::Result<(), validator::ValidationError> {
    if v.keys().any(|k| ammonia::is_html(k))
        || v.values().any(|v| ammonia::is_html(v))
    {
        std::result::Result::Err(validator::ValidationError::new(
            "xss detected",
        ))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_map_nested<T>(
    v: &std::collections::HashMap<String, T>,
) -> std::result::Result<(), validator::ValidationError>
where
    T: validator::Validate,
{
    if v.keys().any(|k| ammonia::is_html(k))
        || v.values().any(|v| v.validate().is_err())
    {
        std::result::Result::Err(validator::ValidationError::new(
            "xss detected",
        ))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_map<T>(
    v: &std::collections::HashMap<String, T>,
) -> std::result::Result<(), validator::ValidationError> {
    if v.keys().any(|k| ammonia::is_html(k)) {
        std::result::Result::Err(validator::ValidationError::new(
            "xss detected",
        ))
    } else {
        std::result::Result::Ok(())
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DataUserIdDeletePathParams {
    pub user_id: uuid::Uuid,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DataUserIdFingerprintGetPathParams {
    pub user_id: uuid::Uuid,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DataUserIdGetPathParams {
    pub user_id: uuid::Uuid,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DataUserIdGetQueryParams {
    /// Filter by message audience
    #[serde(rename = "audience")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<models::Audience>,
    #[serde(rename = "limit")]
    #[validate(range(min = 1u8, max = 200u8))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(rename = "offset")]
    #[validate(range(min = 0u32))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DataUserIdPostPathParams {
    pub user_id: uuid::Uuid,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SimulateUserIdGetPathParams {
    pub user_id: uuid::Uuid,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SimulateUserIdGetQueryParams {
    #[serde(rename = "limit")]
    #[validate(range(min = 1u8, max = 100u8))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(rename = "offset")]
    #[validate(range(min = 0u32))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SimulateUserIdPostPathParams {
    pub user_id: uuid::Uuid,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SimulateUserIdSimulationsSimulationIdDeletePathParams {
    pub user_id: uuid::Uuid,
    pub simulation_id: uuid::Uuid,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SimulateUserIdSimulationsSimulationIdGetPathParams {
    pub user_id: uuid::Uuid,
    pub simulation_id: uuid::Uuid,
}

/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
#[cfg_attr(
    feature = "conversion",
    derive(frunk_enum_derive::LabelledGenericEnum)
)]
pub enum Audience {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "slack")]
    Slack,
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "unknown")]
    Unknown,
}

impl validator::Validate for Audience {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for Audience {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Audience::Email => write!(f, "email"),
            Audience::Slack => write!(f, "slack"),
            Audience::Sms => write!(f, "sms"),
            Audience::Unknown => write!(f, "unknown"),
        }
    }
}

impl std::str::FromStr for Audience {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "email" => std::result::Result::Ok(Audience::Email),
            "slack" => std::result::Result::Ok(Audience::Slack),
            "sms" => std::result::Result::Ok(Audience::Sms),
            "unknown" => std::result::Result::Ok(Audience::Unknown),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ConfidenceScore {
    /// Aggregate match score across all style dimensions
    #[serde(rename = "overall")]
    #[validate(range(min = 0.0f32, max = 1.0f32))]
    pub overall: f32,

    /// Per-dimension breakdown of how well the output matched the corpus
    #[serde(rename = "dimensions")]
    #[validate(nested)]
    pub dimensions: Vec<models::DimensionScore>,
}

impl ConfidenceScore {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        overall: f32,
        dimensions: Vec<models::DimensionScore>,
    ) -> ConfidenceScore {
        ConfidenceScore { overall, dimensions }
    }
}

/// Converts the ConfidenceScore value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ConfidenceScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("overall".to_string()),
            Some(self.overall.to_string()),
            // Skipping dimensions in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ConfidenceScore value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ConfidenceScore {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub overall: Vec<f32>,
            pub dimensions: Vec<Vec<models::DimensionScore>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ConfidenceScore"
                            .to_string(),
                    );
                },
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "overall" => intermediate_rep.overall.push(<f32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "dimensions" => return std::result::Result::Err("Parsing a container in this style is not supported in ConfidenceScore".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ConfidenceScore".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ConfidenceScore {
            overall: intermediate_rep.overall.into_iter().next().ok_or_else(
                || "overall missing in ConfidenceScore".to_string(),
            )?,
            dimensions: intermediate_rep
                .dimensions
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "dimensions missing in ConfidenceScore".to_string()
                })?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ConfidenceScore> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ConfidenceScore>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ConfidenceScore>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for ConfidenceScore - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<ConfidenceScore>
{
    type Error = String;

    fn try_from(
        hdr_value: HeaderValue,
    ) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ConfidenceScore as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    },
                    std::result::Result::Err(err) => {
                        std::result::Result::Err(format!(
                            r#"Unable to convert header value '{value}' into ConfidenceScore - {err}"#
                        ))
                    },
                }
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CorpusDelete {
    /// User to remove data for.
    #[serde(rename = "user_id")]
    pub user_id: uuid::Uuid,
}

impl CorpusDelete {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(user_id: uuid::Uuid) -> CorpusDelete {
        CorpusDelete { user_id }
    }
}

/// Converts the CorpusDelete value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for CorpusDelete {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping user_id in query parameter serialization

        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CorpusDelete value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CorpusDelete {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub user_id: Vec<uuid::Uuid>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing CorpusDelete".to_string(),
                    );
                },
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "user_id" => intermediate_rep.user_id.push(
                        <uuid::Uuid as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing CorpusDelete"
                                .to_string(),
                        );
                    },
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CorpusDelete {
            user_id: intermediate_rep
                .user_id
                .into_iter()
                .next()
                .ok_or_else(|| "user_id missing in CorpusDelete".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CorpusDelete> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<CorpusDelete>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<CorpusDelete>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for CorpusDelete - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<CorpusDelete>
{
    type Error = String;

    fn try_from(
        hdr_value: HeaderValue,
    ) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <CorpusDelete as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    },
                    std::result::Result::Err(err) => {
                        std::result::Result::Err(format!(
                            r#"Unable to convert header value '{value}' into CorpusDelete - {err}"#
                        ))
                    },
                }
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CorpusUpload {
    /// Raw corpus text — emails, Slack messages, texts, etc.
    #[serde(rename = "text")]
    #[validate(length(min = 1), custom(function = "check_xss_string"))]
    pub text: String,

    /// If all text is from one audience, provide a hint. Otherwise the parser will attempt to detect audience per message.
    #[serde(rename = "audience_hint")]
    #[validate(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience_hint: Option<models::Audience>,
}

impl CorpusUpload {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(text: String) -> CorpusUpload {
        CorpusUpload { text, audience_hint: None }
    }
}

/// Converts the CorpusUpload value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for CorpusUpload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("text".to_string()),
            Some(self.text.to_string()),
            // Skipping audience_hint in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CorpusUpload value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CorpusUpload {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub text: Vec<String>,
            pub audience_hint: Vec<models::Audience>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing CorpusUpload".to_string(),
                    );
                },
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "text" => intermediate_rep.text.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "audience_hint" => intermediate_rep.audience_hint.push(
                        <models::Audience as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing CorpusUpload"
                                .to_string(),
                        );
                    },
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CorpusUpload {
            text: intermediate_rep
                .text
                .into_iter()
                .next()
                .ok_or_else(|| "text missing in CorpusUpload".to_string())?,
            audience_hint: intermediate_rep.audience_hint.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CorpusUpload> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<CorpusUpload>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<CorpusUpload>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for CorpusUpload - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<CorpusUpload>
{
    type Error = String;

    fn try_from(
        hdr_value: HeaderValue,
    ) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <CorpusUpload as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    },
                    std::result::Result::Err(err) => {
                        std::result::Result::Err(format!(
                            r#"Unable to convert header value '{value}' into CorpusUpload - {err}"#
                        ))
                    },
                }
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DimensionScore {
    /// Name of the style dimension, e.g. \"formality_score\", \"avg_sentence_length\", \"exclamation_ratio\"
    #[serde(rename = "dimension")]
    #[validate(custom(function = "check_xss_string"))]
    pub dimension: String,

    /// The value measured from the corpus fingerprint
    #[serde(rename = "corpus_value")]
    pub corpus_value: f32,

    /// The value measured from the generated text
    #[serde(rename = "generated_value")]
    pub generated_value: f32,

    /// How closely the generated value matches the corpus value for this dimension
    #[serde(rename = "score")]
    #[validate(range(min = 0.0f32, max = 1.0f32))]
    pub score: f32,
}

impl DimensionScore {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        dimension: String,
        corpus_value: f32,
        generated_value: f32,
        score: f32,
    ) -> DimensionScore {
        DimensionScore { dimension, corpus_value, generated_value, score }
    }
}

/// Converts the DimensionScore value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for DimensionScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("dimension".to_string()),
            Some(self.dimension.to_string()),
            Some("corpus_value".to_string()),
            Some(self.corpus_value.to_string()),
            Some("generated_value".to_string()),
            Some(self.generated_value.to_string()),
            Some("score".to_string()),
            Some(self.score.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DimensionScore value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DimensionScore {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub dimension: Vec<String>,
            pub corpus_value: Vec<f32>,
            pub generated_value: Vec<f32>,
            pub score: Vec<f32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing DimensionScore"
                            .to_string(),
                    );
                },
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "dimension" => intermediate_rep.dimension.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "corpus_value" => intermediate_rep.corpus_value.push(
                        <f32 as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "generated_value" => intermediate_rep.generated_value.push(
                        <f32 as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "score" => intermediate_rep.score.push(
                        <f32 as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing DimensionScore"
                                .to_string(),
                        );
                    },
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DimensionScore {
            dimension: intermediate_rep
                .dimension
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "dimension missing in DimensionScore".to_string()
                })?,
            corpus_value: intermediate_rep
                .corpus_value
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "corpus_value missing in DimensionScore".to_string()
                })?,
            generated_value: intermediate_rep
                .generated_value
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "generated_value missing in DimensionScore".to_string()
                })?,
            score: intermediate_rep
                .score
                .into_iter()
                .next()
                .ok_or_else(|| "score missing in DimensionScore".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DimensionScore> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<DimensionScore>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<DimensionScore>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for DimensionScore - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<DimensionScore>
{
    type Error = String;

    fn try_from(
        hdr_value: HeaderValue,
    ) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <DimensionScore as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    },
                    std::result::Result::Err(err) => {
                        std::result::Result::Err(format!(
                            r#"Unable to convert header value '{value}' into DimensionScore - {err}"#
                        ))
                    },
                }
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ErrorBody {
    #[serde(rename = "error")]
    #[validate(custom(function = "check_xss_string"))]
    pub error: String,
}

impl ErrorBody {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(error: String) -> ErrorBody {
        ErrorBody { error }
    }
}

/// Converts the ErrorBody value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ErrorBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> =
            vec![Some("error".to_string()), Some(self.error.to_string())];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ErrorBody value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ErrorBody {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub error: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ErrorBody".to_string(),
                    );
                },
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "error" => intermediate_rep.error.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ErrorBody"
                                .to_string(),
                        );
                    },
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ErrorBody {
            error: intermediate_rep
                .error
                .into_iter()
                .next()
                .ok_or_else(|| "error missing in ErrorBody".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ErrorBody> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ErrorBody>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ErrorBody>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for ErrorBody - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ErrorBody> {
    type Error = String;

    fn try_from(
        hdr_value: HeaderValue,
    ) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ErrorBody as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    },
                    std::result::Result::Err(err) => {
                        std::result::Result::Err(format!(
                            r#"Unable to convert header value '{value}' into ErrorBody - {err}"#
                        ))
                    },
                }
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GenerateRequest {
    /// Describes the text to generate, e.g. \"write a follow-up email to a recruiter after a first-round interview\"
    #[serde(rename = "prompt")]
    #[validate(length(min = 1), custom(function = "check_xss_string"))]
    pub prompt: String,

    /// Preferred output audience. Influences which corpus examples are retrieved and how the prompt is framed.
    #[serde(rename = "audience")]
    #[validate(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<models::Audience>,

    /// Number of similar corpus messages to retrieve as few-shot examples
    #[serde(rename = "retrieval_count")]
    #[validate(range(min = 1u8, max = 10u8))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_count: Option<u8>,

    /// Optional freeform correction appended to the generation prompt, e.g. \"make it shorter\" or \"less formal\"
    #[serde(rename = "nudge")]
    #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nudge: Option<String>,
}

impl GenerateRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(prompt: String) -> GenerateRequest {
        GenerateRequest {
            prompt,
            audience: None,
            retrieval_count: Some(4),
            nudge: None,
        }
    }
}

/// Converts the GenerateRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for GenerateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("prompt".to_string()),
            Some(self.prompt.to_string()),
            // Skipping audience in query parameter serialization
            self.retrieval_count.as_ref().map(|retrieval_count| {
                ["retrieval_count".to_string(), retrieval_count.to_string()]
                    .join(",")
            }),
            self.nudge.as_ref().map(|nudge| {
                ["nudge".to_string(), nudge.to_string()].join(",")
            }),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GenerateRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GenerateRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub prompt: Vec<String>,
            pub audience: Vec<models::Audience>,
            pub retrieval_count: Vec<u8>,
            pub nudge: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing GenerateRequest"
                            .to_string(),
                    );
                },
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "prompt" => intermediate_rep.prompt.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "audience" => intermediate_rep.audience.push(
                        <models::Audience as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "retrieval_count" => intermediate_rep.retrieval_count.push(
                        <u8 as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "nudge" => intermediate_rep.nudge.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing GenerateRequest"
                                .to_string(),
                        );
                    },
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GenerateRequest {
            prompt: intermediate_rep.prompt.into_iter().next().ok_or_else(
                || "prompt missing in GenerateRequest".to_string(),
            )?,
            audience: intermediate_rep.audience.into_iter().next(),
            retrieval_count: intermediate_rep
                .retrieval_count
                .into_iter()
                .next(),
            nudge: intermediate_rep.nudge.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GenerateRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<GenerateRequest>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<GenerateRequest>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for GenerateRequest - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<GenerateRequest>
{
    type Error = String;

    fn try_from(
        hdr_value: HeaderValue,
    ) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <GenerateRequest as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    },
                    std::result::Result::Err(err) => {
                        std::result::Result::Err(format!(
                            r#"Unable to convert header value '{value}' into GenerateRequest - {err}"#
                        ))
                    },
                }
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GenerateResponse {
    #[serde(rename = "generated_text")]
    #[validate(custom(function = "check_xss_string"))]
    pub generated_text: String,

    /// The corpus messages used as few-shot examples
    #[serde(rename = "retrieved_examples")]
    #[validate(nested)]
    pub retrieved_examples: Vec<models::RetrievedMessage>,

    /// The fingerprint included in the generation prompt
    #[serde(rename = "fingerprint_used")]
    #[validate(nested)]
    pub fingerprint_used: models::StyleFingerprint,

    #[serde(rename = "confidence")]
    #[validate(nested)]
    pub confidence: models::ConfidenceScore,
}

impl GenerateResponse {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        generated_text: String,
        retrieved_examples: Vec<models::RetrievedMessage>,
        fingerprint_used: models::StyleFingerprint,
        confidence: models::ConfidenceScore,
    ) -> GenerateResponse {
        GenerateResponse {
            generated_text,
            retrieved_examples,
            fingerprint_used,
            confidence,
        }
    }
}

/// Converts the GenerateResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for GenerateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("generated_text".to_string()),
            Some(self.generated_text.to_string()),
            // Skipping retrieved_examples in query parameter serialization

            // Skipping fingerprint_used in query parameter serialization

            // Skipping confidence in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GenerateResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GenerateResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub generated_text: Vec<String>,
            pub retrieved_examples: Vec<Vec<models::RetrievedMessage>>,
            pub fingerprint_used: Vec<models::StyleFingerprint>,
            pub confidence: Vec<models::ConfidenceScore>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing GenerateResponse"
                            .to_string(),
                    );
                },
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "generated_text" => intermediate_rep.generated_text.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "retrieved_examples" => return std::result::Result::Err("Parsing a container in this style is not supported in GenerateResponse".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "fingerprint_used" => intermediate_rep.fingerprint_used.push(<models::StyleFingerprint as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "confidence" => intermediate_rep.confidence.push(<models::ConfidenceScore as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GenerateResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GenerateResponse {
            generated_text: intermediate_rep
                .generated_text
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "generated_text missing in GenerateResponse".to_string()
                })?,
            retrieved_examples: intermediate_rep
                .retrieved_examples
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "retrieved_examples missing in GenerateResponse".to_string()
                })?,
            fingerprint_used: intermediate_rep
                .fingerprint_used
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "fingerprint_used missing in GenerateResponse".to_string()
                })?,
            confidence: intermediate_rep
                .confidence
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "confidence missing in GenerateResponse".to_string()
                })?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GenerateResponse> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<GenerateResponse>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<GenerateResponse>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for GenerateResponse - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<GenerateResponse>
{
    type Error = String;

    fn try_from(
        hdr_value: HeaderValue,
    ) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <GenerateResponse as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    },
                    std::result::Result::Err(err) => {
                        std::result::Result::Err(format!(
                            r#"Unable to convert header value '{value}' into GenerateResponse - {err}"#
                        ))
                    },
                }
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct HistoryList {
    #[serde(rename = "entries")]
    #[validate(nested)]
    pub entries: Vec<models::SimulationEntry>,

    #[serde(rename = "total")]
    pub total: i32,

    #[serde(rename = "limit")]
    pub limit: i32,

    #[serde(rename = "offset")]
    pub offset: i32,
}

impl HistoryList {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        entries: Vec<models::SimulationEntry>,
        total: i32,
        limit: i32,
        offset: i32,
    ) -> HistoryList {
        HistoryList { entries, total, limit, offset }
    }
}

/// Converts the HistoryList value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for HistoryList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping entries in query parameter serialization
            Some("total".to_string()),
            Some(self.total.to_string()),
            Some("limit".to_string()),
            Some(self.limit.to_string()),
            Some("offset".to_string()),
            Some(self.offset.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a HistoryList value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for HistoryList {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub entries: Vec<Vec<models::SimulationEntry>>,
            pub total: Vec<i32>,
            pub limit: Vec<i32>,
            pub offset: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing HistoryList".to_string(),
                    );
                },
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "entries" => return std::result::Result::Err("Parsing a container in this style is not supported in HistoryList".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "total" => intermediate_rep.total.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "limit" => intermediate_rep.limit.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "offset" => intermediate_rep.offset.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing HistoryList".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(HistoryList {
            entries: intermediate_rep
                .entries
                .into_iter()
                .next()
                .ok_or_else(|| "entries missing in HistoryList".to_string())?,
            total: intermediate_rep
                .total
                .into_iter()
                .next()
                .ok_or_else(|| "total missing in HistoryList".to_string())?,
            limit: intermediate_rep
                .limit
                .into_iter()
                .next()
                .ok_or_else(|| "limit missing in HistoryList".to_string())?,
            offset: intermediate_rep
                .offset
                .into_iter()
                .next()
                .ok_or_else(|| "offset missing in HistoryList".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<HistoryList> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<HistoryList>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<HistoryList>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for HistoryList - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<HistoryList>
{
    type Error = String;

    fn try_from(
        hdr_value: HeaderValue,
    ) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <HistoryList as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    },
                    std::result::Result::Err(err) => {
                        std::result::Result::Err(format!(
                            r#"Unable to convert header value '{value}' into HistoryList - {err}"#
                        ))
                    },
                }
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct IngestResult {
    /// Number of messages parsed from the corpus
    #[serde(rename = "message_count")]
    pub message_count: i32,

    /// Number of embeddings successfully stored
    #[serde(rename = "embedding_count")]
    pub embedding_count: i32,

    #[serde(rename = "fingerprint")]
    #[validate(nested)]
    pub fingerprint: models::StyleFingerprint,
}

impl IngestResult {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        message_count: i32,
        embedding_count: i32,
        fingerprint: models::StyleFingerprint,
    ) -> IngestResult {
        IngestResult { message_count, embedding_count, fingerprint }
    }
}

/// Converts the IngestResult value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for IngestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("message_count".to_string()),
            Some(self.message_count.to_string()),
            Some("embedding_count".to_string()),
            Some(self.embedding_count.to_string()),
            // Skipping fingerprint in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a IngestResult value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for IngestResult {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub message_count: Vec<i32>,
            pub embedding_count: Vec<i32>,
            pub fingerprint: Vec<models::StyleFingerprint>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing IngestResult".to_string(),
                    );
                },
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "message_count" => intermediate_rep.message_count.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "embedding_count" => intermediate_rep.embedding_count.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "fingerprint" => intermediate_rep.fingerprint.push(<models::StyleFingerprint as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing IngestResult".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(IngestResult {
            message_count: intermediate_rep
                .message_count
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "message_count missing in IngestResult".to_string()
                })?,
            embedding_count: intermediate_rep
                .embedding_count
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "embedding_count missing in IngestResult".to_string()
                })?,
            fingerprint: intermediate_rep
                .fingerprint
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "fingerprint missing in IngestResult".to_string()
                })?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<IngestResult> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<IngestResult>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<IngestResult>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for IngestResult - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<IngestResult>
{
    type Error = String;

    fn try_from(
        hdr_value: HeaderValue,
    ) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <IngestResult as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    },
                    std::result::Result::Err(err) => {
                        std::result::Result::Err(format!(
                            r#"Unable to convert header value '{value}' into IngestResult - {err}"#
                        ))
                    },
                }
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MessageList {
    #[serde(rename = "messages")]
    #[validate(nested)]
    pub messages: Vec<models::StoredMessage>,

    #[serde(rename = "total")]
    pub total: i32,

    #[serde(rename = "limit")]
    pub limit: i32,

    #[serde(rename = "offset")]
    pub offset: i32,
}

impl MessageList {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        messages: Vec<models::StoredMessage>,
        total: i32,
        limit: i32,
        offset: i32,
    ) -> MessageList {
        MessageList { messages, total, limit, offset }
    }
}

/// Converts the MessageList value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for MessageList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping messages in query parameter serialization
            Some("total".to_string()),
            Some(self.total.to_string()),
            Some("limit".to_string()),
            Some(self.limit.to_string()),
            Some("offset".to_string()),
            Some(self.offset.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MessageList value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MessageList {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub messages: Vec<Vec<models::StoredMessage>>,
            pub total: Vec<i32>,
            pub limit: Vec<i32>,
            pub offset: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing MessageList".to_string(),
                    );
                },
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "messages" => return std::result::Result::Err("Parsing a container in this style is not supported in MessageList".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "total" => intermediate_rep.total.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "limit" => intermediate_rep.limit.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "offset" => intermediate_rep.offset.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MessageList".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MessageList {
            messages: intermediate_rep
                .messages
                .into_iter()
                .next()
                .ok_or_else(|| "messages missing in MessageList".to_string())?,
            total: intermediate_rep
                .total
                .into_iter()
                .next()
                .ok_or_else(|| "total missing in MessageList".to_string())?,
            limit: intermediate_rep
                .limit
                .into_iter()
                .next()
                .ok_or_else(|| "limit missing in MessageList".to_string())?,
            offset: intermediate_rep
                .offset
                .into_iter()
                .next()
                .ok_or_else(|| "offset missing in MessageList".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MessageList> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<MessageList>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<MessageList>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for MessageList - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<MessageList>
{
    type Error = String;

    fn try_from(
        hdr_value: HeaderValue,
    ) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <MessageList as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    },
                    std::result::Result::Err(err) => {
                        std::result::Result::Err(format!(
                            r#"Unable to convert header value '{value}' into MessageList - {err}"#
                        ))
                    },
                }
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RetrievedMessage {
    #[serde(rename = "text")]
    #[validate(custom(function = "check_xss_string"))]
    pub text: String,

    #[serde(rename = "audience")]
    #[validate(nested)]
    pub audience: models::Audience,

    #[serde(rename = "similarity_score")]
    #[validate(range(min = 0.0f32, max = 1.0f32))]
    pub similarity_score: f32,
}

impl RetrievedMessage {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        text: String,
        audience: models::Audience,
        similarity_score: f32,
    ) -> RetrievedMessage {
        RetrievedMessage { text, audience, similarity_score }
    }
}

/// Converts the RetrievedMessage value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for RetrievedMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("text".to_string()),
            Some(self.text.to_string()),
            // Skipping audience in query parameter serialization
            Some("similarity_score".to_string()),
            Some(self.similarity_score.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RetrievedMessage value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RetrievedMessage {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub text: Vec<String>,
            pub audience: Vec<models::Audience>,
            pub similarity_score: Vec<f32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing RetrievedMessage"
                            .to_string(),
                    );
                },
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "text" => intermediate_rep.text.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "audience" => intermediate_rep.audience.push(
                        <models::Audience as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "similarity_score" => {
                        intermediate_rep.similarity_score.push(
                            <f32 as std::str::FromStr>::from_str(val)
                                .map_err(|x| x.to_string())?,
                        )
                    },
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing RetrievedMessage"
                                .to_string(),
                        );
                    },
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RetrievedMessage {
            text: intermediate_rep.text.into_iter().next().ok_or_else(
                || "text missing in RetrievedMessage".to_string(),
            )?,
            audience: intermediate_rep.audience.into_iter().next().ok_or_else(
                || "audience missing in RetrievedMessage".to_string(),
            )?,
            similarity_score: intermediate_rep
                .similarity_score
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "similarity_score missing in RetrievedMessage".to_string()
                })?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RetrievedMessage> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<RetrievedMessage>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<RetrievedMessage>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for RetrievedMessage - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<RetrievedMessage>
{
    type Error = String;

    fn try_from(
        hdr_value: HeaderValue,
    ) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <RetrievedMessage as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    },
                    std::result::Result::Err(err) => {
                        std::result::Result::Err(format!(
                            r#"Unable to convert header value '{value}' into RetrievedMessage - {err}"#
                        ))
                    },
                }
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SimulationEntry {
    #[serde(rename = "simulation_id")]
    pub simulation_id: uuid::Uuid,

    #[serde(rename = "prompt")]
    #[validate(custom(function = "check_xss_string"))]
    pub prompt: String,

    #[serde(rename = "audience")]
    #[validate(nested)]
    pub audience: models::Audience,

    #[serde(rename = "nudge")]
    #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nudge: Option<String>,

    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "result")]
    #[validate(nested)]
    pub result: models::GenerateResponse,
}

impl SimulationEntry {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        simulation_id: uuid::Uuid,
        prompt: String,
        audience: models::Audience,
        created_at: chrono::DateTime<chrono::Utc>,
        result: models::GenerateResponse,
    ) -> SimulationEntry {
        SimulationEntry {
            simulation_id,
            prompt,
            audience,
            nudge: None,
            created_at,
            result,
        }
    }
}

/// Converts the SimulationEntry value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for SimulationEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping simulation_id in query parameter serialization
            Some("prompt".to_string()),
            Some(self.prompt.to_string()),
            // Skipping audience in query parameter serialization
            self.nudge.as_ref().map(|nudge| {
                ["nudge".to_string(), nudge.to_string()].join(",")
            }),
            // Skipping created_at in query parameter serialization

            // Skipping result in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SimulationEntry value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SimulationEntry {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub simulation_id: Vec<uuid::Uuid>,
            pub prompt: Vec<String>,
            pub audience: Vec<models::Audience>,
            pub nudge: Vec<String>,
            pub created_at: Vec<chrono::DateTime<chrono::Utc>>,
            pub result: Vec<models::GenerateResponse>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing SimulationEntry"
                            .to_string(),
                    );
                },
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "simulation_id" => intermediate_rep.simulation_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "prompt" => intermediate_rep.prompt.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "audience" => intermediate_rep.audience.push(<models::Audience as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "nudge" => intermediate_rep.nudge.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "created_at" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "result" => intermediate_rep.result.push(<models::GenerateResponse as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SimulationEntry".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SimulationEntry {
            simulation_id: intermediate_rep
                .simulation_id
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "simulation_id missing in SimulationEntry".to_string()
                })?,
            prompt: intermediate_rep.prompt.into_iter().next().ok_or_else(
                || "prompt missing in SimulationEntry".to_string(),
            )?,
            audience: intermediate_rep.audience.into_iter().next().ok_or_else(
                || "audience missing in SimulationEntry".to_string(),
            )?,
            nudge: intermediate_rep.nudge.into_iter().next(),
            created_at: intermediate_rep
                .created_at
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "created_at missing in SimulationEntry".to_string()
                })?,
            result: intermediate_rep.result.into_iter().next().ok_or_else(
                || "result missing in SimulationEntry".to_string(),
            )?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SimulationEntry> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<SimulationEntry>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<SimulationEntry>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for SimulationEntry - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<SimulationEntry>
{
    type Error = String;

    fn try_from(
        hdr_value: HeaderValue,
    ) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <SimulationEntry as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    },
                    std::result::Result::Err(err) => {
                        std::result::Result::Err(format!(
                            r#"Unable to convert header value '{value}' into SimulationEntry - {err}"#
                        ))
                    },
                }
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct StoredMessage {
    #[serde(rename = "message_id")]
    pub message_id: uuid::Uuid,

    #[serde(rename = "text")]
    #[validate(custom(function = "check_xss_string"))]
    pub text: String,

    #[serde(rename = "audience")]
    #[validate(nested)]
    pub audience: models::Audience,
}

impl StoredMessage {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        message_id: uuid::Uuid,
        text: String,
        audience: models::Audience,
    ) -> StoredMessage {
        StoredMessage { message_id, text, audience }
    }
}

/// Converts the StoredMessage value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for StoredMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping message_id in query parameter serialization
            Some("text".to_string()),
            Some(self.text.to_string()),
            // Skipping audience in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a StoredMessage value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for StoredMessage {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub message_id: Vec<uuid::Uuid>,
            pub text: Vec<String>,
            pub audience: Vec<models::Audience>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing StoredMessage".to_string(),
                    );
                },
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "message_id" => intermediate_rep.message_id.push(
                        <uuid::Uuid as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "text" => intermediate_rep.text.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "audience" => intermediate_rep.audience.push(
                        <models::Audience as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing StoredMessage"
                                .to_string(),
                        );
                    },
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(StoredMessage {
            message_id: intermediate_rep
                .message_id
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "message_id missing in StoredMessage".to_string()
                })?,
            text: intermediate_rep
                .text
                .into_iter()
                .next()
                .ok_or_else(|| "text missing in StoredMessage".to_string())?,
            audience: intermediate_rep.audience.into_iter().next().ok_or_else(
                || "audience missing in StoredMessage".to_string(),
            )?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<StoredMessage> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<StoredMessage>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<StoredMessage>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for StoredMessage - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<StoredMessage>
{
    type Error = String;

    fn try_from(
        hdr_value: HeaderValue,
    ) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <StoredMessage as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    },
                    std::result::Result::Err(err) => {
                        std::result::Result::Err(format!(
                            r#"Unable to convert header value '{value}' into StoredMessage - {err}"#
                        ))
                    },
                }
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct StyleFingerprint {
    /// 0 = very casual, 1 = very formal
    #[serde(rename = "formality_score")]
    #[validate(range(min = 0.0f32, max = 1.0f32))]
    pub formality_score: f32,

    /// Mean word count per sentence
    #[serde(rename = "avg_sentence_length")]
    pub avg_sentence_length: f32,

    #[serde(rename = "sentence_length_variance")]
    pub sentence_length_variance: f32,

    /// Exclamation marks per sentence
    #[serde(rename = "exclamation_ratio")]
    pub exclamation_ratio: f32,

    /// Ellipses per sentence
    #[serde(rename = "ellipsis_ratio")]
    pub ellipsis_ratio: f32,

    /// Emoji per message
    #[serde(rename = "emoji_frequency")]
    pub emoji_frequency: f32,

    /// Proportion of words that are contractions
    #[serde(rename = "contraction_ratio")]
    pub contraction_ratio: f32,

    /// Proportion of sentences containing hedging language (just, maybe, I think, kind of, etc.)
    #[serde(rename = "hedging_ratio")]
    pub hedging_ratio: f32,

    /// Most frequent message-opening phrases
    #[serde(rename = "common_openers")]
    #[validate(custom(function = "check_xss_vec_string"))]
    pub common_openers: Vec<String>,

    /// Most frequent sign-off phrases
    #[serde(rename = "common_closers")]
    #[validate(custom(function = "check_xss_vec_string"))]
    pub common_closers: Vec<String>,

    /// Number of messages this fingerprint was derived from
    #[serde(rename = "message_count")]
    pub message_count: i32,
}

impl StyleFingerprint {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        formality_score: f32,
        avg_sentence_length: f32,
        sentence_length_variance: f32,
        exclamation_ratio: f32,
        ellipsis_ratio: f32,
        emoji_frequency: f32,
        contraction_ratio: f32,
        hedging_ratio: f32,
        common_openers: Vec<String>,
        common_closers: Vec<String>,
        message_count: i32,
    ) -> StyleFingerprint {
        StyleFingerprint {
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
        }
    }
}

/// Converts the StyleFingerprint value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for StyleFingerprint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("formality_score".to_string()),
            Some(self.formality_score.to_string()),
            Some("avg_sentence_length".to_string()),
            Some(self.avg_sentence_length.to_string()),
            Some("sentence_length_variance".to_string()),
            Some(self.sentence_length_variance.to_string()),
            Some("exclamation_ratio".to_string()),
            Some(self.exclamation_ratio.to_string()),
            Some("ellipsis_ratio".to_string()),
            Some(self.ellipsis_ratio.to_string()),
            Some("emoji_frequency".to_string()),
            Some(self.emoji_frequency.to_string()),
            Some("contraction_ratio".to_string()),
            Some(self.contraction_ratio.to_string()),
            Some("hedging_ratio".to_string()),
            Some(self.hedging_ratio.to_string()),
            Some("common_openers".to_string()),
            Some(
                self.common_openers
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
            Some("common_closers".to_string()),
            Some(
                self.common_closers
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
            Some("message_count".to_string()),
            Some(self.message_count.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a StyleFingerprint value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for StyleFingerprint {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub formality_score: Vec<f32>,
            pub avg_sentence_length: Vec<f32>,
            pub sentence_length_variance: Vec<f32>,
            pub exclamation_ratio: Vec<f32>,
            pub ellipsis_ratio: Vec<f32>,
            pub emoji_frequency: Vec<f32>,
            pub contraction_ratio: Vec<f32>,
            pub hedging_ratio: Vec<f32>,
            pub common_openers: Vec<Vec<String>>,
            pub common_closers: Vec<Vec<String>>,
            pub message_count: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing StyleFingerprint"
                            .to_string(),
                    );
                },
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "formality_score" => intermediate_rep.formality_score.push(<f32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "avg_sentence_length" => intermediate_rep.avg_sentence_length.push(<f32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "sentence_length_variance" => intermediate_rep.sentence_length_variance.push(<f32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "exclamation_ratio" => intermediate_rep.exclamation_ratio.push(<f32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ellipsis_ratio" => intermediate_rep.ellipsis_ratio.push(<f32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "emoji_frequency" => intermediate_rep.emoji_frequency.push(<f32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "contraction_ratio" => intermediate_rep.contraction_ratio.push(<f32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "hedging_ratio" => intermediate_rep.hedging_ratio.push(<f32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "common_openers" => return std::result::Result::Err("Parsing a container in this style is not supported in StyleFingerprint".to_string()),
                    "common_closers" => return std::result::Result::Err("Parsing a container in this style is not supported in StyleFingerprint".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "message_count" => intermediate_rep.message_count.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing StyleFingerprint".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(StyleFingerprint {
            formality_score: intermediate_rep
                .formality_score
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "formality_score missing in StyleFingerprint".to_string()
                })?,
            avg_sentence_length: intermediate_rep
                .avg_sentence_length
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "avg_sentence_length missing in StyleFingerprint"
                        .to_string()
                })?,
            sentence_length_variance: intermediate_rep
                .sentence_length_variance
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "sentence_length_variance missing in StyleFingerprint"
                        .to_string()
                })?,
            exclamation_ratio: intermediate_rep
                .exclamation_ratio
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "exclamation_ratio missing in StyleFingerprint".to_string()
                })?,
            ellipsis_ratio: intermediate_rep
                .ellipsis_ratio
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "ellipsis_ratio missing in StyleFingerprint".to_string()
                })?,
            emoji_frequency: intermediate_rep
                .emoji_frequency
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "emoji_frequency missing in StyleFingerprint".to_string()
                })?,
            contraction_ratio: intermediate_rep
                .contraction_ratio
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "contraction_ratio missing in StyleFingerprint".to_string()
                })?,
            hedging_ratio: intermediate_rep
                .hedging_ratio
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "hedging_ratio missing in StyleFingerprint".to_string()
                })?,
            common_openers: intermediate_rep
                .common_openers
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "common_openers missing in StyleFingerprint".to_string()
                })?,
            common_closers: intermediate_rep
                .common_closers
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "common_closers missing in StyleFingerprint".to_string()
                })?,
            message_count: intermediate_rep
                .message_count
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "message_count missing in StyleFingerprint".to_string()
                })?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<StyleFingerprint> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<StyleFingerprint>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<StyleFingerprint>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for StyleFingerprint - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<StyleFingerprint>
{
    type Error = String;

    fn try_from(
        hdr_value: HeaderValue,
    ) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <StyleFingerprint as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    },
                    std::result::Result::Err(err) => {
                        std::result::Result::Err(format!(
                            r#"Unable to convert header value '{value}' into StyleFingerprint - {err}"#
                        ))
                    },
                }
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}
