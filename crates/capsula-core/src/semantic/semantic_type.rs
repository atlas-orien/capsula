use std::{fmt, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize};

use crate::{CoreError, Result};

/// Namespace-based semantic type, such as `conversation.message`.
///
/// This is the main AI-facing type entry point. It is not a database type,
/// MIME type, platform schema name, or workflow action.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct SemanticType(String);

impl SemanticType {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        Self::validate(&value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn validate(value: &str) -> Result<()> {
        if value.is_empty() {
            return Err(CoreError::FormatError(
                "semantic type must not be empty".to_string(),
            ));
        }

        if value.trim() != value {
            return Err(CoreError::FormatError(
                "semantic type must not contain leading or trailing whitespace".to_string(),
            ));
        }

        if !value.contains('.') {
            return Err(CoreError::FormatError(
                "semantic type must contain a namespace separator '.'".to_string(),
            ));
        }

        if value.starts_with('.') || value.ends_with('.') || value.contains("..") {
            return Err(CoreError::FormatError(
                "semantic type must contain non-empty namespace segments".to_string(),
            ));
        }

        let valid_chars = value.chars().all(|c| {
            c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.' || c == '_' || c == '-'
        });

        if !valid_chars {
            return Err(CoreError::FormatError(
                "semantic type may only contain lowercase ascii letters, digits, '.', '_' and '-'"
                    .to_string(),
            ));
        }

        Ok(())
    }
}

impl fmt::Display for SemanticType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for SemanticType {
    type Err = CoreError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<String> for SemanticType {
    type Error = CoreError;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<SemanticType> for String {
    fn from(value: SemanticType) -> Self {
        value.0
    }
}

impl<'de> Deserialize<'de> for SemanticType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_namespaced_semantic_type() {
        let semantic_type = SemanticType::new("conversation.message").unwrap();
        assert_eq!(semantic_type.as_str(), "conversation.message");
    }

    #[test]
    fn rejects_empty_semantic_type() {
        assert!(SemanticType::new("").is_err());
    }

    #[test]
    fn rejects_type_without_namespace() {
        assert!(SemanticType::new("message").is_err());
    }

    #[test]
    fn rejects_platform_style_uppercase_type() {
        assert!(SemanticType::new("Slack.Message").is_err());
    }
}
