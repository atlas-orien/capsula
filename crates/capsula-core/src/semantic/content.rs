use serde::{Deserialize, Serialize};

use crate::{CoreError, Result};

/// Text-first semantic content.
///
/// This intentionally excludes binary payloads, embeddings, rendering state,
/// workflow data, and platform-specific schema fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticContent {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

impl SemanticContent {
    pub fn new(text: impl Into<String>) -> Result<Self> {
        let text = text.into();
        if text.trim().is_empty() {
            return Err(CoreError::FormatError(
                "semantic content text must not be empty".to_string(),
            ));
        }

        Ok(Self {
            text,
            title: None,
            language: None,
            summary: None,
        })
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }
}
