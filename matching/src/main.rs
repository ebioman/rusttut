enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

// matching return the last value of each indepdently of printing
// similar but more elegant to a if/else structure
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("I am a penny");
            1
        }
        Coin::Nickel => {
            println!("I am a Nickel");
            5
        }

        Coin::Dime => {
            println!("I am a Dime");
            10
        }
        Coin::Quarter(state) => {
            println!("I am a Quarter from {:?}", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alabama));
}
