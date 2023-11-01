use sqlx::Row;

use crate::{
    utils::{ emails::send_multipart_email, auth::password::hash },
    types::general::{ SuccessResponse, ErrorResponse },
};

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct NewUser {
    email: String,
    password: String,
    display_name: String,
    unique_name: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CreateNewUser {
    email: String,
    password: String,
    display_name: String,
    unique_name: String,
}
#[tracing::instrument(name = "Adding a new user",
skip( pool, new_user, redis_pool),
fields(
    new_user_email = %new_user.email,
    new_user_display_name = %new_user.display_name,
    new_user_unique_name = %new_user.unique_name,
))]

#[actix_web::post("/register")]
pub async fn register_user(
    pool: actix_web::web::Data<sqlx::postgres::PgPool>,
    new_user: actix_web::web::Json<NewUser>,
    redis_pool: actix_web::web::Data<deadpool_redis::Pool>
) -> actix_web::HttpResponse {
    let mut transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(e) => {
            tracing::event!(target: "discord_backend", tracing::Level::ERROR, "Unable to begin DB transaction: {:#?}", e);
            return actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Something unexpected happend. Kindly try again.".to_string(),
            });
        }
    };
    let hashed_password = hash(new_user.0.password.as_bytes()).await;

    let create_new_user = CreateNewUser {
        password: hashed_password,
        email: new_user.0.email,
        display_name: new_user.0.display_name,
        unique_name: new_user.0.unique_name,
    };

    let user_id = match insert_created_user_into_db(&mut transaction, &create_new_user).await {
        Ok(id) => id,
        Err(e) => {
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "Failed to insert user into DB: {:#?}", e);
            let error_message = if
                e.as_database_error().unwrap().code().unwrap().parse::<i32>().unwrap() == 23505
            {
                ErrorResponse {
                    error: "A user with that email address already exists".to_string(),
                }
            } else {
                ErrorResponse {
                    error: "Error inserting user into the database".to_string(),
                }
            };
            return actix_web::HttpResponse::InternalServerError().json(error_message);
        }
    };

    // send confirmation email to the new user.
    let mut redis_con = redis_pool
        .get().await
        .map_err(|e| {
            tracing::event!(target: "discord_backend", tracing::Level::ERROR, "{}", e);
            actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                error: "We cannot activate your account at the moment".to_string(),
            })
        })
        .expect("Redis connection cannot be gotten.");

    send_multipart_email(
        "Welcome To Discord Clone Please Verify your email 😀".to_string(),
        user_id,
        create_new_user.email,
        create_new_user.display_name,
        "verification_email.html",
        &mut redis_con
    ).await.unwrap();

    if transaction.commit().await.is_err() {
        return actix_web::HttpResponse::InternalServerError().finish();
    }

    tracing::event!(target: "discord_backend", tracing::Level::INFO, "User created successfully.");
    actix_web::HttpResponse::Ok().json(SuccessResponse {
        message: "Your account was created successfully. Check your email address to activate your account as we just sent you an activation link. Ensure you activate your account before the link expires".to_string(),
    })
}

#[tracing::instrument(name = "Inserting new user into DB.", skip(transaction, new_user),fields(
    new_user_email = %new_user.email,
    new_user_display_name = %new_user.display_name,
    new_user_unique_name = %new_user.unique_name,

))]
async fn insert_created_user_into_db(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    new_user: &CreateNewUser
) -> Result<uuid::Uuid, sqlx::Error> {
    let user_id = match
        sqlx
            ::query(
                "INSERT INTO users (email, password, display_name, unique_name) VALUES ($1, $2, $3,$4) RETURNING id"
            )
            .bind(&new_user.email)
            .bind(&new_user.password)
            .bind(&new_user.display_name)
            .bind(&new_user.unique_name)
            .map(|row: sqlx::postgres::PgRow| -> uuid::Uuid { row.get("id") })
            .fetch_one(&mut *transaction.as_mut()).await
    {
        Ok(id) => id,
        Err(e) => {
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "Failed to insert user into DB: {:#?}", e);
            return Err(e);
        }
    };

    match
        sqlx
            ::query(
                "INSERT INTO user_profile (user_id) 
                VALUES ($1) 
            ON CONFLICT (user_id) 
            DO NOTHING
            RETURNING user_id"
            )
            .bind(user_id)
            .map(|row: sqlx::postgres::PgRow| -> uuid::Uuid { row.get("user_id") })
            .fetch_one(&mut *transaction.as_mut()).await
    {
        Ok(id) => {
            tracing::event!(target: "sqlx",tracing::Level::INFO, "User profile created successfully {}.", id);
            Ok(id)
        }
        Err(e) => {
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "Failed to insert user's profile into DB: {:#?}", e);
            Err(e)
        }
    }
}
