pub fn map(x: u16, y: u16) -> f64 {
    let mut z: u32 = 0;

    for i in 0..16 {
        z |= if (x as u32 & (1 << i)) != 0 { 1 } else { 0 } << (i * 2);
        z |= if (y as u32 & (1 << i)) != 0 { 1 } else { 0 } << (i * 2 + 1);
    }

    return (z as f64) / (u32::MAX as f64);
}
