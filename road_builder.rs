pub enum RoadBlock {
    CyanTerracotta,
    Stone,
    YellowWool,
    WhiteWool,
    QuartzStairs { facing: String },
}

pub fn build_roads(tags: &std::collections::HashMap<String, String>) -> RoadBlock {
    let surface = tags.get("surface").map(|s| s.as_str()).unwrap_or("asphalt");
    let tactile = tags.get("tactile_paving").map(|s| s.as_str()).unwrap_or("no");

    if tactile == "yes" {
        RoadBlock::YellowWool
    } else if surface == "paved" || surface == "stone" {
        RoadBlock::Stone
    } else {
        RoadBlock::CyanTerracotta
    }
}
