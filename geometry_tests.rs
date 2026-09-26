#[cfg(test)]
mod tests {
    use osm2mc_core::osm::buffer::buffer_road_centerline;

    #[test]
    fn test_road_buffering() {
        let coords = vec![(28.5383, -81.3792), (28.5385, -81.3790)];
        let buffered = buffer_road_centerline(coords, 14.0);
        assert_eq!(buffered.len(), 2);
    }
}
