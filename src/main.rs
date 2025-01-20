fn main() {
    println!(
        "Running on {}: {}",
        std::env::consts::OS,
        std::env::consts::ARCH
    );
    // The main driver progrem is incomplete
    // use: cargo test --test install_test -- --ignored --show-output
    // to run the install test that currenlty drives the installer

    // my plan for this is to use the same crate as what ragenix uses to implement its cli
}
