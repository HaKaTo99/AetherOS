use aether_sdk::{scheduler, net, quantum};

fn main() {
    println!("Starting Distributed Service Node...");
    
    // 1. Initialize local mesh node
    let node = net::init_mesh_node();
    println!("Node ID: {}", node.id);

    // 2. Spawn a listener for quantum messages
    scheduler::spawn(move || {
        loop {
            if let Some(msg) = node.receive() {
                let decrypted = quantum::decrypt(&msg);
                println!("Received secure message: {}", decrypted);
            }
        }
    });

    // 3. Keep main thread alive
    loop {
        scheduler::yield_now();
    }
}
