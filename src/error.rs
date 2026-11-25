use serde::{Serialize, Serializer, ser::SerializeStruct as _};

/// Why a command failed.
///
/// JavaScript receives it as `{ kind, message }`, where `kind` is the
/// variant's name in camelCase and `message` is its [`Display`](core::fmt::Display) text.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The query lists no family.
    #[error("families must name at least one family")]
    NoFamilies,
    /// The wanted weight is outside the CSS range of 1 to 1000.
    #[error("weight must be between 1 and 1000, got {0}")]
    WeightOutOfRange(u16),
    /// No face has any family the query lists; they are held here in the
    /// query's order.
    #[error("no face matches {}", .0.join(", "))]
    NoMatch(Vec<String>),
    /// The plugin does not support this platform (Android and iOS).
    #[error("tauri-plugin-fontgres does not support this platform")]
    UnsupportedPlatform,
}

impl Error {
    /// The `kind` JavaScript receives.
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::NoFamilies => "noFamilies",
            Self::WeightOutOfRange(_) => "weightOutOfRange",
            Self::NoMatch(_) => "noMatch",
            Self::UnsupportedPlatform => "unsupportedPlatform",
        }
    }
}

impl Serialize for Error {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut error = serializer.serialize_struct("Error", 2)?;
        error.serialize_field("kind", self.kind())?;
        error.serialize_field("message", &self.to_string())?;
        error.end()
    }
}

#[cfg(desktop)]
impl From<fontgres_core::models::QueryError> for Error {
    fn from(error: fontgres_core::models::QueryError) -> Self {
        use fontgres_core::models::QueryError;
        match error {
            QueryError::NoFamilies => Self::NoFamilies,
            QueryError::WeightOutOfRange(weight) => Self::WeightOutOfRange(weight),
            QueryError::NoMatch(families) => Self::NoMatch(families),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serializes_as_kind_and_message() {
        assert_eq!(
            serde_json::to_value(Error::NoMatch(vec!["A".into(), "B".into()])).unwrap(),
            json!({ "kind": "noMatch", "message": "no face matches A, B" })
        );
        assert_eq!(
            serde_json::to_value(Error::WeightOutOfRange(0)).unwrap(),
            json!({ "kind": "weightOutOfRange", "message": "weight must be between 1 and 1000, got 0" })
        );
        assert_eq!(
            serde_json::to_value(Error::NoFamilies).unwrap()["kind"],
            "noFamilies"
        );
        assert_eq!(
            serde_json::to_value(Error::UnsupportedPlatform).unwrap()["kind"],
            "unsupportedPlatform"
        );
    }
}
