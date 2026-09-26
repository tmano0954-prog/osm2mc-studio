use osm2mc_core::osm::parser::OSMDocument;

fn main() {
    println!("OSM2MC Studio Core Engine - Standalone Runner");
    let sample_data = r#"{"elements": []}"#;
    let doc = OSMDocument::from_json(sample_data);
    println!("Loaded OSM Document. Elements count: {}", doc.elements.len());
}
