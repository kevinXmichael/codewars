// mod test;
// mod create_phone_num;
// mod digital_root;
mod largest_five_digit_number;

fn main() {
    let result = largest_five_digit_number::largest_five_digit_number("1234567890");
    println!("The result is: {} === {}", result, 67890);
}
