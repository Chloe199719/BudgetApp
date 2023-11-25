use actix_multipart::form;
use actix_web::{ patch, web::Data, HttpResponse };
use chrono::{ DateTime, Utc };
use serde::Deserialize;
use sqlx::{ PgPool, Postgres, Transaction };
use uuid::Uuid;
use crate::{
    uploads::client::Client,
    types::{ general::ErrorResponse, UserVisible },
    utils::users::get_active_user_from_db,
};

use super::logout::session_user_id;

#[derive(form::MultipartForm)]
pub struct UserForm {
    unique_name: Option<form::text::Text<String>>,
    display_name: Option<form::text::Text<String>>,
    #[multipart(limit = "1 MiB")]
    avatar: Option<form::tempfile::TempFile>,
    phone_number: Option<form::text::Text<String>>,
    birth_date: Option<form::text::Text<DateTime<Utc>>>,
    github_link: Option<form::text::Text<String>>,
    about_me: Option<form::text::Text<String>>,
    pronouns: Option<form::text::Text<String>>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateUser {
    pub unique_name: Option<String>,
    pub display_name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateUserProfile {
    pub phone_number: Option<String>,
    pub birth_date: Option<DateTime<Utc>>,
    pub github_link: Option<String>,
    pub about_me: Option<String>,
    pub pronouns: Option<String>,
    pub avatar_link: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Avatar {
    pub avatar: Option<String>,
}
#[rustfmt::skip]

#[tracing::instrument(name = "Updating an user", skip(pool, form, session, s3_client))]
#[patch("/update_user")]
pub async fn update_users_details(
    pool: Data<PgPool>,
    form: actix_multipart::form::MultipartForm<UserForm>,
    session: actix_session::Session,
    s3_client: Data<Client>
) -> actix_web::HttpResponse {
    let session_uuid = match session_user_id(&session).await {
        Ok(id) => id,
        Err(e) => {
            tracing::event!(target: "session", tracing::Level::ERROR, "Failed to get user from session. User unauthorized: {}", e);
            return actix_web::HttpResponse::Unauthorized().json(ErrorResponse {
                error: "You are not logged in. Kindly ensure you are logged in and try again".to_string(),
            });
        }
    };

    //Create a transaction object

    let mut transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(e) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "Unable to begin DB transaction: {:#?}", e);
            return actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Something unexpected happened. Kindly try again.".to_string(),
            });
        }
    };

    let mut update_user = UpdateUser {
        unique_name: None,
        display_name: None,
    };

    let mut user_profile = UpdateUserProfile {
        phone_number: None,
        birth_date: None,
        github_link: None,
        about_me: None,
        pronouns: None,
        avatar_link: None,
    };

    if let Some(avatar) = &form.0.avatar {

        //Get the user's current avatar
        let user_current_avatar = match
            sqlx
                ::query!(
                    r#"
                SELECT avatar_link
                FROM user_profile
                WHERE user_id = $1
            "#,
                    session_uuid
                )
                .fetch_one(&mut *transaction).await
        {
            Ok(user_current_avatar) => user_current_avatar.avatar_link,
            Err(e) => {
                tracing::event!(target: "sqlx",tracing::Level::ERROR, "Failed to get user thumbnail from the DB: {:#?}", e);
                None
            }
        };

        //If There is a current image, delete it

        if let Some(url) = user_current_avatar {
            let s3_image_key = &url[url.find("media").unwrap_or(url.len())..];

            if !s3_client.delete_file(s3_image_key).await {
                tracing::event!(target: "backend",  tracing::Level::INFO ,  "We could not delete the current avatar of user with ID: {}", session_uuid)
            }
        }

        // make key prefix (makes sure it ends with a slash)

        let s3_key_prefix = format!("media/discord_backend/avatar/{session_uuid}/");

        let uploaded_file = s3_client.upload(avatar,&s3_key_prefix).await;
        user_profile.avatar_link= Some(uploaded_file.s3_url)
    }

    // Update the unique_name // TODO: Add a limited number of times a user can change their unique_name per week
    if let Some(unique_name) = form.0.unique_name {
        update_user.unique_name = Some(unique_name.0);
    }

    // Update the display_name
    if let Some(display_name) = form.0.display_name {
        update_user.display_name = Some(display_name.0);
    }

    // Update the phone_number
    if let Some(phone_number) = form.0.phone_number {
        user_profile.phone_number = Some(phone_number.0);
    }

    // Update the birth_date
    if let Some(birth_date) =form.0.birth_date {
        user_profile.birth_date = Some(birth_date.0);
    }

    // Update the github_link
    if let Some(github_link) = form.0.github_link {
        user_profile.github_link = Some(github_link.0);
    }

    if let Some(about_me) = form.0.about_me {
        user_profile.about_me = Some(about_me.0);
    }

    // Update the pronouns
    if let Some(pronouns) = form.0.pronouns {
        user_profile.pronouns = Some(pronouns.0);
    }

    // Update the user in the DB

    match update_user_in_db(&mut transaction, &update_user, &user_profile, session_uuid).await {
        Ok(u) => u,
        Err(e) => {
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "Failed to update user in DB: {:#?}", e);
            let error_message = ErrorResponse {
                error: format!("User could not be updated: {e}"),
            };
            return actix_web::HttpResponse::InternalServerError().json(error_message);
        }
    }
    let updated_user = match get_active_user_from_db(
        None,
        Some(&mut transaction),
        Some(session_uuid),
        None,
    ).await {
        Ok(user) => {
            tracing::event!(target: "discord-backend", tracing::Level::INFO, "User retrieved from the DB.");
            UserVisible {
                id: user.id,
                email: user.email,
                display_name: user.display_name,
                unique_name: user.unique_name,
                is_active: user.is_active,
                is_staff: user.is_staff,
                is_superuser: user.is_superuser,
                thumbnail: user.thumbnail,
                data_joined: user.data_joined,
                profile: user.profile,
            }
        }
        Err(e) => {
            tracing::event!(target: "discord-backend", tracing::Level::ERROR, "User cannot be retrieved from the DB: {:#?}", e);
            let error_message = ErrorResponse {
                error: "User was not found".to_string(),
            };
            return actix_web::HttpResponse::NotFound().json(error_message);
        }
    };
    if transaction.commit().await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    tracing::event!(target: "discord-backend", tracing::Level::INFO, "User updated successfully.");
    HttpResponse::Ok().json(updated_user)
}

