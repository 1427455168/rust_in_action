use std::ops::Add;
use std::time::Duration;


fn main() {
    let floats = add(1.1, 2.2);
    let ints = add(1, 2);
    let durations = add(Duration::new(5, 0), Duration::new(10, 0));

    println!("{}", floats);
    println!("{}", ints);
    println!("{:?}", durations);
}


fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}
