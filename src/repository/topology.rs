use crate::dto::TopologyWay;
use geo_traits::to_geo::ToGeoGeometry;
use geojson::Feature;
use sqlx::{PgConnection, Row};
use wkt::Wkt;

pub async fn filter(
    pool: &mut PgConnection,
    where_clause: &str,
    data_col: &str,
) -> Result<Vec<TopologyWay>, sqlx::Error> {
    let query = format!(
        "
        SELECT
            W.ID,
            ST_ASTEXT (ST_LINEMERGE (ST_COLLECT (W.THE_GEOM))) AS __GEOM,
            {}::DOUBLE PRECISION AS __DATA
        FROM
            WAYS W
            JOIN DATA D ON W.ID = D.ID {}
        GROUP BY
            W.ID,
            __DATA
        ",
        data_col,
        where_clause
    );

    let rows = sqlx::query(query.as_str())
        .fetch_all(pool)
        .await?;

    let mut topo_ways = Vec::new();

    for row in rows {
        let the_geom = row
            .get::<&str, &str>("__geom")
            .parse::<Wkt>()
            .map_err(|_e| {
                eprintln!("{:?}", _e);
                sqlx::Error::Protocol(format!("Could not deserialize WKT: {}", _e))
            })?
            .to_geometry();

        let data : Option<f64> = row.try_get("__data").unwrap_or(None);

        topo_ways.push(TopologyWay {
            id: row.get("id"),
            geom: Feature::from(geojson::Geometry::from(&the_geom)),
            data,
        });
    }

    Ok(topo_ways)
}
