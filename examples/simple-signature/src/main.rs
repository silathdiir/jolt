use std::time::Instant;

pub fn main() {
    let (prove_sig, verify_sig) = guest::build_sig();
    let program_summary = guest::analyze_sig();

    let start = Instant::now();
    let (output, proof) = prove_sig();
    let duration = start.elapsed();
    println!("Proving time is: {} ms", duration.as_millis());

    let is_valid = verify_sig(proof);

    println!("valid: {}", is_valid);
}
