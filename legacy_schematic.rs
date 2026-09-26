pub struct LegacySchematic {
    pub width: i16,
    pub height: i16,
    pub length: i16,
    pub blocks: Vec<u8>,      // Block IDs (e.g., 159 for Stained Clay)
    pub data_values: Vec<u8>, // Metadata (e.g., 9 for Cyan)
}

pub fn export_1_7_10_schematic(width: i16, height: i16, length: i16) -> LegacySchematic {
    LegacySchematic {
        width,
        height,
        length,
        blocks: vec![159, 159, 35],
        data_values: vec![9, 9, 4], // Cyan clay + Yellow wool
    }
}
