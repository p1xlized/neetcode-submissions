impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }

        let first = &strs[0];
        
        for (i, byte) in first.as_bytes().iter().enumerate() {
            for other in strs.iter().skip(1) {
                let other_bytes = other.as_bytes();
                if i >= other_bytes.len() || other_bytes[i] != *byte {
                    return first[..i].to_string();
                }
            }
        }

        first.to_string()
    }
}