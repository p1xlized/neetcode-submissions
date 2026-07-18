impl Solution {
pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::with_capacity(word1.len() + word2.len());
        
        let mut chars1 = word1.chars();
        let mut chars2 = word2.chars();
    
        loop {
            let char1 = chars1.next();
            let char2 = chars2.next();
            
            if char1.is_none() && char2.is_none() {
                break;
            }
            if let Some(c) = char1 { result.push(c); }
            if let Some(c) = char2 { result.push(c); }
        }
        
        result
    }
}
