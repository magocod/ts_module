use add_one::day2a::luhn;

fn main() {
    let cc_number = "1234 5678 1234 5670";
    println!(
        "Is {} a valid credit card number? {}",
        cc_number,
        if luhn(cc_number) { "yes" } else { "no" }
    );

    let cc_number = "1234 bvc";
    println!(
        "Is {} a valid credit card number? {}",
        cc_number,
        if luhn(cc_number) { "yes" } else { "no" }
    );
}
