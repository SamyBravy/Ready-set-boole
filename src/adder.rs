pub fn adder(a: u32, b: u32) -> u32 {
    let mut carry: u32 = 0;
    let mut ret: u32 = 0;
    for i in 0..32 {
        let sum_bit = (a >> i & 1) ^ (b >> i & 1) ^ (carry);
        ret |= sum_bit << i;
        carry = ((a >> i & 1) & (b >> i & 1)) | (((a >> i & 1) ^ (b >> i & 1)) & carry);
    }
    return ret;
}
