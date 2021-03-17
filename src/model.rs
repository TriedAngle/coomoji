use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Emoji {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct NewEmoji {
    pub name: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Operation {
    pub id: i32,
    pub emoji: i32
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct NewOperation {
    pub emoji: i32
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Food {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub emoji: i32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct NewFood {
    pub name: String,
    pub description: Option<String>,
    pub emoji: i32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Recipe {
    pub id: i32,
    pub operation: i32,
    pub outcome: i32,
    pub components: Vec<i32>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct NewRecipe {
    pub id: i32,
    pub operation: i32,
    pub outcome: i32,
    pub components: Vec<i32>,
}