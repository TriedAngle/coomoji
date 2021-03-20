use crate::model::{Game, NewGame};
use anyhow::Result;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;
use uuid::Uuid;

impl Game {
    pub async fn all(pool: &PgPool) -> Result<Vec<Self>> {
        let mut items = Vec::new();

        let recs = sqlx::query!(
            r#"
                SELECT id, channel_id, players, player_inventories, inventory
                    FROM games
                ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await?;

        for rec in recs {
            items.push(Self {
                id: rec.id,
                channel_id: rec.channel_id,
                players: rec.players,
                player_inventories: rec.player_inventories,
                inventory: rec.inventory,
            });
        }

        Ok(items)
    }

    pub async fn by_id(id: Uuid, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM games WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: rec.id,
            channel_id: rec.channel_id,
            players: rec.players,
            player_inventories: rec.player_inventories,
            inventory: rec.inventory,
        })
    }

    pub async fn by_channel_id(channel_id: String, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM games WHERE channel_id = $1
            "#,
            channel_id
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: rec.id,
            channel_id: rec.channel_id,
            players: rec.players,
            player_inventories: rec.player_inventories,
            inventory: rec.inventory,
        })
    }

    pub async fn create(game: NewGame, pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let players: Vec<i32> = vec![];
        let player_inventories: Vec<i32> = vec![];
        let created = sqlx::query(
            r#"
                INSERT INTO games (id, channel_id, players, player_inventories, inventory) VALUES ($1, $2, $3, $4, $5)
                RETURNING id, discord_id
            "#,
        )
            .bind(Uuid::new_v4())
            .bind(&game.channel_id)
            .bind(&players)
            .bind(&player_inventories)
            .bind(&game.inventory)
            .map(|row: PgRow| Self {
                id: row.get(0),
                channel_id: row.get(1),
                players: row.get(2),
                player_inventories: row.get(3),
                inventory: row.get(4)
            })
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(created)
    }

    pub async fn update_inventory(id: Uuid, inventory: &[i32], pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let updated = sqlx::query(
            r#"
                UPDATE games SET inventory = $1
                WHERE id = $2
                RETURNING id, channel_id, players, player_inventories, inventory
            "#,
        )
        .bind(&inventory)
        .bind(id)
        .map(|row: PgRow| Self {
            id: row.get(0),
            channel_id: row.get(1),
            players: row.get(2),
            player_inventories: row.get(3),
            inventory: row.get(4),
        })
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(updated)
    }

    pub async fn update_players(
        id: Uuid,
        players: &[i32],
        player_inventories: &[Uuid],
        pool: &PgPool,
    ) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let updated = sqlx::query(
            r#"
                UPDATE games SET players = $1, player_inventories = $2,
                WHERE id = $3
                RETURNING id, channel_id, players, player_inventories, inventory
            "#,
        )
        .bind(&players)
        .bind(&player_inventories)
        .bind(id)
        .map(|row: PgRow| Self {
            id: row.get(0),
            channel_id: row.get(1),
            players: row.get(2),
            player_inventories: row.get(3),
            inventory: row.get(4),
        })
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(updated)
    }

    pub async fn delete(id: Uuid, pool: &PgPool) -> Result<bool> {
        let mut tx = pool.begin().await?;
        sqlx::query(
            r#"
                DELETE FROM games
                WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(true)
    }
}
