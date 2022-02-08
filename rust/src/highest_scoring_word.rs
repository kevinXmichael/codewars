pub fn high(input: &str) -> &str {
    let mut max_len = 0;
    let mut max_len_local = 0;
    let mut max_word_index = 0;
    for (index, word) in input.split(" ").enumerate() {
        for char_ in word.chars() {
            max_len_local += (char_ as u32) - 96;
        }
        if max_len_local > max_len {
            max_len = max_len_local;
            max_word_index = index;
        }
        max_len_local = 0;
    }
    let c: Vec<&str> = input.split(" ").collect();
    return c[max_word_index]
}

// Given a string of words, you need to find the highest scoring word.

// Each letter of a word scores points according to its position in the alphabet: a = 1, b = 2, c = 3 etc.

// You need to return the highest scoring word as a string.

// If two words score the same, return the word that appears earliest in the original string.

// All letters will be lowercase and all inputs will be valid.
