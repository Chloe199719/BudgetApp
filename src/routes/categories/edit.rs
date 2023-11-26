use actix_web::{ web::{ Path, Data }, put, HttpResponse };
use actix_web_validator::Json;
use serde::{ Deserialize, Serialize };
use sqlx::PgPool;
use validator::Validate;

use crate::{
    routes::users::logout::session_user_id,
    types::{ general::ErrorResponse, categories::Category },
    queries::category::check_category_exists,
    utils::constant::BACK_END_TARGET,
};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct EditCategory {
    #[validate(length(min = 3, max = 50))]
    pub name: Option<String>,
    #[validate(length(min = 3, max = 500))]
    pub description: Option<String>,
}
impl EditCategory {
    pub fn is_some(&self) -> bool {
        self.name.is_some() || self.description.is_some()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PathCategory {
    pub category_id: i32,
}
#[tracing::instrument(name = "Editing a category", skip(pool, session, edit_category))]
#[put("/edit/{category_id}")]
pub async fn edit_category(
    pool: Data<PgPool>,
    session: actix_session::Session,
    data: Path<PathCategory>,
    edit_category: Json<EditCategory>
) -> HttpResponse {
    if !edit_category.is_some() {
        tracing::event!(target: BACK_END_TARGET, tracing::Level::INFO, "No fields to edit");
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "No fields to edit".to_string(),
        });
    }

    let mut transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(e) => {
            tracing::event!(target: "discord_backend", tracing::Level::ERROR, "Unable to begin DB transaction: {:#?}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Something unexpected happened. Kindly try again.".to_string(),
            });
        }
    };
    let session_uuid = match session_user_id(&session).await {
        Ok(id) => id,
        Err(e) => {
            tracing::event!(target: "session", tracing::Level::ERROR, "Failed to get user from session. User unauthorized: {}", e);
            return HttpResponse::Unauthorized().json(ErrorResponse {
                error: "You are not logged in. Kindly ensure you are logged in and try again".to_string(),
            });
        }
    };
    match check_category_exists(&mut transaction, data.category_id, session_uuid).await {
        Ok(true) => (),
        Ok(false) => {
            transaction.rollback().await.expect("Failed to rollback transaction");
            tracing::event!(target: BACK_END_TARGET, tracing::Level::INFO, "Category does not exist");
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Category does not exist".to_string(),
            });
        }
        Err(e) => {
            transaction.rollback().await.expect("Failed to rollback transaction");
            tracing::event!(target:BACK_END_TARGET, tracing::Level::ERROR, "Failed to check if category exists: {:#?}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to edit category. Kindly try again.".to_string(),
            });
        }
    }
    let return_data = match
        edit_category_in_db(&mut transaction, data.category_id, session_uuid, &edit_category).await
    {
        Ok(e) => e,
        Err(e) => {
            transaction.rollback().await.expect("Failed to rollback transaction");
            tracing::event!(target:BACK_END_TARGET, tracing::Level::ERROR, "Failed to edit category in DB: {:#?}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to edit category. Kindly try again.".to_string(),
            });
        }
    };
    match transaction.commit().await.is_ok() {
        true => {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::INFO, "Successfully edited category");
            return HttpResponse::Ok().json(return_data);
        }
        false => {
            tracing::event!(target:BACK_END_TARGET, tracing::Level::ERROR, "Failed to commit transaction");
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to edit category. Kindly try again.".to_string(),
            });
        }
    }
}

#[rustfmt::skip]
#[tracing::instrument(name = "Edit category on DB", skip(transaction))]
async fn edit_category_in_db(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    category_id: i32,
    user_id: uuid::Uuid,
    edit_data: &EditCategory
) -> Result<Category, sqlx::Error> {
    match sqlx::query_as!(
        Category,
        r#"
            UPDATE categories
            SET
                category_name = COALESCE($1, category_name),
                description = COALESCE($2, description)
            WHERE category_id = $3 AND user_id = $4 
            RETURNING *
        "#,
        edit_data.name,
        edit_data.description,
        category_id,
        user_id
    )
    .fetch_one(transaction.as_mut())
    .await
    {
        Ok(e) => {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::INFO, "Successfully edited category");
            Ok(e)
        }
        Err(e) => {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Failed to edit category in DB: {:#?}", e);
            Err(e)
        }
    }
}
