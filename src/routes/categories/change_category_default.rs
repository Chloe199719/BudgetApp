use actix_web::{
    put,
    web::{Data, Path},
    HttpResponse,
};
use serde::Deserialize;
use sqlx::{PgPool, Postgres, Transaction};

use crate::{
    queries::category::check_category_exists,
    routes::users::logout::session_user_id,
    types::general::{ErrorResponse, SuccessResponse},
    utils::constant::BACK_END_TARGET,
};

#[derive(Deserialize, Debug)]
pub struct PathDefaultCategory {
    pub category_id: i32,
}

#[tracing::instrument(name = "Changing default category", skip(pool, session, data))]
#[put("/change_default/{category_id}")]
pub async fn change_category_default(
    pool: Data<PgPool>,
    session: actix_session::Session,
    data: Path<PathDefaultCategory>,
) -> HttpResponse {
    let session_uuid = match session_user_id(&session).await {
        Ok(id) => id,
        Err(e) => {
            tracing::event!(
                target: "session",
                tracing::Level::ERROR,
                "Failed to get user from session. User unauthorized: {}",
                e
            );
            return HttpResponse::Unauthorized().json(ErrorResponse {
                error: "You are not logged in. Kindly ensure you are logged in and try again"
                    .to_string(),
            });
        }
    };

    match check_category_exists(&pool, data.category_id, session_uuid).await {
        Ok(true) => (),
        Ok(false) => {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::INFO, "Category does not exist");
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Category does not exist".to_string(),
            });
        }
        Err(e) => {
            tracing::event!(target:BACK_END_TARGET, tracing::Level::ERROR, "Failed to check if category exists: {:#?}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to delete category. Kindly try again.".to_string(),
            });
        }
    }

    //Create a transaction object

    let mut transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(e) => {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Unable to begin DB transaction: {:#?}", e);
            return actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Something unexpected happened. Kindly try again.".to_string(),
            });
        }
    };

    match change_the_default_category(&mut transaction, data.category_id, session_uuid).await {
        Ok(_) => {
            match transaction.commit().await {
                Ok(_) => (),
                Err(e) => {
                    tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Failed to commit transaction: {:#?}", e);
                    return HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "Failed to change default category. Kindly try again.".to_string(),
                    });
                }
            };
            HttpResponse::Ok().json(SuccessResponse {
                message: "Successfully changed default category".to_string(),
            })
        }
        Err(e) => {
            tracing::event!(target:BACK_END_TARGET, tracing::Level::ERROR, "Failed to change default category: {:#?}", e);
            transaction.rollback().await.unwrap();
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to change default category. Kindly try again.".to_string(),
            });
        }
    }
    // todo!("Change default category")
}

#[tracing::instrument(name = "Changing the Default Category", skip(transaction))]
async fn change_the_default_category(
    transaction: &mut Transaction<'_, Postgres>,
    category_id: i32,
    user_id: uuid::Uuid,
) -> Result<(), sqlx::Error> {
    //Remove old default category
    match sqlx::query!(
        r#"
        UPDATE categories
        SET is_default = false
        WHERE user_id = $1 AND is_default = true
        "#,
        user_id
    )
    .execute(transaction.as_mut())
    .await
    {
        Ok(_) => (),
        Err(e) => {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Failed to change default category: {:#?}", e);
            return Err(e);
        }
    };
    //Change desired category to default
    match sqlx::query!(
        r#"
        UPDATE categories
        SET is_default = true
        WHERE user_id = $1 AND category_id = $2
        "#,
        user_id,
        category_id
    )
    .execute(transaction.as_mut())
    .await
    {
        Ok(_) => (),
        Err(e) => {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Failed to change default category: {:#?}", e);
            return Err(e);
        }
    };

    Ok(())
}
