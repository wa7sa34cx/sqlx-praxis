// use serde::{Serialize, Deserialize};
// use derive_more::{Add, Display, From, Into};
// use derive_more::{From, Into};

// #[derive(Serialize, Deserialize, Debug)]
// #[derive(From, Into, Debug)]
#[derive(sqlx::FromRow, Debug)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub date: String,
    pub published: bool,
}