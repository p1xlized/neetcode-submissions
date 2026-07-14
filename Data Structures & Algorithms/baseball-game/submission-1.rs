impl Solution {
pub fn cal_points(operations: Vec<String>) -> i32 {
    let mut records: Vec<i32> = Vec::new();
    for i in 0..operations.len() {
        match operations[i].as_str() {
            "1" => records.push(1),
            "2" => records.push(2),
            "+" => {
                let last = records[records.len() - 1];
                let second_last = records[records.len() - 2];
                records.push(last + second_last);
            }
            "C" => {
                records.pop();
            }
            "5" => records.push(5),
            "D" => {
                let last_score = records[records.len() - 1];
                records.push(last_score * 2);
            }
            num_str => {
    if let Ok(num) = num_str.parse::<i32>() {
        records.push(num);
    }
}
        }
    }
    records.iter().sum()
}

}
