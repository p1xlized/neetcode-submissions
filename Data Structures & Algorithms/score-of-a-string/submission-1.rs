impl Solution {
pub fn score_of_string(s: String) -> i32 {
    let chars: Vec<i32> = s.chars().map(|c| c as i32).collect();

    chars.windows(2).map(|w| (w[0] - w[1]).abs()).sum()
}

}
