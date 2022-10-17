use pluralizer::pluralize;

fn main() {
    pluralizer::initialize();

    // It can convert to plural
    println!("{}", pluralize("House", 2, true)); // 2 Houses

    // But also can convert to singular
    println!("{}", pluralize("Houses", 1, true)); // 1 House

    // And keep singularization if needed
    println!("{}", pluralize("House", 1, false)); // House

    // Or keep pluralization
    println!("{}", pluralize("Houses", 2, false)); // Houses
}
