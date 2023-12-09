use discord_backend::routes::categories::create_category::CreateCategory;
use discord_backend::types::categories::Category;
use fake::faker::lorem::en::Sentence;
use fake::faker::name::en::Name;
use fake::Fake;
use sqlx::PgPool;

mod create_category;
mod delete_category;
mod edit_category;

mod change_default_category;
mod get_all_categories_by_user_id;
mod get_category_by_id;

pub const TEST_CATEGORY_NAME: &str = "Test Category";
pub const TEST_CATEGORY_DESCRIPTION: &str = "Test Description";

#[rustfmt::skip]
pub async fn create_category_in_db(
    pool: &PgPool,
    user_id: uuid::Uuid
) -> Result<Category, sqlx::Error> {
    let create_category = CreateCategory {
        name: Name().fake(),
        description: Sentence(1..2).fake(),
    };
    match
        sqlx::query_as!(
            Category,"
                INSERT INTO categories (category_name, description, user_id)
                VALUES ($1, $2, $3)
                RETURNING *;",
                create_category.name,
                create_category.description,
                user_id
            )
            .fetch_one(pool).await
    {
        Ok(e) => {
            Ok(e)
        }
        Err(e) => {
            Err(e)
        }
    }
}
