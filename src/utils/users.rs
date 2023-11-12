use sqlx::postgres::PgRow;
use sqlx::Row;
use crate::{ queries::users::USER_AND_USER_PROFILE_QUERY, types::User };

#[tracing::instrument(name = "Getting an active user from the DB.", skip(pool))]
pub async fn get_active_user_from_db(
    pool: Option<&sqlx::postgres::PgPool>,
    transaction: Option<&mut sqlx::Transaction<'_, sqlx::Postgres>>,
    id: Option<uuid::Uuid>,
    email: Option<&String>
) -> Result<crate::types::User, sqlx::Error> {
    let mut query_builder = sqlx::query_builder::QueryBuilder::new(USER_AND_USER_PROFILE_QUERY);

    if let Some(id) = id {
        query_builder.push(" u.id=");
        query_builder.push_bind(id);
    } else if let Some(e) = email {
        query_builder.push(" u.email=");
        query_builder.push_bind(e);
    } else {
        return Err(sqlx::Error::RowNotFound);
    }

    let sqlx_query = query_builder.build().map(|row: PgRow| User {
        id: row.get("u_id"),
        email: row.get("u_email"),
        password: row.get("u_password"),
        display_name: row.get("u_display_name"),
        unique_name: row.get("u_unique_name"),
        is_active: row.get("u_is_active"),
        is_staff: row.get("u_is_staff"),
        is_superuser: row.get("u_is_superuser"),
        thumbnail: row.get("u_thumbnail"),
        data_joined: row.get("u_data_joined"),
        profile: crate::types::UserProfile {
            id: row.get("p_id"),
            phone_number: row.get("p_phone_number"),
            birth_date: row.get("p_birth_date"),
            github_link: row.get("p_github_link"),
            about_me: row.get("p_about_me"),
            pronouns: row.get("p_pronouns"),
            avatar_link: row.get("p_avatar_link"),
        },
    });
    let fetched_query = {
        if pool.is_some() {
            let p = pool.unwrap();
            sqlx_query.fetch_one(p).await
        } else {
            let t = transaction.unwrap();
            sqlx_query.fetch_one(&mut *t.as_mut()).await
        }
    };
    match fetched_query {
        Ok(u) => Ok(u),
        Err(e) => {
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "User not found in DB: {:#?}", e);
            Err(e)
        }
    }
}
