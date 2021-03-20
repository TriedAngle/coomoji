use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone)]
pub struct Emoji {
    pub id: i32,
    pub name: String,
    pub utf8: String,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct NewEmoji {
    pub name: String,
    pub utf8: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Operation {
    pub id: i32,
    pub emoji: i32,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct NewOperation {
    pub emoji: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Food {
    pub id: i32,
    pub name: String,
    pub emoji: i32,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct NewFood {
    pub name: String,
    pub emoji: i32,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Recipe {
    pub id: i32,
    pub operation: i32,
    pub outcome: i32,
    pub components: Vec<i32>,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct NewRecipe {
    pub operation: i32,
    pub outcome: i32,
    pub components: Vec<i32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: i32,
    pub discord_id: String,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct NewPlayer {
    pub discord_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub id: Uuid,
    pub channel_id: String,
    pub players: Vec<i32>,
    pub player_inventories: Vec<Uuid>,
    pub inventory: Vec<i32>,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct NewGame {
    pub channel_id: String,
    pub inventory: Vec<i32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Story {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub story: String,
    pub items: Vec<i32>,
    pub recipes: Vec<i32>,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct NewStory {
    pub title: String,
    pub description: String,
    pub story: String,
    pub items: Vec<i32>,
    pub recipes: Vec<i32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerInventory {
    pub id: Uuid,
    pub player: i32,
    pub game: Uuid,
    pub inventory: Vec<i32>,
}
