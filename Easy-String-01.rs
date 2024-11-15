// 3110. Score of a String
impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut score = 0i32;
        let bytes = s.as_bytes();

        for i in 1..bytes.len(){
            score += (bytes[i] as i32 - bytes[i-1] as i32).abs();
        }

        score
    }
}

// 1119. Remove Vowels from a String
impl Solution {
    pub fn remove_vowels(s: String) -> String {
        let vowels: String = String::from("aeiouAEIOU");

        let result = s.chars()
        .filter(|&c| !vowels.contains(c))
        .collect();

        result
    }
}


// 2011. Final Value of Variable After Performing Operations
impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut result = 0i32;
        for operation in operations.iter(){
            match operation.as_str(){
                "--X" | "X--" => result -=1,
                "++X" | "X++" => result +=1,
                _ => ()
            }
        }

        result
    }
}



// 1108. Defanging an IP Address
impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        address.replace(".", "[.]")
    }
}

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let mut result = String::new();
        for c in address.chars(){
            if c == '.'{
                result.push_str("[.]");
            }else{
                result.push(c);
            }
        }
        result
    }
}


// 771. Jewels and Stones
impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut result = 0i32;

        for stone in stones.chars() {
            if jewels.contains(stone) {
                result += 1;
            }
        }

        result
    }
}
