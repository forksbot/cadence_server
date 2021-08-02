use chrono::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// `Workspace` represents a database record
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub description: String,
    pub status: Status,
    pub created_at: DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Status {
    Pending,
    Doing,
    Complete,
}

#[derive(Clone, Debug, Serialize)]
pub struct ProgressReport {
    pub pending: i64,
    pub doing: i64,
    pub complete: i64,
}
