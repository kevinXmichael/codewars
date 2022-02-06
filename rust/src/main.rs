// mod test;
mod create_phone_num;
mod digital_root;

fn main() {
    let result = digital_root::digital_root(321);
    println!("The result is: {}", result);
}
