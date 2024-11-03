fn main() {
    // Given two strings s and t, return true if s is a subsequence of t, or false otherwise.

    // A subsequence of a string is a sequence of characters that can be obtained by deleting some (or none) of the characters from the original string, while maintaining the relative order of the remaining characters. For example, "ace" is a subsequence of "abcde" while "aec" is not.

    let res = is_subsequence("ace", "abcde");

    println!("{res}");
}

// Solution
fn is_subsequence(s: &str, t: &str) -> bool {
    let mut i = 0;
    let mut j = 0;

    while i < s.len() && j < t.len() {
        if s.as_bytes()[i] == t.as_bytes()[j] {
            i += 1;
        }

        // increment j no matter what
        j += 1;
    }

    return i == s.len();
}
