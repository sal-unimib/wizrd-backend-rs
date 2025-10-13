use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("wizrd")]
pub struct Db(sqlx::PgPool);

