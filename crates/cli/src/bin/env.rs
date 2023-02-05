use cli::utils::Environment;

fn main() {
    let v = Environment::Local.as_str();

    println!("Environment to str {}", v);
    println!("Environment to str {}", Environment::Production.as_str());
}
