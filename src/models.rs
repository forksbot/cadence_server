use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// `Board` model (database record)
#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}

/// `Board` model (database record)
#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

/// The `List` model represents a list, or column, on a `Board`.
/// The `List` is responsible for holding `Ticket` structures.
#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct List {
    pub id: i64,
    pub list_id: i64,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A `Ticket` is a database entity that holds the data pertaining
/// to a particular 'task', 'issue' or 'job'.
#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Ticket {
    pub id: i64,
    pub list_id: i64,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
