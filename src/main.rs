

fn mock_rand(n: u8) -> f32 {
    let base: u32 = 0b00111111_00000000_00000000_00000000;
    let large_n: u32 = (n as u32) << 15;
    let f32_bits = base | large_n;
    let m = f32::from_bits(f32_bits);
    2.0 * (m - 0.5)
}

fn main() {
    println!("{:08b} -> {:?}", 0xff, mock_rand(0xff));
    println!("{:08b} -> {:?}", 0x7f, mock_rand(0x7f));
    println!("{:08b} -> {:?}", 0x00, mock_rand(0x00));
}
