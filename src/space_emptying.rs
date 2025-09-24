pub fn reverse_map(n: f64) -> (u16, u16) {
	let n = n as u64;
	let x = compact1by1(n);
	let y = compact1by1(n >> 1);
	(x as u16, y as u16)
}

fn compact1by1(mut x: u64) -> u32 {
	x &= 0x5555_5555_5555_5555;
	x = (x | (x >> 1)) & 0x3333_3333_3333_3333;
	x = (x | (x >> 2)) & 0x0F0F_0F0F_0F0F_0F0F;
	x = (x | (x >> 4)) & 0x00FF_00FF_00FF_00FF;
	x = (x | (x >> 8)) & 0x0000_FFFF_0000_FFFF;
	x = (x | (x >> 16)) & 0x0000_0000_FFFF_FFFF;
	x as u32
}