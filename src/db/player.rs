use crate::model::{NewPlayer, Player};
use anyhow::Result;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;

impl Player {
    pub async fn all(pool: &PgPool) -> Result<Vec<Self>> {
        let mut items = Vec::new();

        let recs = sqlx::query!(
            r#"
                SELECT id, discord_id
                    FROM players
                ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await?;

        for rec in recs {
            items.push(Self {
                id: rec.id,
                discord_id: rec.discord_id,
            });
        }

        Ok(items)
    }

    pub async fn by_id(id: i32, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM players WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: rec.id,
            discord_id: rec.discord_id,
        })
    }

    pub async fn by_discord_id(discord_id: String, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM players WHERE discord_id = $1
            "#,
            discord_id
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: rec.id,
            discord_id: rec.discord_id,
        })
    }

    pub async fn create(item: NewPlayer, pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let created = sqlx::query(
            r#"
                INSERT INTO players (discord_id) VALUES ($1)
                RETURNING id, discord_id
            "#,
        )
        .bind(&item.discord_id)
        .map(|row: PgRow| Self {
            id: row.get(0),
            discord_id: row.get(1),
        })
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(created)
    }

    pub async fn update(id: i32, item: NewPlayer, pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let updated = sqlx::query(
            r#"
                UPDATE foods SET discord_id = $1
                WHERE id = $2
                RETURNING id, discord_id
            "#,
        )
        .bind(&item.discord_id)
        .bind(id)
        .map(|row: PgRow| Self {
            id: row.get(0),
            discord_id: row.get(1),
        })
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(updated)
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<bool> {
        let mut tx = pool.begin().await?;
        sqlx::query(
            r#"
                DELETE FROM players
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
