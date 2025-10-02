// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/main.rs

// Use the public structs and implementations from our cdqn library crate.
use cdqn::cdu::Cdu;

fn main() {
    println!("Chronosa Core: Validating CDU constructor...");

    // 1. Define a sample payload.
    let payload1 = b"This is the first event.".to_vec();
    // 2. Create a new CDU using the constructor.
    let cdu1 = Cdu::new(payload1, "genesis", vec![]);

    println!("\nSuccessfully created the first CDU:");
    println!("{:#?}", cdu1);

    // 3. Create a second CDU that is caused by the first.
    let payload2 = b"This event was caused by the first.".to_vec();
    let cdu2 = Cdu::new(payload2, "response", vec![cdu1.name.clone()]);

    println!("\nSuccessfully created a second CDU, causally linked to the first:");
    println!("{:#?}", cdu2);

    println!("\nValidation complete. CDU constructor is operational.");
}
