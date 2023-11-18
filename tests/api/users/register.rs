use fake::{faker::{internet::en::SafeEmail, name::en::NameWithTitle}, Fake};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use crate::helpers::spawn_app;


#[derive(Serialize, Debug ,Deserialize)]
pub struct NewUser<'a> {
    email: &'a str,
    password: String,
    unique_name: String,
    display_name:String,
}

#[sqlx::test]
async fn test_register_user_success(pool:PgPool){
    let app = spawn_app(pool.clone()).await;

    // Request data
    let email:String = SafeEmail().fake();
    let unique_name:String = fake::faker::name::en::Name().fake();
    let display_name:String = fake::faker::name::en::Name().fake();
    let password:String = NameWithTitle().fake();

    let new_user = NewUser{
        email: &email,
        password,
        unique_name,
        display_name,
    };
    
    let response = app.api_client.post(&format!("{}/users/register",&app.address))
        .json(&new_user)
        .header("Content-Type","application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

   let saved_user = sqlx::query!(
      "SELECT 
            u.id AS u_id, 
            u.email AS u_email, 
            u.password AS u_password, 
            u.unique_name AS u_unique_name,
            u.display_name AS u_display_name, 
            u.is_active AS u_is_active, 
            u.is_staff AS u_is_staff, 
            u.is_superuser AS u_is_superuser, 
            u.data_joined AS u_data_joined, 
            p.id AS p_id, 
            p.avatar_link AS p_avatar_link,
            p.user_id AS p_user_id, 
            p.phone_number AS p_phone_number, 
            p.birth_date AS p_birth_date, 
            p.github_link AS p_github_link 
        FROM 
            users u 
            LEFT JOIN user_profile p ON p.user_id = u.id
        WHERE 
            u.is_active=false AND u.email=$1
    ",
    &email
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch saved user.");
   assert_eq!(saved_user.u_email,email);
   assert_eq!(saved_user.u_is_active,false);
   assert_eq!(saved_user.p_avatar_link,None);
   assert_eq!(saved_user.u_id, saved_user.p_user_id);
   assert_eq!(saved_user.p_phone_number,None);
}
