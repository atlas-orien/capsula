use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use super::{Owner, SemanticContent, SemanticContext, SemanticSource, SemanticType};
use crate::{CoreError, Result};

/// Minimal AI-native text semantic Capsule IR.
///
/// This is the new core abstraction described in
/// `arthitectures/00-semantic-ir-foundation.md`. It contains semantic data for
/// agent cognition and intentionally excludes behavior, encryption, policy,
/// keyrings, sealed blocks, database concerns, and platform schemas.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticCapsule {
    pub id: String,
    pub owner: Owner,
    pub semantic_type: SemanticType,
    pub content: SemanticContent,
    #[serde(default)]
    pub context: SemanticContext,
    #[serde(default)]
    pub source: SemanticSource,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

impl SemanticCapsule {
    pub fn new(
        owner: Owner,
        semantic_type: SemanticType,
        content: SemanticContent,
    ) -> Result<Self> {
        Self::with_id(
            generate_semantic_capsule_id(),
            owner,
            semantic_type,
            content,
        )
    }

    pub fn with_id(
        id: impl Into<String>,
        owner: Owner,
        semantic_type: SemanticType,
        content: SemanticContent,
    ) -> Result<Self> {
        let id = id.into();
        if id.trim().is_empty() {
            return Err(CoreError::FormatError(
                "semantic capsule id must not be empty".to_string(),
            ));
        }

        Ok(Self {
            id,
            owner,
            semantic_type,
            content,
            context: SemanticContext::default(),
            source: SemanticSource::default(),
            created_at: OffsetDateTime::now_utc(),
        })
    }

    pub fn with_context(mut self, context: SemanticContext) -> Self {
        self.context = context;
        self
    }

    pub fn with_source(mut self, source: SemanticSource) -> Self {
        self.source = source;
        self
    }
}

fn generate_semantic_capsule_id() -> String {
    format!("scid:{}", Uuid::new_v4())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic::SemanticActor;

    #[test]
    fn semantic_capsule_round_trips_json() {
        let capsule = SemanticCapsule::with_id(
            "scid:test-1",
            Owner::new("user:alice").unwrap().with_display_name("Alice"),
            SemanticType::new("conversation.message").unwrap(),
            SemanticContent::new("hello from a source platform")
                .unwrap()
                .with_language("en"),
        )
        .unwrap()
        .with_context(
            SemanticContext::new()
                .with_actor(SemanticActor::named("Alice").with_role("sender"))
                .with_topic("greeting"),
        )
        .with_source(
            SemanticSource::new()
                .with_platform("slack")
                .with_source_type("message")
                .with_source_id("m-1"),
        );

        let json = serde_json::to_string(&capsule).unwrap();
        assert!(!json.contains("policy"));
        assert!(!json.contains("keyring"));
        assert!(!json.contains("sealed"));

        let decoded: SemanticCapsule = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.id, "scid:test-1");
        assert_eq!(decoded.semantic_type.as_str(), "conversation.message");
        assert_eq!(decoded.content.text, "hello from a source platform");
        assert_eq!(decoded.source.platform.as_deref(), Some("slack"));
    }
}
