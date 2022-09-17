fn main() {

    let x = |i:i32|->i32{i*10};
    println!("Hello, world! {:?}", x(12));

    let y = | i |{i*10};
    println!("Hello, world! {:?}", y(10));
}
