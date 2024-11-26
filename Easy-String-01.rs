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


// 2942. Find Words Containing Character
impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        let mut result : Vec<i32> = Vec::new();

        for i in 0..words.len(){
            if words[i].contains(x){
                result.push(i as i32);
            }
        }

        result
    }
}


// 3280. Convert Date to Binary
impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        let splited : Vec<&str> = date.split("-").collect();

        let result = splited.iter()
        .map(|each| {
            let number = each.parse::<u32>().expect("Invalid");
            format!("{:b}", number)
        })
        .collect::<Vec<String>>()
        .join("-");

        result
    }
}


// 1684. Count the Number of Consistent Strings
use std::collections::HashSet;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let allowed_set: HashSet<char> = allowed.chars().collect();

        words.iter()
        .filter(|word| word.chars().all(|ch| allowed_set.contains(&ch)))
        .count() as i32
    }
}


// 3146. Permutation Difference between Two Strings
use std::collections::HashMap;

impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut result = 0i32;

        let mut t_index_map = HashMap::new();
        for (i, ch) in t.chars().enumerate() {
            t_index_map.insert(ch, i as i32);
        }

        for (i, ch) in s.chars().enumerate() {
            if let Some(&t_index) = t_index_map.get(&ch) {
                result += (i as i32 - t_index).abs();
            } else {
                panic!("Character '{}' not found in string t", ch);
            }
        }
        result
    }
}

// 1165. Single-Row Keyboard
use std::collections::HashMap;

impl Solution {
    pub fn calculate_time(keyboard: String, word: String) -> i32 {
        let mut i_keyboard: HashMap<char, usize> = HashMap::new();

        for (i, c) in keyboard.chars().enumerate(){
            i_keyboard.insert(c, i);
        }

        let mut prev = 0;
        let mut result = 0i32;

        for c in word.chars(){
            if let Some(&hey) = i_keyboard.get(&c){
                result += (hey as i32 - prev as i32).abs();
                prev = hey;
            }
        }

        result
    }
}
