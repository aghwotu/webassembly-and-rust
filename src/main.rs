fn add(num_one: i32, num_two: i32) -> i32 {
    num_one + num_two
}

fn main() {
    //   let foo:bool = false;
    let mut total = add(20, 10);
    let mut free_shipping = false;

    if total > 50 {
        println!("You quality for free shipping!");
        free_shipping = true;
    } else if total > 20 {
        println!("If you add more items, you can qualify for free shipping.")
    } else {
        println!("No free shipping.")
    }

    match free_shipping {
        true => total = total + 0,
        false => total = total + 5,
    }

    println!("Total: {:?}", total);
}
