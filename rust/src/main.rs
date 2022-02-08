// mod test;
// mod create_phone_num;
// mod digital_root;
// mod largest_five_digit_number;
// mod get_middle;
// mod mexican_wave;
// mod odd_or_even;
mod buying_a_car;

fn main() {
    let mut result = buying_a_car::nb_months(2000, 8000, 1000, 1.5);
    println!("The result is: {:?} == {:?}", result, (6, 766));
    
    result = buying_a_car::nb_months(18000, 32000, 1500, 1.25);
    println!("The result is: {:?} == {:?}", result, (8, 332));
    
    result = buying_a_car::nb_months(7500, 32000, 300, 1.55);
    println!("The result is: {:?} == {:?}", result, (25, 122));

    // test::run();
}
