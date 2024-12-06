// 1678. Goal Parser Interpretation
use std::collections::HashMap;

impl Solution {
    pub fn interpret(command: String) -> String {
        let mut interpretor: HashMap<&str, &str> = HashMap::new();
        interpretor.insert("()", "o");
        interpretor.insert("(al)", "al");
        interpretor.insert("G", "G");

        let mut result = String::new();
        let mut temp = String::new();

        //Stack 사용
        for c in command.chars(){
            temp.push(c);
            if let Some(&temp_result) = interpretor.get(temp.as_str()){
                result.push_str(temp_result);
                temp.clear();
            }
        }

        result
    }
}


// 2114. Maximum Number of Words Found in Sentences
use std::cmp::max;

impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut max_result = 0i32;

        for word in sentences.iter(){
            let ws: Vec<&str> = word.split(" ").collect();
            let each_len = ws.len();
            
            max_result = max(max_result, each_len as i32);
        }

        max_result
    }
}


// 1221. Split a String in Balanced Strings
impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut count = 0i32;

        let mut temp = 0i32;
        for c in s.chars(){
            if c == 'R'{
                temp += 1;
            }else{
                temp -=1;
            }
            if temp == 0{
                count += 1;
            }
        }

        count
    }
}

// 2000. Reverse Prefix of Word


// 1816. Truncate Sentence
impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let splited : Vec<&str> = s.split(" ").collect();

        let mut result_vec : Vec<&str> = Vec::new();
        for i in 0..k as usize{
            result_vec.push(splited[i]);
        }
        result_vec.join(" ")
    }
}

// 1662. Check If Two String Arrays are Equivalent
impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let joined_word1: String = word1.join("");
        let joined_word2: String = word2.join("");

        joined_word1 == joined_word2
    }
}

// 1528. Shuffle String
use std::collections::HashMap;

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut s_map : HashMap<i32, char> = HashMap::new();

        for (i, c) in s.chars().enumerate(){
            s_map.insert(indices[i], c);
        }

        let mut result = String::new();
        for i in 0..s.len()as i32{
            if let Some(&charchar) = s_map.get(&i){
                result.push(charchar);
            }
        }

        for i in 0..s.len(){
            if let Some(&charchar) = s_map.get(i as &i32){
                result.push(charchar);
            }
        }

        result
    }
}


// 2325. Decode the Message


// 2418. Sort the People
use std::collections::HashMap;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut name_map: HashMap<i32, String> = HashMap::new();
        for i in 0..names.len(){
            name_map.insert(heights[i], names[i].clone());
        }
        
        let mut sorted_heights: Vec<i32> = heights.clone();
        sorted_heights.sort_by(|a, b| b.cmp(a));

        let mut result: Vec<String> = Vec::new();

        for height in sorted_heights.iter(){
            if let Some(each) = name_map.get(height){
                result.push(each.clone());
            }
        }

        result
    }
}

// 1773. Count Items Matching a Rule
impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let mut result = 0i32;

        for each_vec in items.iter(){
            match rule_key.as_str(){
                "type" => if each_vec[0] == rule_value {result += 1},
                "color" => if each_vec[1] == rule_value {result += 1},
                "name" => if each_vec[2] == rule_value {result += 1},
                _ => (),
            }
        }

        result
    }
}
