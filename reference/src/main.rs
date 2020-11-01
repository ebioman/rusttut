fn main() {
    let s1 = String::from("Here we go again");
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1,len);
    let mut s3 = String::from("Go to hell");
    change(&mut s3);
    println!("{}",s3);
    { 
        let _r1 = &mut s3;
    }
    //let _r3 = &mut s4;
    //wont work as both can otherwise result in a race condition
    //only way is similar to _r1 with local scope
    let r3 = &s3;
    let r5 = &s3;
    println!("We referenced {} amd {} ", r3,r5);
    let r4 = &mut s3;
    println!("{}",r4);

    //create dangling reference
    let _stupid = dangle();
    
}

fn dangle()-> String{
    let s = String::from("coucou");
    s
}

fn change(x: &mut String){
    x.push_str(" Trump");
}

fn calculate_length(s: &String)-> usize{
    let l = s.len();
    l
}
