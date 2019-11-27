extern crate rl;

fn main() {
    let mut scope = rl::scope::Scope::new();
    scope.put("n".to_string(), Box::new(rl::types::RLInt::from(27)));
    let value = scope.get("n".to_string()).expect("No such variable found");
    println!("Getting value: {}", value.display());
    scope.do_file("test/simple.rl".to_string());
}
