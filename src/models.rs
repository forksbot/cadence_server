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

/// `Task` represents a database record
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub description: String,
    pub status: Status,
    pub created_at: DateTime<chrono::Utc>,
}

/// `Status` is used to track the current state of a `Task`
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Status {
    Pending,
    Doing,
    Complete,
}

// TODO: Flesh out how `ProgressReport` is going to work.
//  The general idea is to assist with retrieving the current state of a
//  user's tasks across all workspaces.

#[derive(Clone, Debug, Serialize)]
pub struct ProgressReport {
    pub pending: i64,
    pub doing: i64,
    pub complete: i64,
}
