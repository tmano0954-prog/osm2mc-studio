pub struct BuildingFootprint {
    pub height_blocks: u32,
    pub wall_block: String,
    pub glass_block: String,
}

pub fn parse_building_height(tags: &std::collections::HashMap<String, String>) -> BuildingFootprint {
    let height = tags.get("height")
        .and_then(|h| h.replace("m", "").parse::<u32>().ok())
        .unwrap_or_else(|| {
            tags.get("building:levels")
                .and_then(|l| l.parse::<u32>().ok())
                .map(|l| l * 4)
                .unwrap_or(4)
        });

    BuildingFootprint {
        height_blocks: height,
        wall_block: "minecraft:stone_bricks".to_string(),
        glass_block: "minecraft:glass_pane".to_string(),
    }
}
