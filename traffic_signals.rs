pub struct SignalGantry {
    pub span_width: u32,
    pub structure_material: String,
    pub has_mast_arm: bool,
}

pub fn generate_signal_gantry(road_width: u32) -> SignalGantry {
    SignalGantry {
        span_width: road_width + 2,
        structure_material: "minecraft:anvil".to_string(),
        has_mast_arm: true,
    }
}
