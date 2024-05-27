use moka::future::Cache;
use serde::Serialize;
use sqlx::PgPool;
use tokio::sync::OnceCell;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Member {
    pub id: i32,
    pub team_id: i32,
    pub account_id: Uuid,
    pub has_join_room: bool,
}

impl Member {
    async fn get_one_by_account_id_internal(
        account_id: Uuid,
        database: &PgPool,
    ) -> sqlx::Result<Member> {
        sqlx::query_as!(
            Member,
            "SELECT * FROM members WHERE account_id = $1",
            account_id
        )
        .fetch_one(database)
        .await
    }

    pub async fn get_one_by_account_id(
        account_id: Uuid,
        database: &PgPool,
    ) -> anyhow::Result<Member> {
        // TODO: find best cache size
        const CACHE_SIZE: u64 = 50;
        static CACHE: OnceCell<Cache<Uuid, Member>> = OnceCell::const_new();

        let cache = CACHE.get_or_init(|| async { Cache::new(CACHE_SIZE) }).await;

        match cache
            .try_get_with(
                account_id,
                Member::get_one_by_account_id_internal(account_id, database),
            )
            .await
        {
            Ok(member) => Ok(member),
            Err(error) => anyhow::bail!(error.to_string()),
        }
    }
}
