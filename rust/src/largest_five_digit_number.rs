pub fn largest_five_digit_number(num: &str) -> u32 {
    let step = 5;
    let mut largest_num = 0;
    for n in 0..(num.len() - step + 1) {
        let slice = &num[n..n+step];
        let slice_num = slice.parse().unwrap();
        if slice_num > largest_num {
            largest_num = slice_num;
        }
    }
    return largest_num;
}
