pub fn create_phone_number(numbers: &[u8]) -> String {
    let s: String = numbers.into_iter().map(|i| i.to_string()).collect();
    format!("({}) {}-{}", &s[..3], &s[3..6], &s[6..])
}

// Write a function that accepts an array of 10 integers (between 0 and 9), that returns a string of those numbers in the form of a phone number.

// Example
// create_phone_number(&[1,2,3,4,5,6,7,8,9,0]); // returns "(123) 456-7890"
// The returned format must be correct in order to complete this challenge.
// Don't forget the space after the closing parentheses!

// fn slice_to_str(slice: &[u8]) -> String {
//     let mut result = String::new();
//     for num in slice {
//         result += &num.to_string();
//     }
//     return result;
// }

// pub fn create_phone_number(numbers: &[u8]) -> String {
//     let mut result = String::new();
//     result.push_str(&(format!("({})", slice_to_str(&numbers[0..3]))));
//     result.push_str(&(format!(" {}", slice_to_str(&numbers[3..6]))));
//     result.push_str(&(format!("-{}", slice_to_str(&numbers[6..10]))));
//     return result;
// }
