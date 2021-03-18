use crate::model::{NewRecipe, Recipe};
use anyhow::Result;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;

impl Recipe {
    pub async fn all(pool: &PgPool) -> Result<Vec<Self>> {
        let mut items = Vec::new();

        let recs = sqlx::query!(
            r#"
                SELECT id, operation, outcome, components
                    FROM recipes
                ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await?;

        for rec in recs {
            items.push(Self {
                id: rec.id,
                operation: rec.operation,
                outcome: rec.outcome,
                components: rec.components,
            });
        }

        Ok(items)
    }

    pub async fn by_id(id: i32, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM recipes WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: rec.id,
            operation: rec.operation,
            outcome: rec.outcome,
            components: rec.components,
        })
    }

    pub async fn by_operation(operation: i32, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM recipes WHERE operation = $1
            "#,
            operation
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: rec.id,
            operation: rec.operation,
            outcome: rec.outcome,
            components: rec.components,
        })
    }

    pub async fn by_outcome(outcome: i32, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM recipes WHERE outcome = $1
            "#,
            outcome
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: rec.id,
            operation: rec.operation,
            outcome: rec.outcome,
            components: rec.components,
        })
    }

    pub async fn by_components(components: &[i32], pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM recipes WHERE components @> $1
            "#,
            components
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: rec.id,
            operation: rec.operation,
            outcome: rec.outcome,
            components: rec.components,
        })
    }

    pub async fn create(item: NewRecipe, pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let created = sqlx::query(
            r#"
                INSERT INTO recipes (operation, outcome, components) VALUES ($1, $2, $3)
                RETURNING id, name, description, emoji
            "#,
        )
        .bind(&item.operation)
        .bind(&item.outcome)
        .bind(&item.components)
        .map(|row: PgRow| Self {
            id: row.get(0),
            operation: row.get(1),
            outcome: row.get(2),
            components: row.get(3),
        })
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(created)
    }

    pub async fn update(id: i32, item: NewRecipe, pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let updated = sqlx::query(
            r#"
                UPDATE recipes SET operation = $1, outcome = $2, components = $3
                WHERE id = $4
                RETURNING id, operation, outcome, components
            "#,
        )
        .bind(&item.operation)
        .bind(&item.outcome)
        .bind(&item.components)
        .bind(id)
        .map(|row: PgRow| Self {
            id: row.get(0),
            operation: row.get(1),
            outcome: row.get(2),
            components: row.get(3),
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
                DELETE FROM recipes
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
