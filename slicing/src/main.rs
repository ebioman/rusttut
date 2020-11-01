fn main() {
    let new_string = String::from("Emanuel is tired");
    let word = first_word(&new_string[..]);
    println!("the first word of '{}' is {}", new_string, word);
    let first3 = &new_string[0..3];
    println!("My first 3 letters are {}",first3);
    let my_string_literal = "hello world";
    let word  =  first_word(&my_string_literal[..]);
    println!("word {}",word);
    let word2 = first_word(my_string_literal);
    println!("word2 {}",word2);
} 

fn first_word(w: &str) -> &str {
    let bytes = w.as_bytes();
    for(i,&item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &w[0..i];
        }
    }
    &w[..]
}