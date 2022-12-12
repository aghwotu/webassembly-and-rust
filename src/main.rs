fn add(num_one: i32, num_two: i32) -> i32 {
    num_one + num_two
}

fn main() {
//    let foo:bool = false;
   let foo = add(10, 45);
    println!("{}", foo); // 55
    println!("{} {}", foo, true); // 55 true
    println!("{0} {0}", foo); // 55 55
    println!("{:?}", foo); // 55
}   
