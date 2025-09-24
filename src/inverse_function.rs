pub fn reverse_map(n: f64) -> (u16, u16) {
    if n < 0.0 || n > 1.0 {
        println!("Input must be in the range [0.0, 1.0]");
        return (0, 0);
    }

    let scaled: u32 = (n * (u32::MAX as f64)) as u32;
    let mut x: u32 = 0;
    let mut y: u32 = 0;

    for i in 0..16 {
        x |= ((scaled >> (i * 2)) & 1) << i;
        y |= ((scaled >> (i * 2 + 1)) & 1) << i;
    }

    return (x as u16, y as u16);
}
