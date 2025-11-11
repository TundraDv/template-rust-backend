use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[schema(example = "2025-11-11T12:30:00-03:00")]
pub struct DateTimeString(pub DateTime<FixedOffset>);
