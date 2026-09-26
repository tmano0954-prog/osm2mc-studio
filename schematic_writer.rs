use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SpongeSchematic {
    pub version: i32,
    pub data_version: i32,
    pub width: i16,
    pub height: i16,
    pub length: i16,
    pub palette: HashMap<String, i32>,
    pub block_data: Vec<u8>,
}

pub fn create_schematic(width: i16, height: i16, length: i16) -> SpongeSchematic {
    let mut palette = HashMap::new();
    palette.insert("minecraft:cyan_terracotta".to_string(), 0);
    palette.insert("minecraft:yellow_wool".to_string(), 1);

    SpongeSchematic {
        version: 2,
        data_version: 3465,
        width,
        height,
        length,
        palette,
        block_data: vec![0, 0, 1, 0],
    }
}
