use budget_app::routes::categories::create_category::CreateCategory;
use budget_app::types::categories::Category;
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

pub async fn create_category_in_db(
    pool: &PgPool,
    user_id: uuid::Uuid,
) -> Result<Category, sqlx::Error> {
    let create_category = CreateCategory {
        name: Name().fake(),
        description: Sentence(1..2).fake(),
    };
    match sqlx::query_as!(
        Category,
        "
        WITH new_category AS (
            INSERT INTO categories (category_name, description, user_id)
            VALUES ($1, $2, $3)
            RETURNING *
        )
        SELECT 
            nc.category_id, 
            nc.category_name, 
            nc.description, 
            nc.user_id, 
            nc.created_at, 
            nc.updated_at, 
            nc.is_default,
            nc.budget_id,
            b.amount,
            b.start_date,
            b.end_date,
            b.recurring
        FROM 
            new_category nc
        LEFT JOIN 
            budgets b
        ON 
            nc.budget_id = b.budget_id;",
        create_category.name,
        create_category.description,
        user_id
    )
    .fetch_one(pool)
    .await
    {
        Ok(e) => Ok(e),
        Err(e) => Err(e),
    }
}
