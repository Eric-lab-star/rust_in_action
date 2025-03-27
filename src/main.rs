
fn main() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;
    let a_addr: usize = unsafe {
        std::mem::transmute(a_ptr)
    };

    println!("a: {} ({:p} ... 0X{:X})", a, a_ptr, a_addr + 7);

}
