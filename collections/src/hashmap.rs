use std::collections::HashMap;

pub fn init_hashmap() {
    let mut score = HashMap::new();
    score.insert(String::from("blue"), 10);
    score.insert(String::from("yellow"), 50);

    let team_name = String::from("blue");
    let team_score = score.get(&team_name).copied().unwrap_or(0);
    println!("{team_name}'s score = {team_score}");
}

pub fn iterate_hashmap() {
    let mut score = HashMap::new();
    score.insert(String::from("blue"), 10);
    score.insert(String::from("yellow"), 50);

    for (key, val) in &score {
            println!("{key}: {val}");
    }

    let color = String::from("orange");
    let val = 50;
    score.insert(color, val); // String var color, is moved into the mashmap. and no longer valid
    // println!("{color}"); can't use color, because its value is moved; ownership is transferred to the hashmap
    
    // IMPORTANT: 
    // if we insert references to values into the hash map, the values won’t be moved into the hash map. BUT..
    // references point to must be valid for at least as long as the hash map is valid.

    /**
     * using a hashmap
     */

    let mut accounts = HashMap::new();
    accounts.insert(String::from("account1"), 50);
    accounts.insert(String::from("account1"), 100); // 50 gets overriddeen because account1 key exists

    // check whether the key for the account5 has a value associated with it. If it doesn’t, we want to insert the value 10
    // same thing as m["account5"] = 10; if there doesn't exists account5, then insert it into the map with value 10
    // --- important: IF it DOES exist, leave it alone!
    accounts.entry(String::from("account5")).or_insert(10);

    /**
     * UPDATING hashmap values
     */
    let str_slice = "hello world wonderful world";
    let mut m = HashMap::new();
    for token in str_slice.split_whitespace() {
        // or insert gives back the mutable reference to token
        // similar to it = m[token] , then it->second += 1;
        let map_value_of_token = m.entry(token).or_insert(0);
        *map_value_of_token += 1;
    }
    println!("{m:?}");

}