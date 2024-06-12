use sqlx::PgPool;

#[derive(sqlx::FromRow)]
struct ScoreRecord {
    room_id: i32,
    team_id: i32,
    score: i64,
}

/// Cron job for update scores table (**highly experimental**)
#[tracing::instrument(err)]
pub async fn update_score(database: &PgPool) -> anyhow::Result<()> {
    // TODO: what is this????
    let score_records = sqlx::query_as::<_, ScoreRecord>(
        r#"
SELECT room_id, team_id, SUM(score) as score
FROM (
	SELECT
		scores.room_id as room_id,
		submit_histories.question_id as queston_id, 
		members.team_id as team_id, 
		MAX(score) as score
	FROM submit_histories
	INNER JOIN members ON members.id = submit_histories.member_id
	INNER JOIN scores ON scores.id = submit_histories.score_id
	GROUP BY scores.room_id, submit_histories.question_id, members.team_id
) score_by_question
GROUP BY room_id, team_id;
"#,
    )
    .fetch_all(database)
    .await?;

    for score_record in score_records {
        let (room_id, team_id, score) = (
            score_record.room_id,
            score_record.team_id,
            score_record.score,
        );

        sqlx::query!(
            r#"
                    UPDATE scores
                    SET total_score = $3
                    WHERE room_id = $1 AND team_id = $2
                    "#,
            room_id,
            team_id,
            score as i32
        )
        .execute(database)
        .await?;
    }

    Ok(())
}