#[tracing::instrument(name = "Updating user in DB.", skip(transaction))]
async fn update_user_in_db(
    transaction: &mut Transaction<'_, Postgres>,
    user_to_update: &UpdateUser,
    user_profile_to_update: &UpdateUserProfile,
    user_id: Uuid
) -> Result<(), sqlx::Error> {
    match
        sqlx
            ::query(
                r#"
        UPDATE users
        SET unique_name = COALESCE($1, unique_name), display_name = COALESCE($2, display_name)
        WHERE id = $3
        AND is_active = true
        AND (
            $1 IS NOT NULL
            AND $1 IS DISTINCT
            FROM
                unique_name
                OR $2 IS NOT NULL
                AND $2 IS DISTINCT
            FROM
                display_name
        )
        "#
            )
            .bind(&user_to_update.unique_name)
            .bind(&user_to_update.display_name)
            .bind(user_id)
            .execute(&mut *transaction.as_mut()).await
    {
        Ok(r) => {
            tracing::event!(target: "sqlx", tracing::Level::INFO, "User has been updated successfully: {:#?}", r);
        }
        Err(e) => {
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "Failed to update user into DB: {:#?}", e);
            return Err(e);
        }
    }

    match
        sqlx
            ::query(
                "
            UPDATE 
                user_profile 
            SET 
                phone_number = COALESCE($1, phone_number), 
                birth_date = $2, 
                github_link = COALESCE($3, github_link),
                avatar_link = COALESCE($4, avatar_link),
                about_me = COALESCE($5, about_me),
                pronouns = COALESCE($6, pronouns)
            WHERE 
                user_id = $7 
                AND (
                    $1 IS NOT NULL 
                    AND $1 IS DISTINCT 
                    FROM 
                        phone_number 
                        OR $2 IS NOT NULL 
                        AND $2 IS DISTINCT 
                    FROM 
                        birth_date 
                        OR $3 IS NOT NULL 
                        AND $3 IS DISTINCT 
                    FROM 
                        github_link
                        OR $4 IS NOT NULL
                        AND $4 IS DISTINCT
                    FROM
                        avatar_link
                        OR $5 IS NOT NULL
                        AND $5 IS DISTINCT
                    FROM
                        about_me
                        OR $6 IS NOT NULL
                        AND $6 IS DISTINCT
                    FROM
                        pronouns
                )"
            )
            .bind(&user_profile_to_update.phone_number)
            .bind(user_profile_to_update.birth_date)
            .bind(&user_profile_to_update.github_link)
            .bind(&user_profile_to_update.avatar_link)
            .bind(&user_profile_to_update.about_me)
            .bind(&user_profile_to_update.pronouns)
            .bind(user_id)
            .execute(&mut *transaction.as_mut()).await
    {
        Ok(r) => {
            tracing::event!(target: "sqlx", tracing::Level::INFO, "User profile has been updated successfully: {:#?}", r);
        }
        Err(e) => {
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "Failed to update user profile into DB: {:#?}", e);
            return Err(e);
        }
    }

    Ok(())
}
