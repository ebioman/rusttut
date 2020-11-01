fn main() {
    let mystring  = String::from("Guten Morten");
    let mystring2 = mystring.clone();
    takes_ownership(mystring);
    let myint = 5;
    copy_ownership(myint);
    println!("{} {}",myint,mystring2);
    // through return values
    let s1 = get_owner();
    let s2 = String::from("und noch eines!");
    let s3 = take_owner_and_return(s2);
    println!("{} and {} ", s1,s3);
    // avoiding moving ownership
    let s4 = String::from("Stefanie");
    let (s5,slen) = calculate_length(s4);
    println!("The length of {} is {}", s5,slen);
}

fn calculate_length(k: String)->(String, usize){
    let length = k.len();
    (k,length)
}

fn get_owner() -> String {
   let x = String::from("Auf ein neues!");
   x
}

fn take_owner_and_return(n: String)->String{
    n
}

fn takes_ownership(n: String){
    println!("{}",n);
}

fn copy_ownership(n: i32){
    println!("{}",n);
}
