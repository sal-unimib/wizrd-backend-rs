use crate::model::poi::Poi;
use rocket_db_pools::sqlx;
use sqlx::PgConnection;

pub async fn get_all(conn: &mut PgConnection) -> Result<Vec<Poi>, sqlx::Error> {
    sqlx::query_as::<_, Poi>(
        r#"
            SELECT
                *
            FROM
                POIS
            "#,
    )
    .fetch_all(conn)
    .await
}

pub async fn get_all_by_kind(conn: &mut PgConnection, kind: &str) -> Result<Vec<Poi>, sqlx::Error> {
    sqlx::query_as::<_, Poi>(
        r#"
            SELECT
                *
            FROM
                POIS
            WHERE
                TYPE = $1
            "#,
    )
    .bind(kind)
    .fetch_all(conn)
    .await
}

pub async fn find_nearest(
    conn: &mut PgConnection,
    lat: f64,
    lon: f64,
) -> Result<Option<Poi>, sqlx::Error> {
    sqlx::query_as::<_, Poi>(
        r#"
            SELECT
                *
            FROM
                POIS
            ORDER BY
                ST_DISTANCE (ST_MAKEPOINT (LON, LAT), ST_MAKEPOINT ($1, $2))
            LIMIT
                1
            "#,
    )
    .bind(lon)
    .bind(lat)
    .fetch_optional(conn)
    .await
}

pub async fn find_nearest_by_kind(
    conn: &mut PgConnection,
    lat: f64,
    lon: f64,
    kind: &str,
) -> Result<Option<Poi>, sqlx::Error> {
    sqlx::query_as::<_, Poi>(
        r#"
            SELECT
                *
            FROM
                POIS
            WHERE
                TYPE = $1
            ORDER BY
                ST_DISTANCE (ST_MAKEPOINT (LON, LAT), ST_MAKEPOINT ($2, $3))
            LIMIT
                1
            "#,
    )
    .bind(kind)
    .bind(lon)
    .bind(lat)
    .fetch_optional(conn)
    .await
}
