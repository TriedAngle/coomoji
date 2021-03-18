use crate::model::{Operation, NewOperation};
use anyhow::Result;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;

impl Operation {
    pub async fn all(pool: &PgPool) -> Result<Vec<Self>> {
        let mut items = Vec::new();

        let recs = sqlx::query!(
            r#"
                SELECT id, emoji
                    FROM operations
                ORDER BY id
            "#
        )
            .fetch_all(pool)
            .await?;

        for rec in recs {
            items.push(Self {
                id: rec.id,
                emoji: rec.emoji,
            });
        }

        Ok(items)
    }

    pub async fn by_id(id: i32, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM operations WHERE id = $1
            "#,
            id
        )
            .fetch_one(pool)
            .await?;

        Ok(Self {
            id: rec.id,
            emoji: rec.emoji,
        })
    }

    pub async fn by_emoji(emoji: i32, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM operations WHERE emoji = $1
            "#, emoji
        )
            .fetch_one(pool)
            .await?;

        Ok(Self {
            id: rec.id,
            emoji: rec.emoji,
        })
    }

    pub async fn create(item: NewOperation, pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let created = sqlx::query(
            r#"
                INSERT INTO operations (emoji) VALUES ($1)
                RETURNING id, emoji
            "#,
        )
            .bind(&item.emoji)
            .map(|row: PgRow| Self {
                id: row.get(0),
                emoji: row.get(1),
            })
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(created)
    }

    pub async fn update(id: i32, item: NewOperation, pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let updated = sqlx::query(
            r#"
                UPDATE operations SET emoji = $1
                WHERE id = $2
                RETURNING id, emoji
            "#,
        )
            .bind(&item.emoji)
            .bind(id)
            .map(|row: PgRow| Self {
                id: row.get(0),
                emoji: row.get(1),
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
                DELETE FROM operations
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
