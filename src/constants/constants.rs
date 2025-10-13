pub const ROUTING_ALGO_DIJKSTRA: &str = "dijkstra";
pub const _ROUTING_ALGO_KSP: &str = "ksp";
pub const _ROUTING_ALGO_ASTAR: &str = "astar";

pub const TRAFFIC_SOURCE: &str = "traffic";
pub const AIR_QUALITY_SOURCE: &str = "pm2";

pub fn multilevel_source(source: &str, level: i32) -> String {
    match source {
        TRAFFIC_SOURCE | AIR_QUALITY_SOURCE => match level {
            1 => format!("{}_low", source),
            2 => format!("{}_medium", source),
            3 => format!("{}_high", source),
            _ => source.to_string(),
        },
        _ => source.to_string(),
    }
}