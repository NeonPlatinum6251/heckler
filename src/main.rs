use std::env::args;


fn main() {
    let symbolsallowed:[char; 16]  = ['1','2','3','4','5','6','7','8','9','0','A','B','C','D','E','F'];
    let hexcolour = std::env::args().nth(1).expect("no argument given");
    let length = hexcolour.chars().count();
    if length != 6 {
        println!("Expected 6 digits of hex got {}",length);
        let correctlength = true;
    }
    let is_valid = hexcolour.chars().all(|c| symbolsallowed.contains(&c));

    }
}
