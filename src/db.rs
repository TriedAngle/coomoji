use anyhow::Result;
use crate::model::Emoji;
use sqlx::postgres::PgPool;

impl Emoji {
    pub async fn all(pool: &PgPool) -> Result<Vec<Emoji>> {
        let mut emoji = Vec::new();

        let recs = sqlx::query!(
            r#"
                SELECT id, name
                    FROM emojis
                ORDER BY id
            "#
        )
            .fetch_all(pool)
            .await?;

        for rec in recs {
            emoji.push( Emoji {
                id: rec.id,
                name: rec.name,
            });
        }

        Ok(emoji)
    }

    pub async fn by_id(id: i32, pool: &PgPool) -> Result<Emoji> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM emojis WHERE id = $1
            "#,
             id
        ).fetch_one(pool)
            .await?;

        Ok(Emoji {
            id: rec.id,
            name: rec.name,
        })
    }

    pub async fn by_name(name: String, pool: &PgPool) -> Result<Emoji> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM emojis WHERE name = $1
            "#,
        ).fetch_one(pool)
            .await?;

        Ok(Emoji {
            id: rec.id,
            name: rec.name,
        })
    }
}