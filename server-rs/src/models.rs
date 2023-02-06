use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub(crate) struct User {
    pub user_id: String,
    pub name: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub(crate) struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub user_id: i32,
    pub estimated_time: i32,
    pub actual_time: i32,
    pub status: String,
    pub expected_completion_date: String,

}
