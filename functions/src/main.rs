fn main() {
    annoying_boy(4,8);
    let mytuple = (1,2,3,4);
    let x = mytuple.3;
    println!("My number is {} ",x);
    let k = 1;
    let n = getplus10(k);
    println!("We got {} ", n);
}

fn annoying_boy(x: i32, y: i32) {
    println!("the annoying monsters are {} and {} years old",x,y);
}

fn getplus10(k : i32)-> i32 {
    k + 10

}
