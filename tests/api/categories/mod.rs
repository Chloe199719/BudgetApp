use discord_backend::routes::categories::create_category::CreateCategory;
use fake::faker::name::en::Name;
use fake::Fake;
use sqlx::PgPool;
use fake::faker::lorem::en::Sentence;
mod create_category;

#[rustfmt::skip]
pub async fn create_category_in_db(
    pool: &PgPool,
    user_id: uuid::Uuid
) -> Result<i32, sqlx::Error> {
    let create_category = CreateCategory {
        name: Name().fake(),
        description: Sentence(1..2).fake(),
    };
    match
        sqlx::query!("
                INSERT INTO categories (category_name, description, user_id)
                VALUES ($1, $2, $3)
                RETURNING category_id;",
                create_category.name,
                create_category.description,
                user_id
            )
            .fetch_one(pool).await
    {
        Ok(e) => {
            Ok(e.category_id)
        }
        Err(e) => {
            Err(e)
        }
    }
}
