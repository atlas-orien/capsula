//! Semantic IR core for AI-native Data Capsules.
//!
//! This module is the new semantic foundation of Capsula. It intentionally does
//! not depend on the legacy secure container layer (`capsule/`, `block/`,
//! `keyring/`, `policy/`). Security and storage can wrap this IR later.

pub mod capsule;
pub mod content;
pub mod context;
pub mod owner;
pub mod semantic_type;
pub mod source;

pub use capsule::SemanticCapsule;
pub use content::SemanticContent;
pub use context::{SemanticActor, SemanticContext};
pub use owner::Owner;
pub use semantic_type::SemanticType;
pub use source::SemanticSource;
