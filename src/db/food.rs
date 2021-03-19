use crate::model::{Food, NewFood};
use anyhow::Result;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;

impl Food {
    pub async fn all(pool: &PgPool) -> Result<Vec<Self>> {
        let mut items = Vec::new();

        let recs = sqlx::query!(
            r#"
                SELECT id, name, emoji, description
                    FROM foods
                ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await?;

        for rec in recs {
            items.push(Self {
                id: rec.id,
                name: rec.name,
                description: rec.description,
                emoji: rec.emoji,
            });
        }

        Ok(items)
    }

    pub async fn by_id(id: i32, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM foods WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: rec.id,
            name: rec.name,
            emoji: rec.emoji,
            description: rec.description,
        })
    }

    pub async fn by_name(name: String, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM foods WHERE name = $1
            "#,
            name
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: rec.id,
            name: rec.name,
            emoji: rec.emoji,
            description: rec.description,
        })
    }

    pub async fn create(item: NewFood, pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let created = sqlx::query(
            r#"
                INSERT INTO foods (name, emoji, description) VALUES ($1, $2, $3)
                RETURNING id, name, emoji, description
            "#,
        )
        .bind(&item.name)
        .bind(&item.emoji)
        .bind(&item.description)
        .map(|row: PgRow| Self {
            id: row.get(0),
            name: row.get(1),
            emoji: row.get(2),
            description: row.get(3),
        })
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(created)
    }

    pub async fn update(id: i32, item: NewFood, pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let updated = sqlx::query(
            r#"
                UPDATE foods SET name = $1, emoji = $2, description = $3
                WHERE id = $4
                RETURNING id, name, emoji, description
            "#,
        )
        .bind(&item.name)
        .bind(&item.emoji)
        .bind(&item.description)
        .bind(id)
        .map(|row: PgRow| Self {
            id: row.get(0),
            name: row.get(1),
            emoji: row.get(2),
            description: row.get(3),
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
                DELETE FROM foods
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
