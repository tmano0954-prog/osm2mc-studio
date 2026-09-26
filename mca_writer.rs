pub fn write_mca_region(region_x: i32, region_z: i32, chunk_data: &[u8]) -> Vec<u8> {
    println!("Encoding MCA Anvil region ({}, {})", region_x, region_z);
    let mut header = vec![0u8; 8192];
    header.extend_from_slice(chunk_data);
    header
}
