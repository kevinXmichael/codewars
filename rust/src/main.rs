// mod test;
mod highest_and_lowest;

fn main() {
    // test::run();

    let result = highest_and_lowest::high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4");
    println!("The result is: {} === {}?", result, "42 -9");
}
