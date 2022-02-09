pub fn encrypt_this(text: &str) -> String {
    let result: String = text.split(" ").collect::<Vec<&str>>().into_iter().map(|str_| {
        let first_char = str_.chars().nth(0).unwrap();
        if str_.len() == 1 {
            return format!("{} ", (first_char as u32).to_string());
        }
        let second_char = str_.chars().nth(1).unwrap();
        if str_.len() == 2 {
            return format!("{}{} ", first_char as u32, second_char);
        }
        let last_char = str_.chars().nth(str_.len()-1).unwrap();
        return format!("{}{}{}{} ", (first_char as u32).to_string(), last_char, &str_[2..str_.len()-1], second_char)
    }).collect();
    return result.trim().to_owned();
}

// fn encrypt_this(text: &str) -> String {
//     text.split_whitespace().map(|word| {
//         let chars = word.chars().collect::<Vec<char>>();
//         match word.len() {
//             1 => (chars[0] as u8).to_string(),
//             2 => format!("{}{}", chars[0] as u8, chars[1]),
//             3 => format!("{}{}{}", chars[0] as u8, chars[2], chars[1]),
//             _ => format!(
//                     "{}{}{}{}",
//                     chars[0] as u8,
//                     chars[chars.len() - 1],
//                     &chars[2..word.len() - 1].iter().collect::<String>(),
//                     chars[1],
//                  ),
//         }
//     }).collect::<Vec<String>>().join(" ")
// }

// You want to create secret messages which can be deciphered by the Decipher this! Here are the conditions:
// Your message is a string containing space separated words.
// You need to encrypt each word in the message using the following rules:
// The first letter must be converted to its ASCII code.
// The second letter must be switched with the last letter
// Keepin' it simple: There are no special characters in the input.
// Examples:
// encrypt_this("Hello") == "72olle"
// encrypt_this("good") == "103doo"
// encrypt_this("hello world") == "104olle 119drlo"
