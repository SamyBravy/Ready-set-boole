pub fn map(x: u16, y: u16) -> f64 {
    let packed: u64 = morton32(x, y);
    packed as f64
}


fn part1by1(mut x: u32) -> u32 {
    x &= 0x0000_ffff;
    x = (x | (x << 8)) & 0x00FF_00FF;
    x = (x | (x << 4)) & 0x0F0F_0F0F;
    x = (x | (x << 2)) & 0x3333_3333;
    x = (x | (x << 1)) & 0x5555_5555;
    x
}

fn morton32(x: u16, y: u16) -> u64 {
    let xx = part1by1(x as u32);
    let yy = part1by1(y as u32) << 1;
    (yy | xx) as u64
}