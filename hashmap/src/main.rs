use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"),10);
    scores.insert(String::from("red"),20);

    let team_name = String::from("blue");
    let _my_score  = scores.get(&team_name);

    // overwrite value

    scores.insert(String::from("blue"), 25);

    // only insert if no value

    scores.entry(String::from("blue")).or_insert(50);
    scores.entry(String::from("violet")).or_insert(70);

    // iterate and print
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let teams = vec![String::from("blue"),String::from("yellow")];
    let init_scores = vec![10,50];

    // pretty much an abomination
    let mut _scores2: HashMap<_, _> = 
        teams.into_iter().zip(init_scores.into_iter()).collect();

    let field = String::from("orange");
    let score = String::from("blue");
    let mut map = HashMap::new();
    // insert moves actually the value
    map.insert(&field,score);
    println!("My field is {}",field);

    let team_name = String::from("blue");
    let _my_out = scores.get(&team_name);
    for (key,value) in &scores {
        println!("{} :  {}",key,value);
    }

    let field_name = String::from("Fav color");
    let field_value = String::from("Green");
    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    // if we do not reference a string above it will
    // be moved instead and the print below will fail
    println!(" field name {}", field_name);

    // update value in hash as counter
    let text = String::from("this sentence has twice the word has and this is it");
    let mut word_map = HashMap::new();
    for word in text.split_whitespace(){
        let count = word_map.entry(word).or_insert(0);
        *count +=1;
    }

    println!("{:?}",word_map);
}
