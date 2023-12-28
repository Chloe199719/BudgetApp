use actix_web::{ put, web::{ Data, Path }, HttpResponse };
use actix_web_validator::Json;
use serde::{ Deserialize, Serialize };
use sqlx::PgPool;
use validator::Validate;

use crate::{
    queries::category::check_category_exists_return_it,
    routes::users::logout::session_user_id,
    types::{ categories::Category, general::ErrorResponse },
    utils::constant::BACK_END_TARGET,
};

#[derive(Debug, Deserialize, Serialize, Validate, Clone)]
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
    let session_uuid = match session_user_id(&session).await {
        Ok(id) => id,
        Err(e) => {
            tracing::event!(target: "session", tracing::Level::ERROR, "Failed to get user from session. User unauthorized: {}", e);
            return HttpResponse::Unauthorized().json(ErrorResponse {
                error: "You are not logged in. Kindly ensure you are logged in and try again".to_string(),
            });
        }
    };
    match check_category_exists_return_it(&pool, data.category_id, session_uuid).await {
        Ok(category) => {
            let edit_category = edit_category.clone();
            if
                category.description ==
                    edit_category.description.unwrap_or(category.description.clone()) &&
                category.category_name ==
                    edit_category.name.unwrap_or(category.category_name.clone())
            {
                tracing::event!(target: BACK_END_TARGET, tracing::Level::INFO, "No fields to edit");
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error: "No fields to edit".to_string(),
                });
            }
        }
        Err(e) => {
            tracing::event!(target:BACK_END_TARGET, tracing::Level::ERROR, "Failed to check if category exists: {:#?}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to edit category. Kindly try again.".to_string(),
            });
        }
    }
    let return_data = match
        edit_category_in_db(&pool, data.category_id, session_uuid, &edit_category).await
    {
        Ok(e) => e,
        Err(e) => {
            tracing::event!(target:BACK_END_TARGET, tracing::Level::ERROR, "Failed to edit category in DB: {:#?}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to edit category. Kindly try again.".to_string(),
            });
        }
    };

    tracing::event!(target: BACK_END_TARGET, tracing::Level::INFO, "Successfully edited category");
    return HttpResponse::Ok().json(return_data);
}

#[rustfmt::skip]
#[tracing::instrument(name = "Edit category on DB", skip(pool))]
async fn edit_category_in_db(
    pool: &PgPool,
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
            RETURNING category_id, category_name, description, user_id, created_at, updated_at, is_default;
        "#,
        edit_data.name,
        edit_data.description,
        category_id,
        user_id
    )
    .fetch_one(pool)
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
