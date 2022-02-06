// mod test;
mod create_phone_num;

fn main() {
    // test::run();
    let res_phone_number = create_phone_num::create_phone_number(&[1,2,3,4,5,6,7,8,9,0]);
    println!("The result is: {}", res_phone_number);
}
