use serde::{Deserialize, Serialize};

use crate::{CoreError, Result};

/// User or subject that owns the semantic data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Owner {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl Owner {
    pub fn new(id: impl Into<String>) -> Result<Self> {
        let id = id.into();
        if id.trim().is_empty() {
            return Err(CoreError::FormatError(
                "owner id must not be empty".to_string(),
            ));
        }

        Ok(Self {
            id,
            display_name: None,
        })
    }

    pub fn with_display_name(mut self, display_name: impl Into<String>) -> Self {
        self.display_name = Some(display_name.into());
        self
    }
}
