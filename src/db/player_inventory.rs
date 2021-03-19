use crate::model::PlayerInventory;
use anyhow::Result;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;
use uuid::Uuid;

impl PlayerInventory {
    pub async fn by_game_and_player(game_id: Uuid, player_id: i32, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM player_inventories WHERE player = $2 AND game = $1
            "#,
            game_id,
            player_id
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: rec.id,
            player: rec.player,
            game: rec.game,
            inventory: rec.inventory,
        })
    }

    pub async fn create(game_id: Uuid, player_id: i32, pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let inventory = Vec::<i32>::new();
        let created = sqlx::query(
            r#"
                INSERT INTO player_inventories (id, player, game, inventory) VALUES ($1, $2, $3, $4)
                RETURNING id, discord_id
            "#,
        )
        .bind(&Uuid::new_v4())
        .bind(&player_id)
        .bind(&game_id)
        .bind(&inventory)
        .map(|row: PgRow| Self {
            id: row.get(0),
            player: row.get(1),
            game: row.get(2),
            inventory: row.get(3),
        })
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(created)
    }

    pub async fn update_inventory(id: Uuid, items: &[i32], pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let updated = sqlx::query(
            r#"
                UPDATE player_inventories SET inventory = $1
                WHERE id = $2
                RETURNING id, discord_id
            "#,
        )
        .bind(&items)
        .bind(id)
        .map(|row: PgRow| Self {
            id: row.get(0),
            player: row.get(1),
            game: row.get(2),
            inventory: row.get(3),
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
                DELETE FROM player_inventories
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
