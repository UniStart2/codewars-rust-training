fn reverse_words(str: &str) -> String {
    str.split(' ')// split word
        .map(|word| word.chars().rev().collect::<String>())// reverse each word
        .collect::<Vec<String>>()
        .join(" ")
}

// Rust tests
#[test]
fn sample_test() {
    assert_eq!(reverse_words("The quick brown fox jumps over the lazy dog."), "ehT kciuq nworb xof spmuj revo eht yzal .god");
    assert_eq!(reverse_words("apple"), "elppa");
    assert_eq!(reverse_words("a b c d"),"a b c d");
    assert_eq!(reverse_words("double  spaced  words"), "elbuod  decaps  sdrow");
}