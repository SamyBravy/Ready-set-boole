pub fn gray_code(n: u32) -> u32 {
	return n ^ (n >> 1);
}