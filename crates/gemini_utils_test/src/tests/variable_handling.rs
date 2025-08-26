use gemini_utils::gemini_eprintln;

#[test]
fn test_named_variables_various_types() {
    let name = "Rustacean";
    let age = 5;
    let pi = 3.14159;
    let is_active = true;
    gemini_eprintln!("User :name: (Age: :age:, Pi: :pi:, Active: :is_active:)",
        name = name,
        age = age,
        pi = pi,
        is_active = is_active
    );
}
