use crate::model::point::Point;
use rocket_db_pools::sqlx;
use sqlx::PgConnection;
pub async fn get_point(conn: &mut PgConnection, id: i64) -> Result<Point, sqlx::Error> {
    sqlx::query_as::<_, Point>(
        r#"
             SELECT
                V.ID AS ID,
                COALESCE(E.NAME, '') AS NAME,
                COALESCE(E.HIGHWAY, '') AS HIGHWAY,
                E.GREEN AS GREEN,
                V.LAT::DOUBLE PRECISION AS LAT,
                V.LON::DOUBLE PRECISION AS LON
            FROM
                WAYS_VERTICES_PGR AS V
                JOIN WAYS AS E ON E.SOURCE = V.ID
                OR E.TARGET = V.ID
            WHERE
                V.ID = $1
            GROUP BY
                V.ID,
                E.NAME,
                E.HIGHWAY,
                E.GREEN,
                V.LAT,
                V.LON
            "#,
    )
    .bind(id)
    .fetch_one(conn)
    .await
}

pub async fn find_nearest_point(
    conn: &mut PgConnection,
    lat: f64,
    lon: f64,
) -> Result<Point, sqlx::Error> {
    sqlx::query_as::<_, Point>(
        r#"
            SELECT
                V.ID AS ID,
                COALESCE(E.NAME, '') AS NAME,
                COALESCE(E.HIGHWAY, '') AS HIGHWAY,
                E.GREEN AS GREEN,
                V.LAT::DOUBLE PRECISION AS LAT,
                V.LON::DOUBLE PRECISION AS LON
            FROM
                WAYS_VERTICES_PGR AS V
                JOIN WAYS AS E ON E.SOURCE = V.ID
                OR E.TARGET = V.ID
            WHERE
                V.ID = (
                    SELECT
                        ID
                    FROM
                        WAYS_VERTICES_PGR
                    ORDER BY
                        ST_DISTANCE (
                            THE_GEOM,
                            ST_SETSRID (ST_MAKEPOINT ($1, $2), 4326)
                        )
                    LIMIT
                        1
                )
            GROUP BY
                V.ID,
                E.NAME,
                E.HIGHWAY,
                E.GREEN,
                V.LAT,
                V.LON
            "#,
    )
    .bind(lon)
    .bind(lat)
    .fetch_one(conn)
    .await
}
