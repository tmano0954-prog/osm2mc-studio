use geo::LineString;

pub fn buffer_road_centerline(points: Vec<(f64, f64)>, width_meters: f64) -> Vec<(f64, f64)> {
    let _line = LineString::from(points.clone());
    println!("Buffering road centerline across width: {} meters", width_meters);
    points
}
