fn main() {
    let data = "Initial content";
    let mut s = data.to_string();
    s.push_str(" and more");
    s.push('!');
    let s2 = String::from(" And second phrase");
    let s3 = String::from(" Over and out");
    let s4 = format!("{} \n {} \n {}",s,s2,s3);
    println!("My string is {}",s4);
    let sequence = String::from("ATTTTGGA");
    let homopol  = &sequence[1..5];
    println!("The homopolymer of sequence {} is {}",sequence,homopol);
    for nuc in sequence.chars() {
        println!("{}",nuc);
    }
    for b in sequence.bytes() {
        println!("{}",b);
    }
}
