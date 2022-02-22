// mod test;
mod weight_for_weight;

fn main() {
    // test::run();

    let result = weight_for_weight::order_weight("103 123 4444 99 2000");
    println!("The result is: {} === {}?", result, "2000 103 123 4444 99");
}
