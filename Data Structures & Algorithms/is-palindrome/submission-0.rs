impl Solution {
pub fn is_palindrome(s: String) -> bool {
    let chars: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let string: String = chars.iter().collect();
    let reversed: String = chars.iter().rev().collect();
    string == reversed
}
}
