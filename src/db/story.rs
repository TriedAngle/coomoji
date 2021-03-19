use crate::model::{NewStory, Story};
use anyhow::Result;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;
use uuid::Uuid;

impl Story {
    pub async fn all(pool: &PgPool) -> Result<Vec<Self>> {
        let mut items = Vec::new();

        let recs = sqlx::query!(
            r#"
                SELECT id, title, description, story, items, recipes
                    FROM stories
                ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await?;

        for rec in recs {
            items.push(Self {
                id: rec.id,
                title: rec.title,
                description: rec.description,
                story: rec.story,
                items: rec.items,
                recipes: rec.recipes,
            });
        }

        Ok(items)
    }

    pub async fn by_id(id: i32, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM stories WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: rec.id,
            title: rec.title,
            description: rec.description,
            story: rec.story,
            items: rec.items,
            recipes: rec.recipes,
        })
    }

    pub async fn by_title(title: String, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM stories WHERE title = $1
            "#,
            title
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: rec.id,
            title: rec.title,
            description: rec.description,
            story: rec.story,
            items: rec.items,
            recipes: rec.recipes,
        })
    }

    pub async fn create(story: NewStory, pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let created = sqlx::query(
            r#"
                INSERT INTO stories (title, description, story, items, recipes) VALUES ($1, $2, $3, $4, $5)
                RETURNING id, title, description, items, recipes
            "#,
        )
            .bind(story.title)
            .bind(story.description)
            .bind(story.story)
            .bind(story.items)
            .bind(story.recipes)
            .map(|row: PgRow| Self {
                id: row.get(0),
                title: row.get(1),
                description: row.get(2),
                story: row.get(3),
                items: row.get(4),
                recipes: row.get(5)
            })
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(created)
    }

    pub async fn update(id: i32, story: NewStory, pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let updated = sqlx::query(
            r#"
                UPDATE stories SET title = $1, description = $2, story = $3 items = $4, recipes = $5
                WHERE id = $6
                RETURNING id, channel_id, players, player_inventories, inventory
            "#,
        )
        .bind(story.title)
        .bind(story.description)
        .bind(story.story)
        .bind(story.items)
        .bind(story.recipes)
        .bind(id)
        .map(|row: PgRow| Self {
            id: row.get(0),
            title: row.get(1),
            description: row.get(2),
            story: row.get(3),
            items: row.get(4),
            recipes: row.get(5),
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
                DELETE FROM stories
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
