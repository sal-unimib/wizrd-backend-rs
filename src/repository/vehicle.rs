use crate::model::vehicle::Vehicle;
use rocket_db_pools::sqlx;
use sqlx::PgConnection;

pub async fn get_vehicles(conn: &mut PgConnection) -> Result<Vec<Vehicle>, sqlx::Error> {
    sqlx::query_as::<_, Vehicle>(
        r#"
            SELECT
                *
            FROM
                VEHICLES
            "#,
    )
    .fetch_all(conn)
    .await
}

pub async fn get_vehicle(
    conn: &mut PgConnection,
    name: &str,
) -> Result<Option<Vehicle>, sqlx::Error> {
    sqlx::query_as::<_, Vehicle>(
        r#"
            SELECT
                *
            FROM
                VEHICLES
            WHERE
                KIND = $1::VEHICLE_KIND
            "#,
    )
    .bind(name)
    .fetch_optional(conn)
    .await
}
