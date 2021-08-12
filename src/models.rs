use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// `User` is a user of the application.
#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}

/// `Board` is the current workspace for a `User`.
#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

/// `List` is a container for `Ticket` structures.
#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct List {
    pub id: i64,
    pub list_id: i64,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// `Ticket` holds data pertaining to a "task", "issue", "job", etc.
#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Ticket {
    pub id: i64,
    pub list_id: i64,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
