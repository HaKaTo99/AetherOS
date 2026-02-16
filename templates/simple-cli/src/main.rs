fn main() {
    println!("Hello from AetherOS CLI Template!");
    
    // Example arg parsing
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        println!("Argument received: {}", args[1]);
    }
}
