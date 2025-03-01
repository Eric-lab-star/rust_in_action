use std::ops::Add;




fn main() {
    let floats = add(1.2, 3.4);
    let ints = add(10, 20);
    println!("{}", floats);
    println!("{}", ints);
    
}

fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}
