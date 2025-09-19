use crate::adder::adder;



pub fn multiplier(a: u32, b: u32) -> u32 {
	let mut ret = 0;
	for i in 0..32 {
		if (b & (1 << i)) != 0 {
			ret = adder(ret, a << i);
		}
	}
	return ret;
}