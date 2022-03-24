fn main() {
    luhn_algorithm();
}
// write a function that validates a credit card number using luhn's algorithm
fn luhn_algorithm(){
    let mut card_number = String::new();
    println!("Enter a credit card number");
    std::io::stdin().read_line(&mut card_number).expect("Failed to read line");
    let card_number: u32 = card_number.trim().parse().expect("Please type a number!");
    let mut sum = 0;
    let mut digit;
    let mut double = false;
    for i in card_number.to_string().chars().rev() {
        digit = i.to_digit(10).unwrap();
        if double {
            digit *= 2;
            if digit > 9 {
                digit -= 9;
            }
        }
        sum += digit;
        double = !double;
    }
    if sum % 10 == 0 {
        println!("The number is valid!");
    } else {
        println!("The number is invalid!");
    }
}