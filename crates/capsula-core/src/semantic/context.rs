use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// Actor participating in the semantic context.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticActor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

impl SemanticActor {
    pub fn named(name: impl Into<String>) -> Self {
        Self {
            id: None,
            name: Some(name.into()),
            role: None,
        }
    }

    pub fn with_role(mut self, role: impl Into<String>) -> Self {
        self.role = Some(role.into());
        self
    }
}

/// Semantic environment needed to understand the content.
///
/// `attributes` is deliberately loose so early lowering experiments can keep
/// useful context without freezing a full ontology too early.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SemanticContext {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actors: Vec<SemanticActor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "time::serde::rfc3339::option")]
    pub event_time: Option<OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub attributes: BTreeMap<String, serde_json::Value>,
}

impl SemanticContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_actor(mut self, actor: SemanticActor) -> Self {
        self.actors.push(actor);
        self
    }

    pub fn with_topic(mut self, topic: impl Into<String>) -> Self {
        self.topic = Some(topic.into());
        self
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
}
