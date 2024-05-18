use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Member {
    pub id: i32,
    pub team_id: i32,
    pub account_id: Uuid,
    pub has_join_room: bool,
}

impl Member {
    pub async fn get_one_by_account_id(account_id: Uuid, database: &PgPool) -> anyhow::Result<Member> {
        let member =
            sqlx::query_as!(Member, "SELECT * FROM members WHERE account_id = $1", account_id)
                .fetch_one(database)
                .await?;

        Ok(member)
    }
}
