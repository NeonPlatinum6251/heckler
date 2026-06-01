fn main() {
    let hexcolour = std::env::args().nth(1).expect("no argument given");
    let hexcolour = hexcolour.trim_start_matches('#');
    if hexcolour.len() > 6 {
        println!("Hexcode must be 6 characters long!");
        std::process::exit(1);
    }
    let red = u8::from_str_radix(&hexcolour[0..2],16).expect("invalid red value");
    let green =u8::from_str_radix(&hexcolour[2..4],16).expect("invalid green value");
    let blue =u8::from_str_radix(&hexcolour[4..6],16).expect("invalid blue value");
    println!(
        "\x1b[48;2;{};{};{}m        \x1b[0m",
        red, green, blue
    );
}
