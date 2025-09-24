pub fn map(x: u16, y: u16) -> f64 {
    let mut z: u32 = 0;

    for i in 0..16 {
        z |= ((x as u32 >> i) & 1) << (2 * i);
		z |= ((y as u32 >> i) & 1) << (2 * i + 1);
    }

    return (z as f64) / (u32::MAX as f64);
}
